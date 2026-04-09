use crate::api::client::PlerionClient;
use crate::api::models::assets::AssetsResponse;
use crate::error::PlerionError;

#[derive(Debug, Default)]
pub struct ListAssetsParams {
    pub ids: Option<String>,
    pub regions: Option<String>,
    pub integration_ids: Option<String>,
    pub asset_group_ids: Option<String>,
    pub environment_ids: Option<String>,
    pub severity_levels: Option<String>,
    pub resource_types: Option<String>,
    pub providers: Option<String>,
    pub services: Option<String>,
    pub is_publicly_exposed: Option<bool>,
    pub is_vulnerable: Option<bool>,
    pub has_kev: Option<bool>,
    pub has_exploit: Option<bool>,
    pub has_admin_privileges: Option<bool>,
    pub is_susceptible_to_privilege_escalation: Option<bool>,
    pub execution_ids: Option<String>,
    pub secrets_levels: Option<String>,
    pub first_observed_at_start: Option<String>,
    pub first_observed_at_end: Option<String>,
    pub has_overly_permissive_privileges: Option<bool>,
    pub is_exploitable: Option<bool>,
    pub metadata: Option<String>,
    pub query: Option<String>,
    pub risk_score_gte: Option<f64>,
    pub operational_states: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn list_assets(
    client: &PlerionClient,
    params: &ListAssetsParams,
) -> Result<AssetsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/assets");

    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.regions { req = req.query(&[("regions", v)]); }
    if let Some(v) = &params.integration_ids { req = req.query(&[("integrationIds", v)]); }
    if let Some(v) = &params.asset_group_ids { req = req.query(&[("assetGroupIds", v)]); }
    if let Some(v) = &params.environment_ids { req = req.query(&[("environmentIds", v)]); }
    if let Some(v) = &params.severity_levels { req = req.query(&[("severityLevels", v)]); }
    if let Some(v) = &params.resource_types { req = req.query(&[("resourceTypes", v)]); }
    if let Some(v) = &params.providers { req = req.query(&[("providers", v)]); }
    if let Some(v) = &params.services { req = req.query(&[("services", v)]); }
    if let Some(v) = params.is_publicly_exposed { req = req.query(&[("isPubliclyExposed", v)]); }
    if let Some(v) = params.is_vulnerable { req = req.query(&[("isVulnerable", v)]); }
    if let Some(v) = params.has_kev { req = req.query(&[("hasKev", v)]); }
    if let Some(v) = params.has_exploit { req = req.query(&[("hasExploit", v)]); }
    if let Some(v) = params.has_admin_privileges { req = req.query(&[("hasAdminPrivileges", v)]); }
    if let Some(v) = params.is_susceptible_to_privilege_escalation { req = req.query(&[("isSusceptibleToPrivilegeEscalation", v)]); }
    if let Some(v) = &params.execution_ids { req = req.query(&[("executionIds", v)]); }
    if let Some(v) = &params.secrets_levels { req = req.query(&[("secretsLevels", v)]); }
    if let Some(v) = &params.first_observed_at_start { req = req.query(&[("firstObservedAtStart", v)]); }
    if let Some(v) = &params.first_observed_at_end { req = req.query(&[("firstObservedAtEnd", v)]); }
    if let Some(v) = params.has_overly_permissive_privileges { req = req.query(&[("hasOverlyPermissivePrivileges", v)]); }
    if let Some(v) = params.is_exploitable { req = req.query(&[("isExploitable", v)]); }
    if let Some(v) = &params.metadata { req = req.query(&[("metadata", v)]); }
    if let Some(v) = &params.query { req = req.query(&[("query", v)]); }
    if let Some(v) = params.risk_score_gte { req = req.query(&[("riskScoreGte", v)]); }
    if let Some(v) = &params.operational_states { req = req.query(&[("operationalStates", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = params.page { req = req.query(&[("page", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }

    client.execute(req).await
}

pub async fn get_asset(
    client: &PlerionClient,
    asset_id: &str,
    include: Option<&str>,
) -> Result<serde_json::Value, PlerionError> {
    let mut req = client.get(&format!("/v1/tenant/assets/{asset_id}"));
    if let Some(v) = include { req = req.query(&[("include", v)]); }
    client.execute(req).await
}

pub async fn get_asset_sbom(
    client: &PlerionClient,
    asset_id: &str,
) -> Result<serde_json::Value, PlerionError> {
    client.execute(client.get(&format!("/v1/tenant/assets/{asset_id}/sbom"))).await
}
