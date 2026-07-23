pub mod aws_api;
pub mod deploy;
pub mod plan;
pub mod preflight;
pub mod sdk;
pub mod ui;

use crate::api::client::PlerionClient;
use crate::api::endpoints::aws as aws_endpoints;
use crate::api::models::aws::{CfnTemplateResponse, ExternalIdResponse, TokenResponse};
use crate::error::PlerionError;
use aws_api::{AwsApi, CreateStackRequest};
use thiserror::Error;
use ui::Ui;

pub const PLERION_ACCOUNT_ID_DEFAULT: &str = "588158338731";
pub const DEFAULT_STACK_NAME: &str = "Plerion-Integration";

/// Actions simulated in preflight — mirrors the verified minimum deployer
/// policy's deploy statement.
pub const REQUIRED_ACTIONS: &[&str] = &[
    "cloudformation:GetTemplateSummary",
    "cloudformation:CreateStack",
    "cloudformation:DescribeStacks",
    "cloudformation:DescribeStackEvents",
    "iam:CreateRole",
    "iam:GetRole",
    "iam:TagRole",
    "iam:PutRolePolicy",
    "iam:AttachRolePolicy",
    "iam:PassRole",
    "iam:CreatePolicy",
    "iam:GetPolicy",
    "iam:GetRolePolicy",
    "iam:ListPolicyVersions",
    "iam:ListRoles",
    "lambda:CreateFunction",
    "lambda:GetFunction",
    "lambda:InvokeFunction",
    "lambda:TagResource",
];

#[derive(Debug, Clone)]
pub struct OnboardOptions {
    pub stack_name: String,
    pub auto_update: bool,
    pub kms_key_access_mode: String,
    pub service_account_id: Option<String>,
    pub expect_account_id: Option<String>,
    pub allow_existing: bool,
    pub strict_preflight: bool,
    pub validate_only: bool,
    pub yes: bool,
    pub wait_timeout_secs: u64,
    pub poll_interval_secs: u64,
    pub plerion_account_id: String,
    /// Host the stack reports back to, e.g. `au.api.plerion.com`.
    pub plerion_url: String,
    /// Tenant label shown in the confirmation (region or endpoint host).
    pub tenant_label: String,
}

