use crate::api::client::PlerionClient;
use crate::api::models::integrations::{
    ContinuousScanningResponse, IntegrationTagsResponse, IntegrationsResponse,
    ReplaceUserDefinedTagsRequest, SetContinuousScanningRequest,
};
use crate::error::PlerionError;

pub async fn list_integrations(
    client: &PlerionClient,
    per_page: Option<u32>,
    cursor: Option<&str>,
    include_total: bool,
) -> Result<IntegrationsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/integrations");
    if let Some(v) = per_page { req = req.query(&[("perPage", v)]); }
    if let Some(v) = cursor { req = req.query(&[("cursor", v)]); }
    if include_total { req = req.query(&[("includeTotal", true)]); }
    client.execute(req).await
}

/// Replaces every user-defined tag on the integration; an empty list removes them all.
pub async fn replace_user_defined_tags(
    client: &PlerionClient,
    integration_id: &str,
    body: &ReplaceUserDefinedTagsRequest,
) -> Result<IntegrationTagsResponse, PlerionError> {
    let path = format!(
        "/v1/tenant/integrations/{}/user-defined-tags",
        super::path::segment(integration_id, "integration ID")?
    );
    client.execute(client.put(&path).json(body)).await
}

/// Turns continuous scanning on or off for the listed AWS account integrations.
pub async fn set_continuous_scanning(
    client: &PlerionClient,
    body: &SetContinuousScanningRequest,
) -> Result<ContinuousScanningResponse, PlerionError> {
    client
        .execute(client.put("/v1/tenant/integrations/continuous-scanning").json(body))
        .await
}
