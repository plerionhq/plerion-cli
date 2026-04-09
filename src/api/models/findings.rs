use serde::{Deserialize, Serialize};
use crate::api::models::assets::deserialize_option_u32_or_string;
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub id: Option<String>,
    pub schema_version: Option<String>,
    pub organization_id: Option<String>,
    pub tenant_id: Option<String>,
    pub integration_id: Option<String>,
    pub provider: Option<String>,
    pub execution_id: Option<String>,
    pub asset_id: Option<String>,
    pub provider_account_id: Option<String>,
    pub resource_type: Option<String>,
    pub detection_id: Option<String>,
    pub status: Option<String>,
    pub severity_level: Option<String>,
    pub message: Option<String>,
    pub first_observed_at: Option<String>,
    pub created_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub updated_at: Option<String>,
    pub parameters: Option<Vec<String>>,
    pub tags: Option<serde_json::Value>,
    pub full_resource_name: Option<String>,
    pub resource_id: Option<String>,
    pub provider_full_resource_name: Option<String>,
    pub region: Option<String>,
    pub service: Option<String>,
    pub likelihood: Option<String>,
    pub impact: Option<String>,
    pub calculated_severity: Option<serde_json::Value>,
    pub modified_severity_level: Option<String>,
    pub attack_paths: Option<serde_json::Value>,
    pub is_exempted: Option<bool>,
    pub meta: Option<serde_json::Value>,
    pub resource_tags: Option<serde_json::Value>,
    #[serde(alias = "resourceURL")]
    pub resource_url: Option<String>,
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
    #[serde(default, deserialize_with = "deserialize_option_u32_or_string")]
    pub per_page: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_u32_or_string")]
    pub total: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_u32_or_string")]
    pub page: Option<u32>,
    pub has_next_page: Option<bool>,
    pub has_previous_page: Option<bool>,
}

impl TableRenderable for Finding {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "DETECTION ID", "STATUS", "SEVERITY",
            "CALC SEVERITY", "MODIFIED SEVERITY", "LIKELIHOOD", "IMPACT",
            "PROVIDER", "SERVICE", "RESOURCE TYPE", "REGION",
            "ASSET ID", "RESOURCE ID", "FULL RESOURCE NAME", "PROVIDER FULL RESOURCE NAME",
            "INTEGRATION ID", "PROVIDER ACCOUNT ID", "EXECUTION ID",
            "MESSAGE", "EXEMPTED", "RESOURCE URL",
            "FIRST OBSERVED", "LAST OBSERVED", "CREATED AT", "UPDATED AT",
            "SCHEMA VERSION",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.detection_id.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            value_to_str(&self.calculated_severity),
            self.modified_severity_level.clone().unwrap_or_default(),
            self.likelihood.clone().unwrap_or_default(),
            self.impact.clone().unwrap_or_default(),
            self.provider.clone().unwrap_or_default(),
            self.service.clone().unwrap_or_default(),
            self.resource_type.clone().unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
            self.asset_id.clone().unwrap_or_default(),
            self.resource_id.clone().unwrap_or_default(),
            self.full_resource_name.clone().unwrap_or_default(),
            self.provider_full_resource_name.clone().unwrap_or_default(),
            self.integration_id.clone().unwrap_or_default(),
            self.provider_account_id.clone().unwrap_or_default(),
            self.execution_id.clone().unwrap_or_default(),
            self.message.clone().unwrap_or_default(),
            bool_str(self.is_exempted),
            self.resource_url.clone().unwrap_or_default(),
            self.first_observed_at.clone().unwrap_or_default(),
            self.last_observed_at.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
            self.schema_version.clone().unwrap_or_default(),
        ]
    }
}

fn bool_str(v: Option<bool>) -> String {
    match v {
        Some(true) => "yes".to_string(),
        Some(false) => "no".to_string(),
        None => String::new(),
    }
}

fn value_to_str(v: &Option<serde_json::Value>) -> String {
    match v {
        Some(serde_json::Value::Number(n)) => format!("{:.2}", n.as_f64().unwrap_or(0.0)),
        Some(serde_json::Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}
