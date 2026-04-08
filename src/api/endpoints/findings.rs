use crate::api::client::PlerionClient;
use crate::api::models::findings::FindingsResponse;
use crate::error::PlerionError;

#[derive(Debug, Default)]
pub struct ListFindingsParams {
    pub ids: Option<String>,
    pub detection_ids: Option<String>,
    pub regions: Option<String>,
    pub asset_ids: Option<String>,
    pub integration_ids: Option<String>,
    pub asset_group_ids: Option<String>,
    pub environment_ids: Option<String>,
    pub severity_levels: Option<String>,
    pub statuses: Option<String>,
    pub resource_types: Option<String>,
    pub providers: Option<String>,
    pub services: Option<String>,
    pub is_exempted: Option<bool>,
    pub first_observed_at_start: Option<String>,
    pub first_observed_at_end: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub per_page: Option<u32>,
    pub cursor: Option<String>,
}

pub async fn list_findings(
    client: &PlerionClient,
    params: &ListFindingsParams,
) -> Result<FindingsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/findings");

    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.detection_ids { req = req.query(&[("detectionIds", v)]); }
    if let Some(v) = &params.regions { req = req.query(&[("regions", v)]); }
    if let Some(v) = &params.asset_ids { req = req.query(&[("assetIds", v)]); }
    if let Some(v) = &params.integration_ids { req = req.query(&[("integrationIds", v)]); }
    if let Some(v) = &params.asset_group_ids { req = req.query(&[("assetGroupIds", v)]); }
    if let Some(v) = &params.environment_ids { req = req.query(&[("environmentIds", v)]); }
    if let Some(v) = &params.severity_levels { req = req.query(&[("severityLevels", v)]); }
    if let Some(v) = &params.statuses { req = req.query(&[("statuses", v)]); }
    if let Some(v) = &params.resource_types { req = req.query(&[("resourceTypes", v)]); }
    if let Some(v) = &params.providers { req = req.query(&[("providers", v)]); }
    if let Some(v) = &params.services { req = req.query(&[("services", v)]); }
    if let Some(v) = params.is_exempted { req = req.query(&[("isExempted", v)]); }
    if let Some(v) = &params.first_observed_at_start { req = req.query(&[("firstObservedAtStart", v)]); }
    if let Some(v) = &params.first_observed_at_end { req = req.query(&[("firstObservedAtEnd", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }
    if let Some(v) = &params.cursor { req = req.query(&[("cursor", v)]); }

    client.execute(req).await
}
