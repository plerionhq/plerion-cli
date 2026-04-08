use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::integrations};

#[tokio::test]
async fn test_list_integrations() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "integrationId": "int-123",
                "name": "AWS Prod",
                "provider": "AWS",
                "type": "AWSAccount",
                "status": "Active",
                "riskScore": 8.19,
                "awsAccountId": "123456789012",
                "createdAt": "2023-02-04T06:07:09.092Z"
            }
        ],
        "meta": { "perPage": 10, "cursor": "abc", "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::UrlEncoded("perPage".to_string(), "10".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = integrations::list_integrations(&client, Some(10), None, false).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].integration_id.as_deref(), Some("int-123"));
    assert_eq!(resp.data[0].provider.as_deref(), Some("AWS"));
    assert_eq!(resp.data[0].aws_account_id.as_deref(), Some("123456789012"));
}

#[tokio::test]
async fn test_list_integrations_with_include_total() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "perPage": 10, "total": 42 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("includeTotal".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = integrations::list_integrations(&client, Some(10), None, true).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_integrations_with_cursor() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "perPage": 10 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::UrlEncoded("cursor".to_string(), "page2".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let _resp = integrations::list_integrations(&client, Some(10), Some("page2"), false).await.unwrap();
    mock.assert_async().await;
}