impl Default for OnboardOptions {
    fn default() -> Self {
        Self {
            stack_name: DEFAULT_STACK_NAME.to_string(),
            auto_update: true,
            kms_key_access_mode: "ALL_KEYS".to_string(),
            service_account_id: None,
            expect_account_id: None,
            allow_existing: false,
            strict_preflight: false,
            validate_only: false,
            yes: false,
            wait_timeout_secs: 1800,
            poll_interval_secs: 15,
            plerion_account_id: PLERION_ACCOUNT_ID_DEFAULT.to_string(),
            plerion_url: "au.api.plerion.com".to_string(),
            tenant_label: "au".to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum OnboardError {
    #[error("{0}")]
    InvalidOptions(String),

    #[error("Plerion API error: {0}")]
    Plerion(#[from] PlerionError),

    #[error("Unexpected Plerion API response: {0}")]
    UnexpectedResponse(String),

    #[error("AWS error: {0}")]
    Aws(String),

    #[error("Preflight failed: {0}")]
    PreflightFailed(String),

    #[error("Aborted: {0}")]
    Aborted(String),

    #[error("Account already onboarded: {0}")]
    AlreadyOnboarded(String),

    #[error("Deployment failed: {0}")]
    DeployFailed(String),
}

impl OnboardError {
    /// Exit-code contract for `integrations add aws` (documented in the
    /// command help): 1 config/API, 2 preflight/aborted, 3 deploy,
    /// 4 already onboarded.
    pub fn exit_code(&self) -> i32 {
        match self {
            OnboardError::InvalidOptions(_)
            | OnboardError::Plerion(_)
            | OnboardError::UnexpectedResponse(_) => 1,
            OnboardError::Aws(_)
            | OnboardError::PreflightFailed(_)
            | OnboardError::Aborted(_) => 2,
            OnboardError::DeployFailed(_) => 3,
            OnboardError::AlreadyOnboarded(_) => 4,
        }
    }
}

#[derive(Debug)]
pub enum OnboardOutcome {
    /// Stack created; payload is the machine-readable result for stdout.
    Completed(serde_json::Value),
    /// --validate-only: the preflight report for stdout.
    Validated(serde_json::Value),
}

fn step(n: u8, total: u8, msg: &str) {
    eprintln!("[{n}/{total}] {msg}");
}

fn validate_options(opts: &OnboardOptions) -> Result<(), OnboardError> {
    if opts.auto_update && !opts.stack_name.starts_with("Plerion-") {
        return Err(OnboardError::InvalidOptions(format!(
            "stack name '{}' must start with 'Plerion-' while auto-update is enabled \
             (the auto-update role is IAM-scoped to stacks named Plerion*); \
             use --stack-name Plerion-<name> or --no-auto-update",
            opts.stack_name
        )));
    }
    for (flag, value) in [
        ("--expect-account-id", &opts.expect_account_id),
        ("--service-account-id", &opts.service_account_id),
    ] {
        if let Some(v) = value {
            if v.len() != 12 || !v.chars().all(|c| c.is_ascii_digit()) {
                return Err(OnboardError::InvalidOptions(format!(
                    "{flag} must be a 12-digit AWS account ID (got '{v}')"
                )));
            }
        }
    }
    if !["ALL_KEYS", "SELECTED_KEYS"].contains(&opts.kms_key_access_mode.as_str()) {
        return Err(OnboardError::InvalidOptions(
            "--kms-key-access-mode must be ALL_KEYS or SELECTED_KEYS".to_string(),
        ));
    }
    Ok(())
}

/// End-to-end onboarding of one AWS account. See SPEC-plerion-cli-onboard.md.
pub async fn run(
    plerion: &PlerionClient,
    aws: &dyn AwsApi,
    ui: &mut dyn Ui,
    opts: &OnboardOptions,
) -> Result<OnboardOutcome, OnboardError> {
    validate_options(opts)?;
    let total: u8 = 6;

    // [1/6] Tenant external ID (also serves as the API-key sanity check).
    step(1, total, "Fetching tenant external ID");
    let external_id: ExternalIdResponse =
        from_value(aws_endpoints::get_external_id(plerion).await?)?;
    let external_id = external_id
        .data
        .and_then(|d| d.external_id)
        .ok_or_else(|| OnboardError::UnexpectedResponse("missing data.externalId".to_string()))?;

    // [2/6] Pinned template + service account resolution.
    step(2, total, "Resolving CloudFormation template and service account");
    let template: CfnTemplateResponse = from_value(
        aws_endpoints::get_cloudformation_template(plerion, "AWSAccount").await?,
    )?;
    let template = template
        .data
        .ok_or_else(|| OnboardError::UnexpectedResponse("missing data".to_string()))?;
    let template_url = template.template_url.clone().ok_or_else(|| {
        OnboardError::UnexpectedResponse("missing data.templateURL".to_string())
    })?;
    let template_version = template
        .template_version
        .clone()
        .unwrap_or_else(|| "unknown".to_string());
    let service_account_id = opts
        .service_account_id
        .clone()
        .or(template.service_account_id.clone())
        .ok_or_else(|| {
            OnboardError::InvalidOptions(
                "the Plerion-managed CWPP service account ID could not be resolved \
                 automatically; pass --service-account-id <12-digit-id> (from your \
                 Plerion contact, or the console's Launch Stack link)"
                    .to_string(),
            )
        })?;

    // [3/6] Preflight.
    step(3, total, "Running preflight checks");
    let report = preflight::run(plerion, aws, opts, &template_url, &template_version).await?;

    if opts.validate_only {
        let json = serde_json::to_value(&report)
            .map_err(|e| OnboardError::UnexpectedResponse(e.to_string()))?;
        eprintln!("Preflight passed — validation-only mode, nothing deployed.");
        return Ok(OnboardOutcome::Validated(json));
    }

    // [4/6] Consolidated confirmation, then the short-lived registration token
    // (minted last so its validity window is maximal).
    step(4, total, "Confirming deployment");
    let alias = report
        .account_alias
        .clone()
        .map(|a| format!(" (alias: {a})"))
        .unwrap_or_default();
    let auto_update_label = if opts.auto_update {
        "enabled (creates a guard-railed update role)"
    } else {
        "disabled"
    };
    let prompt = format!(
        "Onboard AWS account {}{} into Plerion tenant region {}?\n\
         \x20 Stack           {} (template {})\n\
         \x20 Capabilities    CSPM + CIEM + CWPP (Plerion-managed scanning, service acct {})\n\
         \x20 KMS access      {}\n\
         \x20 Auto-update     {}\n\
         Proceed?",
        report.account_id,
        alias,
        opts.tenant_label,
        opts.stack_name,
        template_version,
        service_account_id,
        opts.kms_key_access_mode,
        auto_update_label,
    );
    if !opts.yes {
        if !ui.is_interactive() {
            return Err(OnboardError::Aborted(
                "refusing to deploy without confirmation in a non-interactive session; \
                 re-run with --yes (optionally with --expect-account-id as a guard)"
                    .to_string(),
            ));
        }
        if !ui.confirm(&prompt) {
            return Err(OnboardError::Aborted("declined at confirmation".to_string()));
        }
    }

    let token: TokenResponse = from_value(aws_endpoints::generate_token(plerion, None).await?)?;
    let auth_token = token
        .data
        .and_then(|d| d.token)
        .ok_or_else(|| OnboardError::UnexpectedResponse("missing data.token".to_string()))?;

    // [5/6] Create the stack.
    step(5, total, &format!("Creating stack '{}'", opts.stack_name));
    let request = CreateStackRequest {
        stack_name: opts.stack_name.clone(),
        template_url: template_url.clone(),
        parameters: vec![
            ("PlerionURL".to_string(), opts.plerion_url.clone()),
            ("PlerionAccountId".to_string(), opts.plerion_account_id.clone()),
            ("ExternalId".to_string(), external_id),
            ("AuthToken".to_string(), auth_token),
            ("Capabilities".to_string(), "ALL".to_string()),
            (
                "WorkloadScanningType".to_string(),
                "PlerionManagedServiceAccount".to_string(),
            ),
            ("ServiceAccountId".to_string(), service_account_id),
            ("KMSKeyAccessMode".to_string(), opts.kms_key_access_mode.clone()),
            (
                "EnableAutoUpdate".to_string(),
                if opts.auto_update { "true" } else { "false" }.to_string(),
            ),
        ],
    };
    let stack_id = aws.create_stack(&request).await?;

    // [6/6] Wait for completion.
    step(6, total, "Waiting for stack to complete (typically 3-5 minutes)");
    let status = deploy::wait_for_stack(aws, &stack_id, opts).await?;

    let outputs = aws.stack_outputs(&stack_id).await.unwrap_or_default();
    let mut outputs_map = serde_json::Map::new();
    for o in &outputs {
        outputs_map.insert(o.key.clone(), serde_json::Value::String(o.value.clone()));
    }
    let result = serde_json::json!({
        "accountId": report.account_id,
        "stackName": opts.stack_name,
        "stackId": stack_id,
        "status": status,
        "templateVersion": template_version,
        "outputs": outputs_map,
    });
    eprintln!(
        "✓ Onboarded {} — findings appear in Plerion within ~10 minutes \
         (Settings > Integrations > Scans).",
        report.account_id
    );
    Ok(OnboardOutcome::Completed(result))
}

fn from_value<T: serde::de::DeserializeOwned>(v: serde_json::Value) -> Result<T, OnboardError> {
    serde_json::from_value(v).map_err(|e| OnboardError::UnexpectedResponse(e.to_string()))
}
