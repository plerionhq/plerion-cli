use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::assets};

fn empty_assets_response() -> String {
    serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 10, "total": 0 }
    })
    .to_string()
}

#[tokio::test]
async fn test_get_asset() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "id": "asset-123",
            "assetType": "AWS::S3::Bucket",
            "name": "my-bucket",
            "provider": "AWS",
            "region": "us-east-1",
            "riskScore": 8.5
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/assets/asset-123")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = assets::get_asset(&client, "asset-123", None).await.unwrap();
    assert_eq!(resp["data"]["id"], "asset-123");
    assert_eq!(resp["data"]["name"], "my-bucket");
}

#[tokio::test]
async fn test_get_asset_sbom() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "packages": [
                { "name": "openssl", "version": "3.0.1" }
            ]
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/assets/asset-123/sbom")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = assets::get_asset_sbom(&client, "asset-123").await.unwrap();
    assert!(resp["data"]["packages"].is_array());
}

#[tokio::test]
async fn test_list_assets_with_boolean_filters() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("isPubliclyExposed".to_string(), "true".to_string()),
            mockito::Matcher::UrlEncoded("isVulnerable".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_body(empty_assets_response())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = assets::ListAssetsParams {
        is_publicly_exposed: Some(true),
        is_vulnerable: Some(true),
        ..Default::default()
    };
    let _resp = assets::list_assets(&client, &params).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_assets_with_resource_type() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::UrlEncoded(
            "resourceTypes".to_string(),
            "AWS::EC2::Instance".to_string(),
        ))
        .with_status(200)
        .with_body(empty_assets_response())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = assets::ListAssetsParams {
        resource_types: Some("AWS::EC2::Instance".to_string()),
        ..Default::default()
    };
    let _resp = assets::list_assets(&client, &params).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_assets_error_404() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/assets")
        .with_status(404)
        .with_body(r#"{"message":"Not found"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result = assets::list_assets(&client, &assets::ListAssetsParams::default()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("404"));
}

#[tokio::test]
async fn test_list_assets_with_new_filters() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("executionIds".to_string(), "exec-1".to_string()),
            mockito::Matcher::UrlEncoded("secretsLevels".to_string(), "HIGH".to_string()),
            mockito::Matcher::UrlEncoded("firstObservedAtStart".to_string(), "2025-01-01T00:00:00Z".to_string()),
            mockito::Matcher::UrlEncoded("firstObservedAtEnd".to_string(), "2025-06-01T00:00:00Z".to_string()),
            mockito::Matcher::UrlEncoded("hasOverlyPermissivePrivileges".to_string(), "true".to_string()),
            mockito::Matcher::UrlEncoded("isExploitable".to_string(), "true".to_string()),
            mockito::Matcher::UrlEncoded("metadata".to_string(), "linux".to_string()),
        ]))
        .with_status(200)
        .with_body(empty_assets_response())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = assets::ListAssetsParams {
        execution_ids: Some("exec-1".to_string()),
        secrets_levels: Some("HIGH".to_string()),
        first_observed_at_start: Some("2025-01-01T00:00:00Z".to_string()),
        first_observed_at_end: Some("2025-06-01T00:00:00Z".to_string()),
        has_overly_permissive_privileges: Some(true),
        is_exploitable: Some(true),
        metadata: Some("linux".to_string()),
        ..Default::default()
    };
    let _resp = assets::list_assets(&client, &params).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_asset_with_include() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": { "id": "asset-123", "name": "my-bucket" }
    });
    let mock = server
        .mock("GET", "/v1/tenant/assets/asset-123")
        .match_query(mockito::Matcher::UrlEncoded("include".to_string(), "rawData".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = assets::get_asset(&client, "asset-123", Some("rawData")).await.unwrap();
    assert_eq!(resp["data"]["id"], "asset-123");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_assets_with_ids_filter() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::UrlEncoded("ids".to_string(), "a-1,a-2".to_string()))
        .with_status(200)
        .with_body(empty_assets_response())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = assets::ListAssetsParams {
        ids: Some("a-1,a-2".to_string()),
        ..Default::default()
    };
    let _resp = assets::list_assets(&client, &params).await.unwrap();
    mock.assert_async().await;
}
