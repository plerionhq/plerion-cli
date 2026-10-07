use crate::api::client::PlerionClient;
use crate::api::models::custom_reports::CustomReportsResponse;
use crate::error::PlerionError;

pub async fn list_custom_reports(
    client: &PlerionClient,
    per_page: Option<u32>,
    cursor: Option<&str>,
) -> Result<CustomReportsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/custom-reports");
    if let Some(v) = per_page {
        req = req.query(&[("perPage", v)]);
    }
    if let Some(v) = cursor {
        req = req.query(&[("cursor", v)]);
    }
    client.execute(req).await
}
