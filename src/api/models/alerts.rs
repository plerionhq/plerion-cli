use serde::{Deserialize, Deserializer, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

/// Deserialize a value that may be a number or a stringified number (e.g. `"4.44"` or `4.44`).
fn deserialize_option_f64_or_string<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrF64 {
        F64(f64),
        Str(String),
    }

    match Option::<StringOrF64>::deserialize(deserializer)? {
        None => Ok(None),
        Some(StringOrF64::F64(v)) => Ok(Some(v)),
        Some(StringOrF64::Str(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<f64>()
                    .map(Some)
                    .map_err(|_| de::Error::custom(format!("invalid numeric string: {s}")))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: Option<String>,
    pub tenant_id: Option<String>,
    pub integration_id: Option<String>,
    pub status: Option<String>,
    pub flagged: Option<bool>,
    pub acknowledged: Option<bool>,
    pub workflow_id: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub summary: Option<Vec<String>>,
    pub alert_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64_or_string")]
    pub risk_score: Option<f64>,
    pub discovered_date: Option<String>,
    pub last_scanned_at_timestamp: Option<String>,
    pub rules_changed_at_timestamp: Option<String>,
    pub closed_at_timestamp: Option<String>,
    pub meta: Option<AlertMeta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlertMeta {
    pub region: Option<String>,
    pub service: Option<String>,
    pub provider: Option<String>,
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,
    pub resource_type: Option<String>,
    pub first_observed_at: Option<String>,
    pub provider_account_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertsResponse {
    pub data: Vec<Alert>,
    pub meta: PaginationMeta,
}

impl TableRenderable for Alert {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "TITLE", "STATUS", "TYPE", "RISK SCORE",
            "FLAGGED", "ACKNOWLEDGED",
            "TENANT ID", "INTEGRATION ID", "WORKFLOW ID",
            "PROVIDER", "SERVICE", "REGION",
            "RESOURCE ID", "RESOURCE TYPE", "RESOURCE NAME", "ACCOUNT ID",
            "FIRST OBSERVED",
            "DISCOVERED", "LAST SCANNED", "RULES CHANGED", "CLOSED AT",
            "CREATED AT", "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        let meta = self.meta.as_ref();
        vec![
            self.id.clone().unwrap_or_default(),
            self.title.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.alert_type.clone().unwrap_or_default(),
            self.risk_score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            bool_str(self.flagged),
            bool_str(self.acknowledged),
            self.tenant_id.clone().unwrap_or_default(),
            self.integration_id.clone().unwrap_or_default(),
            self.workflow_id.clone().unwrap_or_default(),
            meta.and_then(|m| m.provider.clone()).unwrap_or_default(),
            meta.and_then(|m| m.service.clone()).unwrap_or_default(),
            meta.and_then(|m| m.region.clone()).unwrap_or_default(),
            meta.and_then(|m| m.resource_id.clone()).unwrap_or_default(),
            meta.and_then(|m| m.resource_type.clone()).unwrap_or_default(),
            meta.and_then(|m| m.resource_name.clone()).unwrap_or_default(),
            meta.and_then(|m| m.provider_account_id.clone()).unwrap_or_default(),
            meta.and_then(|m| m.first_observed_at.clone()).unwrap_or_default(),
            self.discovered_date.clone().unwrap_or_default(),
            self.last_scanned_at_timestamp.clone().unwrap_or_default(),
            self.rules_changed_at_timestamp.clone().unwrap_or_default(),
            self.closed_at_timestamp.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
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
