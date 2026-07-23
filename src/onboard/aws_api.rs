use crate::onboard::OnboardError;
use serde::Serialize;

/// Owned domain types + a trait over the AWS calls the onboarding flow makes.
/// `sdk.rs` is the only production implementation; tests supply a scripted
/// mock so the orchestration state machine is unit-testable without AWS.

#[derive(Debug, Clone)]
pub struct CallerIdentity {
    pub account: String,
    pub arn: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimResult {
    pub action: String,
    pub allowed: bool,
}

#[derive(Debug, Clone)]
pub struct TemplateSummary {
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateStackRequest {
    pub stack_name: String,
    pub template_url: String,
    /// (ParameterKey, ParameterValue) pairs. AuthToken is NoEcho in the
    /// template and must never be rendered by callers.
    pub parameters: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackEvent {
    pub logical_id: String,
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackOutput {
    pub key: String,
    pub value: String,
}

#[async_trait::async_trait]
pub trait AwsApi: Send + Sync {
    async fn caller_identity(&self) -> Result<CallerIdentity, OnboardError>;
    /// Best-effort; implementations return Ok(None) when the alias cannot be read.
    async fn account_alias(&self) -> Result<Option<String>, OnboardError>;
    /// Returns the stack status string, or None when no such stack exists.
    async fn find_stack(&self, name: &str) -> Result<Option<String>, OnboardError>;
    /// Role names containing `needle` (paginated ListRoles).
    async fn list_roles_matching(&self, needle: &str) -> Result<Vec<String>, OnboardError>;
    /// Resolve a role name to its full ARN (handles path'd roles, e.g. Identity Center).
    async fn role_arn(&self, role_name: &str) -> Result<String, OnboardError>;
    async fn simulate(
        &self,
        policy_source_arn: &str,
        actions: &[String],
    ) -> Result<Vec<SimResult>, OnboardError>;
    async fn template_summary(&self, template_url: &str) -> Result<TemplateSummary, OnboardError>;
    /// Returns the StackId.
    async fn create_stack(&self, req: &CreateStackRequest) -> Result<String, OnboardError>;
    async fn stack_failure_events(&self, name: &str) -> Result<Vec<StackEvent>, OnboardError>;
    async fn stack_outputs(&self, name: &str) -> Result<Vec<StackOutput>, OnboardError>;
}
