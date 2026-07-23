use crate::api::client::PlerionClient;
use crate::api::endpoints::integrations::list_integrations;
use crate::onboard::aws_api::{AwsApi, SimResult};
use crate::onboard::{OnboardError, OnboardOptions, REQUIRED_ACTIONS};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreflightReport {
    pub account_id: String,
    pub caller_arn: String,
    pub account_alias: Option<String>,
    pub template_version: String,
    pub template_description: Option<String>,
    /// Plerion IAM roles found in the account (any stack name).
    pub existing_plerion_roles: Vec<String>,
    /// True when the tenant already lists an integration for this account.
    pub existing_integration: bool,
    /// Actions the IAM simulator reported as denied. Advisory: the simulator
    /// has known false negatives with AWS Identity Center (SSO) principals.
    pub simulation_denied: Option<Vec<String>>,
}

/// Steps 3a-3e. Returns a report on success; fails with typed errors that map
/// to the command's exit codes.
pub async fn run(
    plerion: &PlerionClient,
    aws: &dyn AwsApi,
    opts: &OnboardOptions,
    template_url: &str,
    template_version: &str,
) -> Result<PreflightReport, OnboardError> {
    // 3a. Who are we deploying as, and is it the intended account?
    let caller = aws.caller_identity().await?;
    if let Some(expected) = &opts.expect_account_id {
        if expected != &caller.account {
            return Err(OnboardError::PreflightFailed(format!(
                "credentials belong to account {}, but --expect-account-id is {} — wrong AWS profile?",
                caller.account, expected
            )));
        }
    }
    let account_alias = aws.account_alias().await.unwrap_or(None);

    // 3b. Same-name stack conflict.
    if let Some(status) = aws.find_stack(&opts.stack_name).await? {
        if status == "ROLLBACK_COMPLETE" {
            return Err(OnboardError::PreflightFailed(format!(
                "stack '{}' exists in ROLLBACK_COMPLETE (a previous failed create); delete it \
                 first: aws cloudformation delete-stack --stack-name {}",
                opts.stack_name, opts.stack_name
            )));
        }
        return Err(OnboardError::PreflightFailed(format!(
            "stack '{}' already exists (status: {})",
            opts.stack_name, status
        )));
    }

    // 3c. Already onboarded? Check both sides: IAM roles in the account (any
    // stack name — console stacks use different names) and the tenant's own
    // integration list.
    let existing_roles = aws.list_roles_matching("PlerionAccessRole").await?;
    let existing_integration = tenant_has_integration(plerion, &caller.account).await?;
    if !existing_roles.is_empty() || existing_integration {
        let mut what = Vec::new();
        if !existing_roles.is_empty() {
            what.push(format!("Plerion IAM role(s): {}", existing_roles.join(", ")));
        }
        if existing_integration {
            what.push("an existing integration for this account in the tenant".to_string());
        }
        let detail = what.join(" and ");
        if !opts.allow_existing {
            return Err(OnboardError::AlreadyOnboarded(format!(
                "found {detail}. Deploying again creates a second integration (sometimes \
                 intentional, e.g. a different tenant). Re-run with --allow-existing to proceed."
            )));
        }
        eprintln!("      ! proceeding despite {detail} (--allow-existing)");
    }

    // 3d. Advisory permission simulation. False denials are a known issue for
    // Identity Center (SSO) principals, so denials warn rather than fail
    // unless --strict-preflight.
    let simulation_denied = match simulate_deployer(aws, &caller.arn, &caller.account).await {
        Ok(results) => {
            let denied: Vec<String> = results
                .iter()
                .filter(|r| !r.allowed)
                .map(|r| r.action.clone())
                .collect();
            if !denied.is_empty() {
                if opts.strict_preflight {
                    return Err(OnboardError::PreflightFailed(format!(
                        "IAM simulation reports denied actions ({}) and --strict-preflight is set",
                        denied.join(", ")
                    )));
                }
                eprintln!(
                    "      ! IAM simulation reports {} action(s) as denied: {}",
                    denied.len(),
                    denied.join(", ")
                );
                eprintln!(
                    "      ! this is often a FALSE NEGATIVE (known with AWS Identity Center / SSO \
                     roles); continuing — a real permission gap fails cleanly at deploy with a \
                     rollback. Use --strict-preflight to make this fatal."
                );
            }
            Some(denied)
        }
        Err(_) => {
            eprintln!(
                "      ! could not run iam:SimulatePrincipalPolicy — skipping the permission \
                 check (deployment may still fail if the deployer lacks the minimum policy)"
            );
            None
        }
    };

    // 3e. Template validation (GetTemplateSummary is covered by the minimum
    // deployer policy, unlike ValidateTemplate).
    let summary = aws.template_summary(template_url).await?;

    Ok(PreflightReport {
        account_id: caller.account,
        caller_arn: caller.arn,
        account_alias,
        template_version: template_version.to_string(),
        template_description: summary.description,
        existing_plerion_roles: existing_roles,
        existing_integration,
        simulation_denied,
    })
}

async fn tenant_has_integration(
    plerion: &PlerionClient,
    account_id: &str,
) -> Result<bool, OnboardError> {
    let mut cursor: Option<String> = None;
    loop {
        let resp = list_integrations(plerion, Some(1000), cursor.as_deref(), false).await?;
        if resp
            .data
            .iter()
            .any(|i| i.aws_account_id.as_deref() == Some(account_id))
        {
            return Ok(true);
        }
        if !resp.meta.has_next_page.unwrap_or(false) {
            return Ok(false);
        }
        cursor = resp.meta.cursor;
    }
}

async fn simulate_deployer(
    aws: &dyn AwsApi,
    caller_arn: &str,
    account: &str,
) -> Result<Vec<SimResult>, OnboardError> {
    // simulate-principal-policy needs an IAM user/role ARN, not the STS
    // assumed-role ARN. Resolve via GetRole so path'd roles (Identity Center)
    // get their real ARN.
    let source_arn = if caller_arn.contains(":assumed-role/") {
        let role_name = caller_arn
            .split('/')
            .nth(1)
            .ok_or_else(|| OnboardError::Aws("unparseable assumed-role ARN".to_string()))?;
        aws.role_arn(role_name)
            .await
            .unwrap_or_else(|_| format!("arn:aws:iam::{account}:role/{role_name}"))
    } else {
        caller_arn.to_string()
    };
    let actions: Vec<String> = REQUIRED_ACTIONS.iter().map(|s| s.to_string()).collect();
    aws.simulate(&source_arn, &actions).await
}
