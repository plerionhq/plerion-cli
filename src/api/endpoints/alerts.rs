use crate::api::client::PlerionClient;
use crate::api::models::alerts::AlertsResponse;
use crate::error::PlerionError;

#[derive(Debug, Default)]
pub struct ListAlertsParams {
    pub ids: Option<String>,
    pub workflow_ids: Option<String>,
    pub integration_ids: Option<String>,
    pub asset_group_ids: Option<String>,
    pub statuses: Option<String>,
    pub resource_types: Option<String>,
    pub providers: Option<String>,
    pub alert_types: Option<String>,
    pub flagged: Option<bool>,
    pub acknowledged: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub cursor: Option<String>,
    pub per_page: Option<u32>,
}

pub async fn list_alerts(
    client: &PlerionClient,
    params: &ListAlertsParams,
) -> Result<AlertsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/alerts");

    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.workflow_ids { req = req.query(&[("workflowIds", v)]); }
    if let Some(v) = &params.integration_ids { req = req.query(&[("integrationIds", v)]); }
    if let Some(v) = &params.asset_group_ids { req = req.query(&[("assetGroupIds", v)]); }
    if let Some(v) = &params.statuses { req = req.query(&[("statuses", v)]); }
    if let Some(v) = &params.resource_types { req = req.query(&[("resourceTypes", v)]); }
    if let Some(v) = &params.providers { req = req.query(&[("providers", v)]); }
    if let Some(v) = &params.alert_types { req = req.query(&[("alertTypes", v)]); }
    if let Some(v) = params.flagged { req = req.query(&[("flagged", v)]); }
    if let Some(v) = params.acknowledged { req = req.query(&[("acknowledged", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = &params.cursor { req = req.query(&[("cursor", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }

    client.execute(req).await
}
