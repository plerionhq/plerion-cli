use crate::api::client::PlerionClient;
use crate::api::models::risks::RisksResponse;
use crate::error::PlerionError;

#[derive(Debug, Default)]
pub struct ListRisksParams {
    pub ids: Option<String>,
    pub risk_type_ids: Option<String>,
    pub environment_ids: Option<String>,
    pub integration_ids: Option<String>,
    pub primary_asset_ids: Option<String>,
    pub severity_levels: Option<String>,
    pub lifecycle_states: Option<String>,
    pub resource_types: Option<String>,
    pub discovered_at_start: Option<String>,
    pub discovered_at_end: Option<String>,
    pub include: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub cursor: Option<String>,
    pub per_page: Option<u32>,
    pub fields: Option<String>,
}

pub async fn list_risks(
    client: &PlerionClient,
    params: &ListRisksParams,
) -> Result<RisksResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/risks");

    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.risk_type_ids { req = req.query(&[("riskTypeIds", v)]); }
    if let Some(v) = &params.environment_ids { req = req.query(&[("environmentIds", v)]); }
    if let Some(v) = &params.integration_ids { req = req.query(&[("integrationIds", v)]); }
    if let Some(v) = &params.primary_asset_ids { req = req.query(&[("primaryAssetIds", v)]); }
    if let Some(v) = &params.severity_levels { req = req.query(&[("severityLevels", v)]); }
    if let Some(v) = &params.lifecycle_states { req = req.query(&[("lifecycleStates", v)]); }
    if let Some(v) = &params.resource_types { req = req.query(&[("resourceTypes", v)]); }
    if let Some(v) = &params.discovered_at_start { req = req.query(&[("discoveredAtStart", v)]); }
    if let Some(v) = &params.discovered_at_end { req = req.query(&[("discoveredAtEnd", v)]); }
    if let Some(v) = &params.include { req = req.query(&[("include", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = &params.cursor { req = req.query(&[("cursor", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }
    if let Some(v) = &params.fields { req = req.query(&[("fields", v)]); }

    client.execute(req).await
}
