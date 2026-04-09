use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;

// --- Scan upload response ---

#[derive(Debug, Serialize, Deserialize)]
pub struct IacScanResponse {
    pub data: Option<String>,
    pub meta: IacScanMeta,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IacScanMeta {
    pub scan_id: Option<String>,
    pub artifact_name: Option<String>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
}

// --- List scans response ---

#[derive(Debug, Serialize, Deserialize)]
pub struct IacScansResponse {
    pub data: Vec<IacScan>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IacScan {
    pub id: Option<String>,
    pub artifact_name: Option<String>,
    pub status: Option<String>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub summary: Option<IacScanSummary>,
    #[serde(default)]
    pub types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IacScanSummary {
    pub started_at: Option<serde_json::Value>,
    pub stopped_at: Option<serde_json::Value>,
    pub total_file_size: Option<u64>,
    pub total_findings: Option<u64>,
    pub total_resources: Option<u64>,
    pub total_parsing_errors: Option<u64>,
    pub total_failed_findings: Option<u64>,
    pub total_passed_findings: Option<u64>,
    pub total_skipped_findings: Option<u64>,
    pub total_vulnerabilities: Option<u64>,
}

impl TableRenderable for IacScan {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "ARTIFACT", "STATUS", "TYPES",
            "FINDINGS", "FAILED", "PASSED", "VULNS",
            "TENANT ID", "ORG ID", "CREATED AT", "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        let summary = self.summary.as_ref();
        vec![
            self.id.clone().unwrap_or_default(),
            self.artifact_name.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.types.join(", "),
            summary.and_then(|s| s.total_findings).map(|n| n.to_string()).unwrap_or_default(),
            summary.and_then(|s| s.total_failed_findings).map(|n| n.to_string()).unwrap_or_default(),
            summary.and_then(|s| s.total_passed_findings).map(|n| n.to_string()).unwrap_or_default(),
            summary.and_then(|s| s.total_vulnerabilities).map(|n| n.to_string()).unwrap_or_default(),
            self.tenant_id.clone().unwrap_or_default(),
            self.organization_id.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

// --- IaC Findings ---

#[derive(Debug, Serialize, Deserialize)]
pub struct IacFindingsResponse {
    pub data: Vec<IacFinding>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IacFinding {
    pub id: Option<String>,
    pub scan_id: Option<String>,
    #[serde(alias = "detectionID")]
    pub detection_id: Option<String>,
    pub detection_title: Option<String>,
    #[serde(rename = "type")]
    pub finding_type: Option<String>,
    pub result: Option<String>,
    pub severity_level: Option<String>,
    pub file: Option<String>,
    pub repository_path: Option<String>,
    pub line_range: Option<Vec<u64>>,
    pub resource: Option<String>,
    pub resource_tags: Option<serde_json::Value>,
    pub evaluated_keys: Option<Vec<String>>,
    pub code_block: Option<serde_json::Value>,
    #[serde(alias = "dashboardURL")]
    pub dashboard_url: Option<String>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl TableRenderable for IacFinding {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "SCAN ID", "DETECTION ID", "DETECTION TITLE",
            "TYPE", "RESULT", "SEVERITY", "FILE", "LINE RANGE",
            "RESOURCE", "DASHBOARD URL",
            "TENANT ID", "ORG ID", "CREATED AT", "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        let line_range = self.line_range.as_ref()
            .map(|lr| lr.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("-"))
            .unwrap_or_default();
        vec![
            self.id.clone().unwrap_or_default(),
            self.scan_id.clone().unwrap_or_default(),
            self.detection_id.clone().unwrap_or_default(),
            self.detection_title.clone().unwrap_or_default(),
            self.finding_type.clone().unwrap_or_default(),
            self.result.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            self.file.clone().unwrap_or_default(),
            line_range,
            self.resource.clone().unwrap_or_default(),
            self.dashboard_url.clone().unwrap_or_default(),
            self.tenant_id.clone().unwrap_or_default(),
            self.organization_id.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

// --- IaC Vulnerabilities ---

#[derive(Debug, Serialize, Deserialize)]
pub struct IacVulnerabilitiesResponse {
    pub data: Vec<IacVulnerability>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IacVulnerability {
    pub id: Option<String>,
    pub vulnerability_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub severity_level: Option<String>,
    pub severity_level_value: Option<i64>,
    pub primary_url: Option<String>,
    pub published_date: Option<String>,
    pub file: Option<String>,
    pub has_kev: Option<bool>,
    pub has_exploit: Option<bool>,
    pub packages: Option<Vec<IacVulnPackage>>,
    pub cwes: Option<Vec<serde_json::Value>>,
    pub known_exploit: Option<serde_json::Value>,
    pub exploits: Option<Vec<serde_json::Value>>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IacVulnPackage {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub package_type: Option<String>,
    pub fixed_version: Option<String>,
    pub installed_version: Option<String>,
}

impl TableRenderable for IacVulnerability {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "CVE / VULN ID", "TITLE", "SEVERITY", "SEVERITY VALUE",
            "DESCRIPTION", "FILE",
            "KEV", "EXPLOIT", "PACKAGES", "PRIMARY URL", "PUBLISHED",
            "TENANT ID", "ORG ID", "CREATED AT", "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        let packages = self.packages.as_ref()
            .map(|pkgs| pkgs.iter()
                .filter_map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", "))
            .unwrap_or_default();
        vec![
            self.id.clone().unwrap_or_default(),
            self.vulnerability_id.clone().unwrap_or_default(),
            self.title.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            self.severity_level_value.map(|n| n.to_string()).unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
            self.file.clone().unwrap_or_default(),
            bool_str(self.has_kev),
            bool_str(self.has_exploit),
            packages,
            self.primary_url.clone().unwrap_or_default(),
            self.published_date.clone().unwrap_or_default(),
            self.tenant_id.clone().unwrap_or_default(),
            self.organization_id.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

fn bool_str(v: Option<bool>) -> String {
    match v {
        Some(true) => "yes".to_string(),
        Some(false) => "no".to_string(),
        None => String::new(),
    }
}
