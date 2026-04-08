use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::audit_logs};

#[tokio::test]
async fn test_list_audit_logs() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "log-1",
                "operation": "UserLogin",
                "operationTime": "2025-01-01T10:00:00Z",
                "operatorUserId": "user-1",
                "operatorEmail": "admin@example.com",
                "ip": "1.2.3.4",
                "userAgent": "Mozilla/5.0",
                "location": {
                    "country": "Australia",
                    "city": "Sydney",
                    "region": "NSW"
                }
            }
        ],
        "meta": { "cursor": null, "perPage": 50 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/audit-logs")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = audit_logs::list_audit_logs(&client, None, None, None, None, None, None)
        .await
        .unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].operation.as_deref(), Some("UserLogin"));
    assert_eq!(resp.data[0].operator_email.as_deref(), Some("admin@example.com"));
    assert_eq!(
        resp.data[0].location.as_ref().unwrap().country.as_deref(),
        Some("Australia")
    );
}

#[tokio::test]
async fn test_list_audit_logs_with_filters() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": [], "meta": { "cursor": null, "perPage": 50 } });
    let mock = server
        .mock("GET", "/v1/tenant/audit-logs")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("operation".to_string(), "UserLogin".to_string()),
            mockito::Matcher::UrlEncoded("userId".to_string(), "user-1".to_string()),
        ]))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    audit_logs::list_audit_logs(&client, None, None, Some("user-1"), Some("UserLogin"), None, None)
        .await
        .unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_audit_logs_with_time_range() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": [], "meta": { "cursor": null, "perPage": 50 } });
    let mock = server
        .mock("GET", "/v1/tenant/audit-logs")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("operationTimeStart".to_string(), "2025-01-01T00:00:00Z".to_string()),
            mockito::Matcher::UrlEncoded("operationTimeEnd".to_string(), "2025-01-31T23:59:59Z".to_string()),
        ]))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    audit_logs::list_audit_logs(
        &client,
        Some("2025-01-01T00:00:00Z"),
        Some("2025-01-31T23:59:59Z"),
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    mock.assert_async().await;
}
