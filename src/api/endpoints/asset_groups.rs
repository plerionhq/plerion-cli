use crate::api::client::PlerionClient;
use crate::api::models::asset_groups::{
    AssetGroupResponse, AssetGroupsResponse, CreateAssetGroupRequest, UpdateAssetGroupRequest,
};
use crate::error::PlerionError;

pub async fn list_asset_groups(
    client: &PlerionClient,
    name: Option<&str>,
    per_page: Option<u32>,
    cursor: Option<&str>,
    include_total: bool,
) -> Result<AssetGroupsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/asset-groups");
    if let Some(v) = name { req = req.query(&[("name", v)]); }
    if let Some(v) = per_page { req = req.query(&[("perPage", v)]); }
    if let Some(v) = cursor { req = req.query(&[("cursor", v)]); }
    if include_total { req = req.query(&[("includeTotal", true)]); }
    client.execute(req).await
}

pub async fn get_asset_group(
    client: &PlerionClient,
    id: &str,
) -> Result<AssetGroupResponse, PlerionError> {
    client.execute(client.get(&format!("/v1/tenant/asset-groups/{id}"))).await
}

pub async fn create_asset_group(
    client: &PlerionClient,
    body: CreateAssetGroupRequest,
) -> Result<AssetGroupResponse, PlerionError> {
    client
        .execute(client.post("/v1/tenant/asset-groups").json(&body))
        .await
}

pub async fn update_asset_group(
    client: &PlerionClient,
    id: &str,
    body: UpdateAssetGroupRequest,
) -> Result<AssetGroupResponse, PlerionError> {
    client
        .execute(client.patch(&format!("/v1/tenant/asset-groups/{id}")).json(&body))
        .await
}

pub async fn delete_asset_group(client: &PlerionClient, id: &str) -> Result<(), PlerionError> {
    let req = client.delete(&format!("/v1/tenant/asset-groups/{id}"));
    let resp = req.send().await?;
    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status().as_u16();
        let message = resp.text().await.unwrap_or_default();
        Err(PlerionError::ApiError { status, message })
    }
}
