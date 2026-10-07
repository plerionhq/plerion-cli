use serde::{Deserialize, Serialize};

use crate::api::models::findings::PaginationMeta;
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomReportSummary {
    pub id: Option<String>,
    pub name: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomReportsResponse {
    pub data: Vec<CustomReportSummary>,
    pub meta: PaginationMeta,
}

impl TableRenderable for CustomReportSummary {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "CREATED AT", "UPDATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}
