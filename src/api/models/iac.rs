use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IacScan {
    pub scan_id: Option<String>,
    pub artifact_name: Option<String>,
    pub status: Option<String>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct IacScansResponse {
    pub data: Vec<IacScan>,
}

impl TableRenderable for IacScan {
    fn headers() -> Vec<&'static str> {
        vec!["SCAN ID", "ARTIFACT", "STATUS", "CREATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.scan_id.clone().unwrap_or_default(),
            self.artifact_name.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IacFinding {
    pub id: Option<String>,
    pub severity_level: Option<String>,
    pub resource_type: Option<String>,
    pub message: Option<String>,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IacFindingsResponse {
    pub data: Vec<IacFinding>,
}

impl TableRenderable for IacFinding {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "SEVERITY", "RESOURCE TYPE", "FILE", "LINE", "MESSAGE"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.severity_level.clone().unwrap_or_default(),
            self.resource_type.clone().unwrap_or_default(),
            self.file_path.clone().unwrap_or_default(),
            self.line_number.map(|n| n.to_string()).unwrap_or_default(),
            self.message.clone().unwrap_or_default(),
        ]
    }
}
