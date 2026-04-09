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
            "SERVICE",
            "RESOURCE TYPE",
            "RESOURCE ID",
            "RISK SCORE",
            "PUBLIC",
            "VULNERABLE",
            "KEV",
            "STATE",
            "INTEGRATION ID",
            "ACCOUNT ID",
            "CRITICAL VULNS",
            "HIGH VULNS",
            "MEDIUM VULNS",
            "LOW VULNS",
            "RESOURCE URL",
            "FIRST OBSERVED",
            "LAST OBSERVED",
        ]
    }

    fn row(&self) -> Vec<String> {
        let risk = match &self.risk_score {
            Some(serde_json::Value::Number(n)) => format!("{:.2}", n.as_f64().unwrap_or(0.0)),
            Some(serde_json::Value::String(s)) => s.clone(),
            _ => String::new(),
        };
        vec![
            self.id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.asset_type.clone().unwrap_or_default(),
            self.provider.clone().unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
            self.service.clone().unwrap_or_default(),
            self.resource_type.clone().unwrap_or_default(),
            self.resource_id.clone().unwrap_or_default(),
            risk,
            bool_to_str(self.is_publicly_exposed),
            bool_to_str(self.is_vulnerable),
            bool_to_str(self.has_kev),
            self.operational_state.clone().unwrap_or_default(),
            self.integration_id.clone().unwrap_or_default(),
            self.provider_account_id.clone().unwrap_or_default(),
            self.number_of_critical_vulnerabilities.map(|n| n.to_string()).unwrap_or_default(),
            self.number_of_high_vulnerabilities.map(|n| n.to_string()).unwrap_or_default(),
            self.number_of_medium_vulnerabilities.map(|n| n.to_string()).unwrap_or_default(),
            self.number_of_low_vulnerabilities.map(|n| n.to_string()).unwrap_or_default(),
            self.resource_url.clone().unwrap_or_default(),
            self.first_observed_at.clone().unwrap_or_default(),
            self.last_observed_at.clone().unwrap_or_default(),
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

