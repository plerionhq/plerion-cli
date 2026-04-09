use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    pub id: Option<String>,
    pub risk_type_id: Option<String>,
    pub organization_id: Option<String>,
    pub tenant_id: Option<String>,
    pub integration_id: Option<String>,
    pub description: Option<String>,
    pub primary_asset_id: Option<String>,
    pub region: Option<String>,
    pub resolutions: Option<serde_json::Value>,
    pub score: Option<f64>,
    pub likelihood: Option<f64>,
    pub impact: Option<f64>,
    pub severity_level: Option<String>,
    pub factors: Option<serde_json::Value>,
    pub meta: Option<RiskMeta>,
    pub discovered_at: Option<String>,
    pub lifecycle_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiskMeta {
    pub asset_name: Option<String>,
    pub resource_type: Option<String>,
    pub full_resource_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RisksResponse {
    pub data: Vec<Risk>,
    pub meta: PaginationMeta,
}

impl TableRenderable for Risk {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "TYPE", "SEVERITY", "SCORE", "LIKELIHOOD", "IMPACT",
            "STATE", "DESCRIPTION",
            "PRIMARY ASSET ID", "ASSET NAME", "RESOURCE TYPE", "FULL RESOURCE NAME",
            "REGION", "INTEGRATION ID",
            "TENANT ID", "ORG ID",
            "DISCOVERED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        let meta = self.meta.as_ref();
        vec![
            self.id.clone().unwrap_or_default(),
            self.risk_type_id.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            self.score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.likelihood.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.impact.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.lifecycle_state.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
            self.primary_asset_id.clone().unwrap_or_default(),
            meta.and_then(|m| m.asset_name.clone()).unwrap_or_default(),
            meta.and_then(|m| m.resource_type.clone()).unwrap_or_default(),
            meta.and_then(|m| m.full_resource_name.clone()).unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
            self.integration_id.clone().unwrap_or_default(),
            self.tenant_id.clone().unwrap_or_default(),
            self.organization_id.clone().unwrap_or_default(),
            self.discovered_at.clone().unwrap_or_default(),
        ]
    }
}
