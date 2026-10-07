use crate::api::client::PlerionClient;
use crate::api::endpoints::path::segment;
use crate::api::models::workload_scans::{
    RequestWorkloadScanRequest, RequestWorkloadScanResponse, WorkloadScan,
};
use crate::error::PlerionError;

pub async fn request_workload_scan(
    client: &PlerionClient,
    body: &RequestWorkloadScanRequest,
) -> Result<RequestWorkloadScanResponse, PlerionError> {
    client
        .execute(client.post("/v1/tenant/workload/scans").json(body))
        .await
}

pub async fn get_workload_scan(
    client: &PlerionClient,
    scan_id: &str,
) -> Result<WorkloadScan, PlerionError> {
    let path = format!("/v1/tenant/workload/scans/{}", segment(scan_id, "scan ID")?);
    client.execute(client.get(&path)).await
}
