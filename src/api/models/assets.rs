use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub asset_type: Option<String>,
    pub provider: Option<String>,
    pub region: Option<String>,
    pub service: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub risk_score: Option<serde_json::Value>,
    pub is_publicly_exposed: Option<bool>,
    pub is_vulnerable: Option<bool>,
    pub has_kev: Option<bool>,
    pub operational_state: Option<String>,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub integration_id: Option<String>,
    pub provider_account_id: Option<String>,
    pub number_of_critical_vulnerabilities: Option<u32>,
    pub number_of_high_vulnerabilities: Option<u32>,
    pub number_of_medium_vulnerabilities: Option<u32>,
    pub number_of_low_vulnerabilities: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsResponse {
    pub data: Vec<Asset>,
    pub meta: PagePaginationMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PagePaginationMeta {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub total: Option<u32>,
    pub has_next_page: Option<bool>,
    pub has_previous_page: Option<bool>,
}

impl TableRenderable for Asset {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "NAME",
            "TYPE",
            "PROVIDER",
            "REGION",
            "RISK SCORE",
            "PUBLIC",
            "VULNERABLE",
            "STATE",
        ]
    }

    fn row(&self) -> Vec<String> {
        let risk = match &self.risk_score {
            Some(serde_json::Value::Number(n)) => format!("{:.2}", n.as_f64().unwrap_or(0.0)),
            Some(serde_json::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        vec![
            self.id.as_deref().map(truncate_id).unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.resource_type.clone().unwrap_or_default(),
            self.provider.clone().unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
            risk,
            bool_to_str(self.is_publicly_exposed),
            bool_to_str(self.is_vulnerable),
            self.operational_state.clone().unwrap_or_default(),
        ]
    }
}

fn bool_to_str(v: Option<bool>) -> String {
    match v {
        Some(true) => "yes".to_string(),
        Some(false) => "no".to_string(),
        None => String::new(),
    }
}

fn truncate_id(id: &str) -> String {
    // Show last 36 chars of PRN-style IDs for table readability
    if id.len() > 40 {
        format!("...{}", &id[id.len() - 36..])
    } else {
        id.to_string()
    }
}
