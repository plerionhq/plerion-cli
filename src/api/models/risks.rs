use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    pub id: Option<String>,
    pub risk_type_id: Option<String>,
    pub description: Option<String>,
    pub score: Option<f64>,
    pub severity_level: Option<String>,
    pub lifecycle_state: Option<String>,
    pub primary_asset_id: Option<String>,
    pub region: Option<String>,
    pub integration_id: Option<String>,
    pub discovered_at: Option<String>,
    pub meta: Option<RiskMeta>,
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
        vec!["ID", "TYPE", "SEVERITY", "SCORE", "STATE", "REGION", "DISCOVERED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.risk_type_id.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            self.score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.lifecycle_state.clone().unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
            self.discovered_at.clone().unwrap_or_default(),
        ]
    }
}
