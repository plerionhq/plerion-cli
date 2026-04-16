use mockito::Server;
use plerion::api::{
    client::PlerionClient,
    endpoints::asset_groups,
    models::asset_groups::{CreateAssetGroupRequest, UpdateAssetGroupRequest},
};

fn mock_asset_group() -> serde_json::Value {
    serde_json::json!({
        "assetGroupId": "ag-123",
        "name": "Production",
        "status": "completed",
        "totalAssets": 42,
        "riskScore": 7.5,
        "createdAt": "2025-01-07T13:37:23.388Z",
        "updatedAt": "2025-01-07T13:37:46.308Z"
    })
}

#[tokio::test]
async fn test_list_asset_groups() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [mock_asset_group()],
        "meta": { "cursor": null, "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/asset-groups")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = asset_groups::list_asset_groups(&client, None, None, None, false).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].name.as_deref(), Some("Production"));
    assert_eq!(resp.data[0].total_assets, Some(42));
}

#[tokio::test]
async fn test_list_asset_groups_with_name_filter() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": [], "meta": {} });
    let mock = server
        .mock("GET", "/v1/tenant/asset-groups")
        .match_query(mockito::Matcher::UrlEncoded("name".to_string(), "Prod".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    asset_groups::list_asset_groups(&client, Some("Prod"), None, None, false).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_asset_group() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": mock_asset_group() });
    let _mock = server
        .mock("GET", "/v1/tenant/asset-groups/ag-123")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = asset_groups::get_asset_group(&client, "ag-123").await.unwrap();
    assert_eq!(resp.data.asset_group_id.as_deref(), Some("ag-123"));
}

#[tokio::test]
async fn test_create_asset_group() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": mock_asset_group() });
    let mock = server
        .mock("POST", "/v1/tenant/asset-groups")
        .with_status(201)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = asset_groups::create_asset_group(
        &client,
        CreateAssetGroupRequest {
            name: "Production".to_string(),
            rules: serde_json::json!([]),
        },
    )
    .await
    .unwrap();
    assert_eq!(resp.data.name.as_deref(), Some("Production"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_asset_group() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": mock_asset_group() });
    let mock = server
        .mock("PATCH", "/v1/tenant/asset-groups/ag-123")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    asset_groups::update_asset_group(
        &client,
        "ag-123",
        UpdateAssetGroupRequest {
            name: Some("New Name".to_string()),
            rules: None,
        },
    )
    .await
    .unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_asset_group() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/tenant/asset-groups/ag-123")
        .with_status(204)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    asset_groups::delete_asset_group(&client, "ag-123").await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_asset_group_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("DELETE", "/v1/tenant/asset-groups/bad-id")
        .with_status(404)
        .with_body(r#"{"message": "Not found"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result = asset_groups::delete_asset_group(&client, "bad-id").await;
    assert!(result.is_err());
}
