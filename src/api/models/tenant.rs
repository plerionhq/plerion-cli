use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantData {
    pub tenant_id: String,
    pub organization_id: String,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub risk_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantResponse {
    pub data: TenantData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantUsageResponse {
    pub data: TenantUsageData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TenantUsageData {
    pub assets: Option<u64>,
    pub integrations: Option<u64>,
}

impl TableRenderable for TenantUsageData {
    fn headers() -> Vec<&'static str> {
        vec!["ASSETS", "INTEGRATIONS"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.assets.map(|v| v.to_string()).unwrap_or_default(),
            self.integrations.map(|v| v.to_string()).unwrap_or_default(),
        ]
    }
}

impl TableRenderable for TenantData {
    fn headers() -> Vec<&'static str> {
        vec!["TENANT ID", "ORG ID", "NAME", "RISK SCORE", "CREATED AT", "UPDATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.tenant_id.clone(),
            self.organization_id.clone(),
            self.name.clone(),
            self.risk_score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeDashboardPreference {
    pub tenant_id: Option<String>,
    pub home_report_id: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeDashboardResponse {
    pub data: HomeDashboardPreference,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeDashboardRequest {
    pub home_report_id: String,
}

impl TableRenderable for HomeDashboardPreference {
    fn headers() -> Vec<&'static str> {
        vec!["TENANT ID", "HOME REPORT ID", "UPDATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.tenant_id.clone().unwrap_or_default(),
            self.home_report_id.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

/// One operation from the OpenAPI document returned by the discover endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApiOperation {
    pub method: String,
    pub path: String,
    pub operation_id: Option<String>,
    pub summary: Option<String>,
}

const HTTP_METHODS: [&str; 8] = [
    "get", "put", "post", "patch", "delete", "head", "options", "trace",
];

impl ApiOperation {
    /// Lists the operations under `paths`, in path order then method order.
    pub fn from_openapi(doc: &serde_json::Value) -> Vec<ApiOperation> {
        let Some(paths) = doc.get("paths").and_then(|p| p.as_object()) else {
            return Vec::new();
        };
        let mut ops = Vec::new();
        for (path, item) in paths {
            for method in HTTP_METHODS {
                let Some(op) = item.get(method).filter(|o| o.is_object()) else {
                    continue;
                };
                let text = |k: &str| op.get(k).and_then(|v| v.as_str()).map(str::to_string);
                ops.push(ApiOperation {
                    method: method.to_uppercase(),
                    path: path.clone(),
                    operation_id: text("operationId"),
                    summary: text("summary"),
                });
            }
        }
        ops.sort_by(|a, b| a.path.cmp(&b.path));
        ops
    }
}

impl TableRenderable for ApiOperation {
    fn headers() -> Vec<&'static str> {
        vec!["METHOD", "PATH", "OPERATION ID", "SUMMARY"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.method.clone(),
            self.path.clone(),
            self.operation_id.clone().unwrap_or_default(),
            self.summary.clone().unwrap_or_default(),
        ]
    }
}
