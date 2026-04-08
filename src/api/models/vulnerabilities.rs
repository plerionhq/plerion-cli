use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::assets::PagePaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vulnerability {
    pub vulnerability_id: Option<String>,
    pub title: Option<String>,
    pub severity_level: Option<String>,
    pub asset_id: Option<String>,
    pub asset_type: Option<String>,
    pub provider: Option<String>,
    pub has_kev: Option<bool>,
    pub has_exploit: Option<bool>,
    pub has_vendor_fix: Option<bool>,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub is_exempted: Option<bool>,
    pub primary_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VulnerabilitiesResponse {
    pub data: Vec<Vulnerability>,
    pub meta: PagePaginationMeta,
}

impl TableRenderable for Vulnerability {
    fn headers() -> Vec<&'static str> {
        vec![
            "CVE / ID",
            "TITLE",
            "SEVERITY",
            "PROVIDER",
            "ASSET TYPE",
            "KEV",
            "EXPLOIT",
            "FIX",
            "FIRST OBSERVED",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.vulnerability_id.clone().unwrap_or_default(),
            truncate(&self.title.clone().unwrap_or_default(), 50),
            self.severity_level.clone().unwrap_or_default(),
            self.provider.clone().unwrap_or_default(),
            self.asset_type.clone().unwrap_or_default(),
            bool_icon(self.has_kev),
            bool_icon(self.has_exploit),
            bool_icon(self.has_vendor_fix),
            self.first_observed_at.clone().unwrap_or_default(),
        ]
    }
}

fn bool_icon(v: Option<bool>) -> String {
    match v {
        Some(true) => "yes".to_string(),
        Some(false) => "no".to_string(),
        None => String::new(),
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max])
    } else {
        s.to_string()
    }
}

// Vulnerability Exemptions

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VulnerabilityExemption {
    pub id: Option<String>,
    pub name: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExemptionsResponse {
    pub data: Vec<VulnerabilityExemption>,
}

impl TableRenderable for VulnerabilityExemption {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "REASON", "CREATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.reason.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
        ]
    }
}
