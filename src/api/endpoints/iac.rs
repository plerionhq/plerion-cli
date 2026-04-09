use crate::api::client::PlerionClient;
use crate::api::models::iac::{IacFindingsResponse, IacScanResponse, IacScansResponse, IacVulnerabilitiesResponse};
use crate::error::PlerionError;

pub async fn upload_iac_scan(
    client: &PlerionClient,
    artifact_name: &str,
    zip_bytes: bytes::Bytes,
) -> Result<IacScanResponse, PlerionError> {
    client.execute(client.upload_iac(artifact_name, zip_bytes)).await
}

pub async fn list_iac_scans(client: &PlerionClient) -> Result<IacScansResponse, PlerionError> {
    client
        .execute(client.get("/v1/tenant/shiftleft/iac/scans"))
        .await
}

pub async fn get_iac_findings(
    client: &PlerionClient,
    scan_id: &str,
) -> Result<IacFindingsResponse, PlerionError> {
    client
        .execute(client.get(&format!("/v1/tenant/shiftleft/iac/scans/{scan_id}/findings")))
        .await
}

pub async fn get_iac_vulnerabilities(
    client: &PlerionClient,
    scan_id: &str,
) -> Result<IacVulnerabilitiesResponse, PlerionError> {
    client
        .execute(client.get(&format!("/v1/tenant/shiftleft/iac/scans/{scan_id}/vulnerabilities")))
        .await
}
