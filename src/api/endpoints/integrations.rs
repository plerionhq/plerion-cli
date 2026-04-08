use crate::api::client::PlerionClient;
use crate::api::models::integrations::IntegrationsResponse;
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
