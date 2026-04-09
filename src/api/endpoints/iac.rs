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

#[derive(Debug, Default)]
pub struct ListIacScansParams {
    pub ids: Option<String>,
    pub artifact_names: Option<String>,
    pub statuses: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn list_iac_scans(
    client: &PlerionClient,
    params: &ListIacScansParams,
) -> Result<IacScansResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/shiftleft/iac/scans");
    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.artifact_names { req = req.query(&[("artifactNames", v)]); }
    if let Some(v) = &params.statuses { req = req.query(&[("statuses", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = params.page { req = req.query(&[("page", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }
    client.execute(req).await
}

#[derive(Debug, Default)]
pub struct ListIacFindingsParams {
    pub ids: Option<String>,
    pub results: Option<String>,
    pub detection_ids: Option<String>,
    pub types: Option<String>,
    pub files: Option<String>,
    pub severity_levels: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn get_iac_findings(
    client: &PlerionClient,
    scan_id: &str,
    params: &ListIacFindingsParams,
) -> Result<IacFindingsResponse, PlerionError> {
    let mut req = client.get(&format!("/v1/tenant/shiftleft/iac/scans/{scan_id}/findings"));
    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.results { req = req.query(&[("results", v)]); }
    if let Some(v) = &params.detection_ids { req = req.query(&[("detectionIds", v)]); }
    if let Some(v) = &params.types { req = req.query(&[("types", v)]); }
    if let Some(v) = &params.files { req = req.query(&[("files", v)]); }
    if let Some(v) = &params.severity_levels { req = req.query(&[("severityLevels", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = params.page { req = req.query(&[("page", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }
    client.execute(req).await
}

#[derive(Debug, Default)]
pub struct ListIacVulnerabilitiesParams {
    pub ids: Option<String>,
    pub vulnerability_ids: Option<String>,
    pub severity_sources: Option<String>,
    pub files: Option<String>,
    pub has_kevs: Option<bool>,
    pub has_exploits: Option<bool>,
    pub severity_levels: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn get_iac_vulnerabilities(
    client: &PlerionClient,
    scan_id: &str,
    params: &ListIacVulnerabilitiesParams,
) -> Result<IacVulnerabilitiesResponse, PlerionError> {
    let mut req = client.get(&format!("/v1/tenant/shiftleft/iac/scans/{scan_id}/vulnerabilities"));
    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.vulnerability_ids { req = req.query(&[("vulnerabilityIds", v)]); }
    if let Some(v) = &params.severity_sources { req = req.query(&[("severitySources", v)]); }
    if let Some(v) = &params.files { req = req.query(&[("files", v)]); }
    if let Some(v) = params.has_kevs { req = req.query(&[("hasKevs", v)]); }
    if let Some(v) = params.has_exploits { req = req.query(&[("hasExploits", v)]); }
    if let Some(v) = &params.severity_levels { req = req.query(&[("severityLevels", v)]); }
    if let Some(v) = &params.sort_by { req = req.query(&[("sortBy", v)]); }
    if let Some(v) = &params.sort_order { req = req.query(&[("sortOrder", v)]); }
    if let Some(v) = params.page { req = req.query(&[("page", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }
    client.execute(req).await
}
