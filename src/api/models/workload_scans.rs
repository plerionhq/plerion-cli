use crate::output::TableRenderable;
use serde::{Deserialize, Serialize};

/// Body of `POST /v1/tenant/workload/scans`: `asset_id` alone, or the three
/// `resource_*` fields together.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestWorkloadScanRequest {
    pub integration_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestWorkloadScanResponse {
    pub scan_id: Option<String>,
    pub status: Option<String>,
    pub execution_id: Option<String>,
}

impl TableRenderable for RequestWorkloadScanResponse {
    fn headers() -> Vec<&'static str> {
        vec!["SCAN ID", "STATUS", "EXECUTION ID"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.scan_id.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.execution_id.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkloadScan {
    pub id: Option<String>,
    pub organization_id: Option<String>,
    pub tenant_id: Option<String>,
    pub integration_id: Option<String>,
    pub asset_id: Option<String>,
    pub appliance_id: Option<String>,
    pub execution_id: Option<String>,
    #[serde(rename = "type")]
    pub scan_type: Option<String>,
    pub status: Option<String>,
    pub status_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub updated_at: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

impl TableRenderable for WorkloadScan {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "STATUS",
            "STATUS MESSAGE",
            "TYPE",
            "ASSET ID",
            "INTEGRATION ID",
            "APPLIANCE ID",
            "EXECUTION ID",
            "STARTED AT",
            "COMPLETED AT",
            "UPDATED AT",
            "ERROR CODE",
            "ERROR MESSAGE",
            "TENANT ID",
            "ORG ID",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.status_message.clone().unwrap_or_default(),
            self.scan_type.clone().unwrap_or_default(),
            self.asset_id.clone().unwrap_or_default(),
            self.integration_id.clone().unwrap_or_default(),
            self.appliance_id.clone().unwrap_or_default(),
            self.execution_id.clone().unwrap_or_default(),
            self.started_at.clone().unwrap_or_default(),
            self.completed_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
            self.error_code.clone().unwrap_or_default(),
            self.error_message.clone().unwrap_or_default(),
            self.tenant_id.clone().unwrap_or_default(),
            self.organization_id.clone().unwrap_or_default(),
        ]
    }
}
