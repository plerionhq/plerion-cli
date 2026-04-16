use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetGroup {
    pub asset_group_id: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub total_assets: Option<u32>,
    pub risk_score: Option<f64>,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetGroupsResponse {
    pub data: Vec<AssetGroup>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetGroupResponse {
    pub data: AssetGroup,
}

impl TableRenderable for AssetGroup {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "NAME",
            "STATUS",
            "ASSETS",
            "RISK SCORE",
            "TENANT ID",
            "ORG ID",
            "CREATED AT",
            "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.asset_group_id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.total_assets.map(|n| n.to_string()).unwrap_or_default(),
            self.risk_score.map(|s| format!("{s:.2}")).unwrap_or_default(),
            self.tenant_id.clone().unwrap_or_default(),
            self.organization_id.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetGroupRequest {
    pub name: String,
    pub rules: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAssetGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<serde_json::Value>,
}
