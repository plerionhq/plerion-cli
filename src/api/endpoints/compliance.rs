use crate::api::client::PlerionClient;
use crate::api::models::compliance::ComplianceFrameworksResponse;
use crate::error::PlerionError;

pub async fn list_compliance_frameworks(
    client: &PlerionClient,
    custom: Option<bool>,
) -> Result<ComplianceFrameworksResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/compliance-frameworks");
    if let Some(v) = custom { req = req.query(&[("custom", v)]); }
    client.execute(req).await
}

pub async fn request_compliance_report(
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

pub async fn download_compliance_report(
    client: &PlerionClient,
    integration_id: &str,
    framework_id: &str,
) -> Result<bytes::Bytes, PlerionError> {
    let req = client.get(&format!(
        "/v1/tenant/integrations/{integration_id}/compliance-frameworks/{framework_id}/download"
    ));
    client.execute_bytes(req).await
}
