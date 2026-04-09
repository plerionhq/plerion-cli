use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::tenant};

#[tokio::test]
async fn test_get_tenant() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "tenantId": "tid-123",
            "organizationId": "oid-456",
            "name": "Test Tenant",
            "createdAt": "2023-01-01T00:00:00Z",
            "updatedAt": "2023-06-01T00:00:00Z",
            "riskScore": 7.5
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::get_tenant(&client).await.unwrap();
    assert_eq!(resp.data.tenant_id, "tid-123");
    assert_eq!(resp.data.name, "Test Tenant");
    assert_eq!(resp.data.risk_score, Some(7.5));
}

#[tokio::test]
async fn test_get_tenant_usage() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": { "assets": 100, "integrations": 5 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/usage")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::get_tenant_usage(&client, None).await.unwrap();
    assert_eq!(resp.data["assets"], 100);
}

#[tokio::test]
async fn test_get_tenant_usage_with_date() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": { "assets": 100, "integrations": 5 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/usage")
        .match_query(mockito::Matcher::UrlEncoded("date".to_string(), "2025-03-01".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::get_tenant_usage(&client, Some("2025-03-01")).await.unwrap();
    assert_eq!(resp.data["assets"], 100);
    mock.assert_async().await;
}
