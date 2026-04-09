use crate::api::client::PlerionClient;
use crate::api::models::vulnerabilities::{VulnerabilitiesResponse, ExemptionsResponse};
use crate::error::PlerionError;

#[derive(Debug, Default)]
pub struct ListVulnerabilitiesParams {
    pub vulnerability_ids: Option<String>,
    pub asset_ids: Option<String>,
    pub providers: Option<String>,
    pub integration_ids: Option<String>,
    pub asset_group_ids: Option<String>,
    pub environment_ids: Option<String>,
    pub package_name: Option<String>,
    pub regions: Option<String>,
    pub has_kev: Option<bool>,
    pub is_exempted: Option<bool>,
    pub is_exploitable: Option<bool>,
    pub has_exploit: Option<bool>,
    pub has_vendor_fix: Option<bool>,
    pub severity_levels: Option<String>,
    pub first_observed_at_start: Option<String>,
    pub first_observed_at_end: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn list_vulnerabilities(
    client: &PlerionClient,
    params: &ListVulnerabilitiesParams,
) -> Result<VulnerabilitiesResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/vulnerabilities");

    if let Some(v) = &params.vulnerability_ids { req = req.query(&[("vulnerabilityIds", v)]); }
    if let Some(v) = &params.asset_ids { req = req.query(&[("assetIds", v)]); }
    if let Some(v) = &params.providers { req = req.query(&[("providers", v)]); }
    if let Some(v) = &params.integration_ids { req = req.query(&[("integrationIds", v)]); }
    if let Some(v) = &params.asset_group_ids { req = req.query(&[("assetGroupIds", v)]); }
    if let Some(v) = &params.environment_ids { req = req.query(&[("environmentIds", v)]); }
    if let Some(v) = &params.package_name { req = req.query(&[("packageName", v)]); }
    if let Some(v) = &params.regions { req = req.query(&[("regions", v)]); }
    if let Some(v) = params.has_kev { req = req.query(&[("hasKev", v)]); }
    if let Some(v) = params.is_exempted { req = req.query(&[("isExempted", v)]); }
    if let Some(v) = params.is_exploitable { req = req.query(&[("isExploitable", v)]); }
    if let Some(v) = params.has_exploit { req = req.query(&[("hasExploit", v)]); }
    if let Some(v) = params.has_vendor_fix { req = req.query(&[("hasVendorFix", v)]); }
    if let Some(v) = &params.severity_levels { req = req.query(&[("severityLevels", v)]); }
    if let Some(v) = &params.first_observed_at_start { req = req.query(&[("firstObservedAtStart", v)]); }
    if let Some(v) = &params.first_observed_at_end { req = req.query(&[("firstObservedAtEnd", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = params.page { req = req.query(&[("page", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }

    client.execute(req).await
}

pub async fn list_exemptions(
    client: &PlerionClient,
    profile_id: &str,
) -> Result<ExemptionsResponse, PlerionError> {
    client
        .execute(client.get(&format!("/v1/tenant/profiles/{profile_id}/vulnerability/exemptions")))
        .await
}

pub async fn get_exemption(
    client: &PlerionClient,
    profile_id: &str,
    exemption_id: &str,
) -> Result<serde_json::Value, PlerionError> {
    client
        .execute(client.get(&format!(
            "/v1/tenant/profiles/{profile_id}/vulnerability/exemptions/{exemption_id}"
        )))
        .await
}

pub async fn create_exemption(
    client: &PlerionClient,
    profile_id: &str,
    body: serde_json::Value,
) -> Result<serde_json::Value, PlerionError> {
    client
        .execute(
            client
                .post(&format!("/v1/tenant/profiles/{profile_id}/vulnerability/exemptions"))
                .json(&body),
        )
        .await
}

pub async fn update_exemption(
    client: &PlerionClient,
    profile_id: &str,
    exemption_id: &str,
    body: serde_json::Value,
) -> Result<serde_json::Value, PlerionError> {
    client
        .execute(
            client
                .patch(&format!(
                    "/v1/tenant/profiles/{profile_id}/vulnerability/exemptions/{exemption_id}"
                ))
                .json(&body),
        )
        .await
}

pub async fn delete_exemption(
    client: &PlerionClient,
    profile_id: &str,
    exemption_id: &str,
) -> Result<(), PlerionError> {
    let req = client.delete(&format!(
        "/v1/tenant/profiles/{profile_id}/vulnerability/exemptions/{exemption_id}"
    ));
    let resp = req.send().await?;
    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status().as_u16();
        let message = resp.text().await.unwrap_or_default();
        Err(PlerionError::ApiError { status, message })
    }
}
