use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub id: Option<String>,
    pub detection_id: Option<String>,
    pub status: Option<String>,
    pub severity_level: Option<String>,
    pub provider: Option<String>,
    pub resource_type: Option<String>,
    pub region: Option<String>,
    pub service: Option<String>,
    pub message: Option<String>,
    pub full_resource_name: Option<String>,
    pub asset_id: Option<String>,
    pub integration_id: Option<String>,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub is_exempted: Option<bool>,
    pub risk_score: Option<f64>,
    pub calculated_severity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindingsResponse {
    pub data: Vec<Finding>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    pub cursor: Option<String>,
    pub per_page: Option<u32>,
    pub total: Option<u32>,
    pub page: Option<u32>,
    pub has_next_page: Option<bool>,
    pub has_previous_page: Option<bool>,
}

impl TableRenderable for Finding {
    fn headers() -> Vec<&'static str> {
        vec![
            "DETECTION ID",
            "STATUS",
            "SEVERITY",
            "PROVIDER",
            "RESOURCE TYPE",
            "REGION",
            "FIRST OBSERVED",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.detection_id.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            self.provider.clone().unwrap_or_default(),
            self.resource_type.clone().unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
            self.first_observed_at.clone().unwrap_or_default(),
        ]
    }
}
