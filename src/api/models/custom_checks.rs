use serde::{Deserialize, Serialize};

use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomCheckTarget {
    pub asset_type: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomCheckAdditionalAsset {
    pub asset_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomCheckDefaults {
    pub severity_level: Option<String>,
    pub message: Option<String>,
    pub informational: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Optional fields are omitted rather than written as null, so `get --output json`
/// can be edited and sent back through `update` or `dry-run`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomCheck {
    pub custom_check_id: Option<String>,
    pub slug: Option<String>,
    pub organization_id: Option<String>,
    pub tenant_id: Option<String>,
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub target: Option<CustomCheckTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_assets: Option<Vec<CustomCheckAdditionalAsset>>,
    pub defaults: Option<CustomCheckDefaults>,
    #[serde(rename = "type")]
    pub check_type: Option<String>,
    pub body: Option<String>,
    pub version: Option<u32>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
}

/// List response. Unlike most endpoints this one returns `items` and a
/// top-level `nextCursor`, absent on the last page.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomChecksResponse {
    pub items: Vec<CustomCheck>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCustomCheckResponse {
    pub accepted: bool,
}

/// The check travels as raw JSON so an unsaved edit is sent exactly as written.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCustomCheckDryRunRequest {
    pub integration_id: String,
    pub check: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomCheckDryRunStarted {
    pub dry_run_id: Option<String>,
    pub status: Option<String>,
    pub poll_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomCheckDryRunStatus {
    pub dry_run_id: Option<String>,
    pub status: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub output: Option<serde_json::Value>,
}

impl TableRenderable for CustomCheck {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "SLUG",
            "TITLE",
            "ASSET TYPE",
            "SCOPE",
            "SEVERITY",
            "VERSION",
            "UPDATED BY",
            "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        let target = self.target.as_ref();
        vec![
            self.custom_check_id.clone().unwrap_or_default(),
            self.slug.clone().unwrap_or_default(),
            self.title.clone().unwrap_or_default(),
            target
                .and_then(|t| t.asset_type.clone())
                .unwrap_or_default(),
            target.and_then(|t| t.scope.clone()).unwrap_or_default(),
            self.defaults
                .as_ref()
                .and_then(|d| d.severity_level.clone())
                .unwrap_or_default(),
            self.version.map(|v| v.to_string()).unwrap_or_default(),
            self.updated_by.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

impl TableRenderable for CustomCheckDryRunStarted {
    fn headers() -> Vec<&'static str> {
        vec!["DRY RUN ID", "STATUS", "POLL URL"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.dry_run_id.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.poll_url.clone().unwrap_or_default(),
        ]
    }
}

impl TableRenderable for CustomCheckDryRunStatus {
    fn headers() -> Vec<&'static str> {
        vec!["DRY RUN ID", "STATUS", "STARTED AT", "COMPLETED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.dry_run_id.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.started_at.clone().unwrap_or_default(),
            self.completed_at.clone().unwrap_or_default(),
        ]
    }
}
