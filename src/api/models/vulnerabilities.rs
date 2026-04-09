use serde::{Deserialize, Serialize};
use crate::api::models::assets::{PagePaginationMeta, deserialize_option_u32_or_string};
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vulnerability {
    pub schema_version: Option<String>,
    pub asset_id: Option<String>,
    pub organization_id: Option<String>,
    pub tenant_id: Option<String>,
    pub integration_id: Option<String>,
    pub vulnerability_id: Option<String>,
    pub provider: Option<String>,
    pub asset_type: Option<String>,
    pub description: Option<String>,
    pub severity_level: Option<String>,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub published_date: Option<String>,
    pub execution_id: Option<String>,
    pub title: Option<String>,
    pub target_name: Option<String>,
    pub severity_source: Option<String>,
    pub primary_url: Option<String>,
    pub packages: Option<serde_json::Value>,
    pub cwes: Option<serde_json::Value>,
    pub has_kev: Option<bool>,
    pub has_exploit: Option<bool>,
    pub has_vendor_fix: Option<bool>,
    pub known_exploit: Option<serde_json::Value>,
    pub exploits: Option<serde_json::Value>,
    pub exemptions: Option<serde_json::Value>,
    pub severity_level_value: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VulnerabilitiesResponse {
    pub data: Vec<Vulnerability>,
    pub meta: PagePaginationMeta,
}

impl TableRenderable for Vulnerability {
    fn headers() -> Vec<&'static str> {
        vec![
            "CVE / ID", "TITLE", "SEVERITY", "SEVERITY VALUE", "SEVERITY SOURCE",
            "PROVIDER", "ASSET ID", "ASSET TYPE", "TARGET NAME",
            "DESCRIPTION", "PRIMARY URL",
            "KEV", "EXPLOIT", "FIX",
            "PUBLISHED", "FIRST OBSERVED", "LAST OBSERVED",
            "INTEGRATION ID", "TENANT ID", "ORG ID", "EXECUTION ID",
            "SCHEMA VERSION",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.vulnerability_id.clone().unwrap_or_default(),
            self.title.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            self.severity_level_value.map(|n| n.to_string()).unwrap_or_default(),
            self.severity_source.clone().unwrap_or_default(),
            self.provider.clone().unwrap_or_default(),
            self.asset_id.clone().unwrap_or_default(),
            self.asset_type.clone().unwrap_or_default(),
            self.target_name.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
            self.primary_url.clone().unwrap_or_default(),
            bool_icon(self.has_kev),
            bool_icon(self.has_exploit),
            bool_icon(self.has_vendor_fix),
            self.published_date.clone().unwrap_or_default(),
            self.first_observed_at.clone().unwrap_or_default(),
            self.last_observed_at.clone().unwrap_or_default(),
            self.integration_id.clone().unwrap_or_default(),
            self.tenant_id.clone().unwrap_or_default(),
            self.organization_id.clone().unwrap_or_default(),
            self.execution_id.clone().unwrap_or_default(),
            self.schema_version.clone().unwrap_or_default(),
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

// Vulnerability Exemptions

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VulnerabilityExemption {
    #[serde(alias = "exemptionId")]
    pub id: Option<String>,
    pub profile_id: Option<String>,
    pub name: Option<String>,
    pub audit_note: Option<String>,
    pub reason: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExemptionsResponse {
    pub data: Vec<VulnerabilityExemption>,
    #[serde(default)]
    pub meta: ExemptionsPaginationMeta,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExemptionsPaginationMeta {
    pub has_next: Option<bool>,
    pub next_cursor: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_u32_or_string")]
    pub total: Option<u32>,
}

impl TableRenderable for VulnerabilityExemption {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "PROFILE ID", "NAME", "REASON", "AUDIT NOTE",
            "CREATED BY", "UPDATED BY",
            "CREATED AT", "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.profile_id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.reason.clone().unwrap_or_default(),
            self.audit_note.clone().unwrap_or_default(),
            self.created_by.clone().unwrap_or_default(),
            self.updated_by.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}
