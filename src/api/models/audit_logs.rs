use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuditLog {
    pub id: Option<String>,
    pub operation: Option<String>,
    pub operation_time: Option<String>,
    pub operator_user_id: Option<String>,
    pub operator_email: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub location: Option<AuditLocation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuditLocation {
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogsResponse {
    pub data: Vec<AuditLog>,
    pub meta: PaginationMeta,
}

impl TableRenderable for AuditLog {
    fn headers() -> Vec<&'static str> {
        vec!["OPERATION", "EMAIL", "IP", "COUNTRY", "TIME"]
    }

    fn row(&self) -> Vec<String> {
        let country = self.location.as_ref()
            .and_then(|l| l.country.clone())
            .unwrap_or_default();
        vec![
            self.operation.clone().unwrap_or_default(),
            self.operator_email.clone().unwrap_or_default(),
            self.ip.clone().unwrap_or_default(),
            country,
            self.operation_time.clone().unwrap_or_default(),
        ]
    }
}
