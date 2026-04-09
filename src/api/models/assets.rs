use serde::{Deserialize, Deserializer, Serialize};
use crate::output::TableRenderable;

/// Deserialize a value that may be a number or a stringified number (e.g. `"1"` or `1`).
pub(crate) fn deserialize_option_u32_or_string<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU32 {
        U32(u32),
        Str(String),
    }

    match Option::<StringOrU32>::deserialize(deserializer)? {
        None => Ok(None),
        Some(StringOrU32::U32(v)) => Ok(Some(v)),
        Some(StringOrU32::Str(s)) => s
            .parse::<u32>()
            .map(Some)
            .map_err(|_| de::Error::custom(format!("invalid numeric string: {s}"))),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: Option<String>,
    pub schema_version: Option<String>,
    pub organization_id: Option<String>,
    pub tenant_id: Option<String>,
    pub integration_id: Option<String>,
    pub execution_id: Option<String>,
    pub provider: Option<String>,
    #[serde(rename = "type")]
    pub asset_type: Option<String>,
    pub name: Option<String>,
    pub created_at: Option<String>,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub updated_at: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub is_publicly_exposed: Option<bool>,
    pub is_vulnerable: Option<bool>,
    pub number_of_low_vulnerabilities: Option<u32>,
    pub number_of_medium_vulnerabilities: Option<u32>,
    pub number_of_high_vulnerabilities: Option<u32>,
    pub number_of_critical_vulnerabilities: Option<u32>,
    pub vulnerability_score: Option<String>,
    pub has_kev: Option<bool>,
    pub has_exploit: Option<bool>,
    pub is_exploitable: Option<bool>,
    pub is_in_vpc: Option<bool>,
    pub last_scan_id: Option<String>,
    pub last_scanned_at: Option<String>,
    pub image_id: Option<String>,
    pub platform: Option<String>,
    pub has_admin_privileges: Option<bool>,
    pub has_overly_permissive_privileges: Option<bool>,
    pub has_authorizer: Option<bool>,
    pub has_tracing_enabled: Option<bool>,
    pub policy: Option<serde_json::Value>,
    pub number_of_low_secrets: Option<u32>,
    pub number_of_medium_secrets: Option<u32>,
    pub number_of_high_secrets: Option<u32>,
    pub number_of_critical_secrets: Option<u32>,
    pub low_secrets: Option<serde_json::Value>,
    pub medium_secrets: Option<serde_json::Value>,
    pub high_secrets: Option<serde_json::Value>,
    pub critical_secrets: Option<serde_json::Value>,
    pub operating_system: Option<serde_json::Value>,
    pub risk_score: Option<serde_json::Value>,
    pub operational_state: Option<String>,
    pub region: Option<String>,
    pub service: Option<String>,
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,
    pub resource_tags: Option<serde_json::Value>,
    pub resource_type: Option<String>,
    pub full_resource_name: Option<String>,
    pub provider_account_id: Option<String>,
    #[serde(alias = "resourceURL")]
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
    #[serde(default, deserialize_with = "deserialize_option_u32_or_string")]
    pub page: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_u32_or_string")]
    pub per_page: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_u32_or_string")]
    pub total: Option<u32>,
    pub has_next_page: Option<bool>,
    pub has_previous_page: Option<bool>,
}

impl TableRenderable for Asset {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "NAME", "TYPE", "PROVIDER", "REGION", "SERVICE",
            "RESOURCE TYPE", "RESOURCE ID", "RESOURCE NAME", "FULL RESOURCE NAME",
            "RISK SCORE", "VULN SCORE", "PUBLIC", "VULNERABLE", "KEV", "EXPLOIT", "EXPLOITABLE",
            "IN VPC", "ADMIN PRIVS", "OVERLY PERMISSIVE", "AUTHORIZER", "TRACING",
            "STATE", "PLATFORM", "IMAGE ID",
            "INTEGRATION ID", "ACCOUNT ID", "EXECUTION ID",
            "CRITICAL VULNS", "HIGH VULNS", "MEDIUM VULNS", "LOW VULNS",
            "CRITICAL SECRETS", "HIGH SECRETS", "MEDIUM SECRETS", "LOW SECRETS",
            "RESOURCE URL", "FIRST OBSERVED", "LAST OBSERVED", "CREATED AT", "UPDATED AT",
            "LAST SCANNED", "SCHEMA VERSION",
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
            self.resource_name.clone().unwrap_or_default(),
            self.full_resource_name.clone().unwrap_or_default(),
            risk,
            self.vulnerability_score.clone().unwrap_or_default(),
            bool_to_str(self.is_publicly_exposed),
            bool_to_str(self.is_vulnerable),
            bool_to_str(self.has_kev),
            bool_to_str(self.has_exploit),
            bool_to_str(self.is_exploitable),
            bool_to_str(self.is_in_vpc),
            bool_to_str(self.has_admin_privileges),
            bool_to_str(self.has_overly_permissive_privileges),
            bool_to_str(self.has_authorizer),
            bool_to_str(self.has_tracing_enabled),
            self.operational_state.clone().unwrap_or_default(),
            self.platform.clone().unwrap_or_default(),
            self.image_id.clone().unwrap_or_default(),
            self.integration_id.clone().unwrap_or_default(),
            self.provider_account_id.clone().unwrap_or_default(),
            self.execution_id.clone().unwrap_or_default(),
            opt_u32(self.number_of_critical_vulnerabilities),
            opt_u32(self.number_of_high_vulnerabilities),
            opt_u32(self.number_of_medium_vulnerabilities),
            opt_u32(self.number_of_low_vulnerabilities),
            opt_u32(self.number_of_critical_secrets),
            opt_u32(self.number_of_high_secrets),
            opt_u32(self.number_of_medium_secrets),
            opt_u32(self.number_of_low_secrets),
            self.resource_url.clone().unwrap_or_default(),
            self.first_observed_at.clone().unwrap_or_default(),
            self.last_observed_at.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
            self.last_scanned_at.clone().unwrap_or_default(),
            self.schema_version.clone().unwrap_or_default(),
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

fn opt_u32(v: Option<u32>) -> String {
    v.map(|n| n.to_string()).unwrap_or_default()
}
