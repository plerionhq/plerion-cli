use crate::api::client::PlerionClient;
use crate::api::models::well_architected::WellArchitectedResponse;
use crate::error::PlerionError;

pub async fn list_well_architected_frameworks(
    client: &PlerionClient,
) -> Result<WellArchitectedResponse, PlerionError> {
    client.execute(client.get("/v1/tenant/well-architected-frameworks")).await
}

pub async fn request_well_architected_report(
    client: &PlerionClient,
    integration_id: &str,
    framework_id: &str,
) -> Result<serde_json::Value, PlerionError> {
    client
        .execute(client.post(&format!(
            "/v1/tenant/integrations/{integration_id}/frameworks/{framework_id}/reports"
        )))
        .await
}

pub async fn download_well_architected_report(
    client: &PlerionClient,
    integration_id: &str,
    framework_id: &str,
) -> Result<bytes::Bytes, PlerionError> {
    let req = client.get(&format!(
        "/v1/tenant/integrations/{integration_id}/well-architected-frameworks/{framework_id}/download"
    ));
    client.execute_bytes(req).await
}
