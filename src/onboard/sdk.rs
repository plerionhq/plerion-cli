use crate::onboard::aws_api::*;
use crate::onboard::OnboardError;
use aws_config::BehaviorVersion;

/// Production `AwsApi` backed by the AWS SDK. The only file in the crate that
/// imports aws-sdk-* types.
pub struct SdkAws {
    sts: aws_sdk_sts::Client,
    iam: aws_sdk_iam::Client,
    cfn: aws_sdk_cloudformation::Client,
}

impl SdkAws {
    pub async fn new(profile: Option<&str>, region: Option<&str>) -> Self {
        let mut loader = aws_config::defaults(BehaviorVersion::latest());
        if let Some(p) = profile {
            loader = loader.profile_name(p);
        }
        if let Some(r) = region {
            loader = loader.region(aws_config::Region::new(r.to_string()));
        }
        let sdk_config = loader.load().await;
        Self {
            sts: aws_sdk_sts::Client::new(&sdk_config),
            iam: aws_sdk_iam::Client::new(&sdk_config),
            cfn: aws_sdk_cloudformation::Client::new(&sdk_config),
        }
    }
}

fn aws_err<E>(context: &str, e: E) -> OnboardError
where
    E: std::error::Error + Send + Sync + 'static,
{
    // Walk the source chain so service-level messages (AccessDenied etc.)
    // surface instead of the SDK's generic "service error".
    let mut msg = e.to_string();
    let mut source = std::error::Error::source(&e);
    while let Some(s) = source {
        msg = format!("{msg}: {s}");
        source = s.source();
    }
    OnboardError::Aws(format!("{context}: {msg}"))
}

#[async_trait::async_trait]
impl AwsApi for SdkAws {
    async fn caller_identity(&self) -> Result<CallerIdentity, OnboardError> {
        let resp = self
            .sts
            .get_caller_identity()
            .send()
            .await
            .map_err(|e| aws_err("sts:GetCallerIdentity", e))?;
        Ok(CallerIdentity {
            account: resp.account().unwrap_or_default().to_string(),
            arn: resp.arn().unwrap_or_default().to_string(),
        })
    }

    async fn account_alias(&self) -> Result<Option<String>, OnboardError> {
        // Best-effort: iam:ListAccountAliases is not in the minimum policy.
        Ok(self
            .iam
            .list_account_aliases()
            .send()
            .await
            .ok()
            .and_then(|r| r.account_aliases().first().cloned()))
    }

    async fn find_stack(&self, name: &str) -> Result<Option<String>, OnboardError> {
        match self.cfn.describe_stacks().stack_name(name).send().await {
            Ok(resp) => Ok(resp
                .stacks()
                .first()
                .and_then(|s| s.stack_status())
                .map(|s| s.as_str().to_string())),
            Err(e) => {
                let msg = format!("{e:?}");
                if msg.contains("does not exist") {
                    Ok(None)
                } else {
                    Err(aws_err("cloudformation:DescribeStacks", e))
                }
            }
        }
    }

    async fn list_roles_matching(&self, needle: &str) -> Result<Vec<String>, OnboardError> {
        let mut matches = Vec::new();
        let mut marker: Option<String> = None;
        loop {
            let resp = self
                .iam
                .list_roles()
                .set_marker(marker.clone())
                .send()
                .await
                .map_err(|e| aws_err("iam:ListRoles", e))?;
            matches.extend(
                resp.roles()
                    .iter()
                    .filter(|r| r.role_name().contains(needle))
                    .map(|r| r.role_name().to_string()),
            );
            if resp.is_truncated() {
                marker = resp.marker().map(String::from);
            } else {
                break;
            }
        }
        Ok(matches)
    }

    async fn role_arn(&self, role_name: &str) -> Result<String, OnboardError> {
        let resp = self
            .iam
            .get_role()
            .role_name(role_name)
            .send()
            .await
            .map_err(|e| aws_err("iam:GetRole", e))?;
        resp.role()
            .map(|r| r.arn().to_string())
            .ok_or_else(|| OnboardError::Aws("iam:GetRole returned no role".to_string()))
    }

    async fn simulate(
        &self,
        policy_source_arn: &str,
        actions: &[String],
    ) -> Result<Vec<SimResult>, OnboardError> {
        let resp = self
            .iam
            .simulate_principal_policy()
            .policy_source_arn(policy_source_arn)
            .set_action_names(Some(actions.to_vec()))
            .send()
            .await
            .map_err(|e| aws_err("iam:SimulatePrincipalPolicy", e))?;
        Ok(resp
            .evaluation_results()
            .iter()
            .map(|r| SimResult {
                action: r.eval_action_name().to_string(),
                allowed: r.eval_decision().as_str() == "allowed",
            })
            .collect())
    }

    async fn template_summary(&self, template_url: &str) -> Result<TemplateSummary, OnboardError> {
        let resp = self
            .cfn
            .get_template_summary()
            .template_url(template_url)
            .send()
            .await
            .map_err(|e| aws_err("cloudformation:GetTemplateSummary", e))?;
        Ok(TemplateSummary {
            description: resp.description().map(String::from),
        })
    }

    async fn create_stack(&self, req: &CreateStackRequest) -> Result<String, OnboardError> {
        use aws_sdk_cloudformation::types::{Capability, OnFailure, Parameter};
        let params = req
            .parameters
            .iter()
            .map(|(k, v)| {
                Parameter::builder()
                    .parameter_key(k)
                    .parameter_value(v)
                    .build()
            })
            .collect::<Vec<_>>();
        let resp = self
            .cfn
            .create_stack()
            .stack_name(&req.stack_name)
            .template_url(&req.template_url)
            .set_parameters(Some(params))
            .capabilities(Capability::CapabilityNamedIam)
            .on_failure(OnFailure::Rollback)
            .send()
            .await
            .map_err(|e| aws_err("cloudformation:CreateStack", e))?;
        resp.stack_id()
            .map(String::from)
            .ok_or_else(|| OnboardError::Aws("CreateStack returned no StackId".to_string()))
    }

    async fn stack_failure_events(&self, name: &str) -> Result<Vec<StackEvent>, OnboardError> {
        let resp = self
            .cfn
            .describe_stack_events()
            .stack_name(name)
            .send()
            .await
            .map_err(|e| aws_err("cloudformation:DescribeStackEvents", e))?;
        Ok(resp
            .stack_events()
            .iter()
            .filter(|e| {
                e.resource_status()
                    .map(|s| s.as_str().contains("FAILED"))
                    .unwrap_or(false)
            })
            .map(|e| StackEvent {
                logical_id: e.logical_resource_id().unwrap_or_default().to_string(),
                status: e
                    .resource_status()
                    .map(|s| s.as_str().to_string())
                    .unwrap_or_default(),
                reason: e.resource_status_reason().map(String::from),
            })
            .collect())
    }

    async fn stack_outputs(&self, name: &str) -> Result<Vec<StackOutput>, OnboardError> {
        let resp = self
            .cfn
            .describe_stacks()
            .stack_name(name)
            .send()
            .await
            .map_err(|e| aws_err("cloudformation:DescribeStacks", e))?;
        Ok(resp
            .stacks()
            .first()
            .map(|s| s.outputs())
            .unwrap_or_default()
            .iter()
            .map(|o| StackOutput {
                key: o.output_key().unwrap_or_default().to_string(),
                value: o.output_value().unwrap_or_default().to_string(),
            })
            .collect())
    }
}
