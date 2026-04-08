use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: Option<String>,
    pub status: Option<String>,
    pub title: Option<String>,
    pub alert_type: Option<String>,
    pub risk_score: Option<f64>,
    pub flagged: Option<bool>,
    pub acknowledged: Option<bool>,
    pub integration_id: Option<String>,
    pub workflow_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub meta: Option<AlertMeta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlertMeta {
    pub region: Option<String>,
    pub service: Option<String>,
    pub provider: Option<String>,
    pub resource_type: Option<String>,
    pub resource_name: Option<String>,
    pub provider_account_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertsResponse {
    pub data: Vec<Alert>,
    pub meta: PaginationMeta,
}

impl TableRenderable for Alert {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "TITLE", "STATUS", "TYPE", "RISK SCORE", "FLAGGED", "CREATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.as_deref().map(|s| &s[s.len().saturating_sub(36)..]).unwrap_or_default().to_string(),
            self.title.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.alert_type.clone().unwrap_or_default(),
            self.risk_score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.flagged.map(|b| if b { "yes" } else { "no" }).unwrap_or_default().to_string(),
            self.created_at.clone().unwrap_or_default(),
        ]
    }
}
