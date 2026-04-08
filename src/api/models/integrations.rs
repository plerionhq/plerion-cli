use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    pub integration_id: Option<String>,
    pub name: Option<String>,
    pub provider: Option<String>,
    #[serde(rename = "type")]
    pub integration_type: Option<String>,
    pub status: Option<String>,
    pub risk_score: Option<f64>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    // Provider-specific
    pub aws_account_id: Option<String>,
    pub azure_subscription_id: Option<String>,
    pub gcp_project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrationsResponse {
    pub data: Vec<Integration>,
    pub meta: PaginationMeta,
}

impl TableRenderable for Integration {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "PROVIDER", "TYPE", "STATUS", "RISK SCORE", "ACCOUNT/PROJECT"]
    }

    fn row(&self) -> Vec<String> {
        let account = self.aws_account_id.clone()
            .or_else(|| self.azure_subscription_id.clone())
            .or_else(|| self.gcp_project_id.clone())
            .unwrap_or_default();
        vec![
            self.integration_id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.provider.clone().unwrap_or_default(),
            self.integration_type.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.risk_score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            account,
        ]
    }
}
