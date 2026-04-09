use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantData {
    pub tenant_id: String,
    pub organization_id: String,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub risk_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantResponse {
    pub data: TenantData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantUsageResponse {
    pub data: TenantUsageData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantUsageData {
    pub assets: Option<u64>,
    pub integrations: Option<u64>,
}

impl TableRenderable for TenantUsageData {
    fn headers() -> Vec<&'static str> {
        vec!["ASSETS", "INTEGRATIONS"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.assets.map(|v| v.to_string()).unwrap_or_default(),
            self.integrations.map(|v| v.to_string()).unwrap_or_default(),
        ]
    }
}

impl TableRenderable for TenantData {
    fn headers() -> Vec<&'static str> {
        vec!["TENANT ID", "ORG ID", "NAME", "RISK SCORE", "CREATED AT", "UPDATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.tenant_id.clone(),
            self.organization_id.clone(),
            self.name.clone(),
            self.risk_score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}
