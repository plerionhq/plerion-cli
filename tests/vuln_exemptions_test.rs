use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::vulnerabilities};

#[tokio::test]
async fn test_list_exemptions() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "ex-1",
                "name": "Accepted risk",
                "reason": "ACCEPTED_RISK",
                "createdAt": "2025-01-01T00:00:00Z"
            }
        ]
    });
    let _mock = server
        .mock("GET", "/v1/tenant/profiles/prof-1/vulnerability/exemptions")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = vulnerabilities::list_exemptions(&client, "prof-1", None, None).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].reason.as_deref(), Some("ACCEPTED_RISK"));
}

#[tokio::test]
async fn test_list_exemptions_with_pagination_params() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "ex-1", "reason": "ACCEPTED_RISK" }],
        "meta": { "hasNext": true, "nextCursor": "cursor-abc", "total": 5 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/profiles/prof-1/vulnerability/exemptions")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("limit".to_string(), "10".to_string()),
            mockito::Matcher::UrlEncoded("cursor".to_string(), "prev-cursor".to_string()),
        ]))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = vulnerabilities::list_exemptions(&client, "prof-1", Some(10), Some("prev-cursor")).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.meta.has_next, Some(true));
    assert_eq!(resp.meta.next_cursor.as_deref(), Some("cursor-abc"));
    assert_eq!(resp.meta.total, Some(5));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_exemption() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "id": "ex-1", "name": "Test" } });
    let _mock = server
        .mock("GET", "/v1/tenant/profiles/prof-1/vulnerability/exemptions/ex-1")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = vulnerabilities::get_exemption(&client, "prof-1", "ex-1").await.unwrap();
    assert_eq!(resp["data"]["id"], "ex-1");
}

#[tokio::test]
async fn test_create_exemption() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "id": "ex-new" } });
    let mock = server
        .mock("POST", "/v1/tenant/profiles/prof-1/vulnerability/exemptions")
        .with_status(201)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let req_body = serde_json::json!({ "name": "New exemption", "reason": "NOT_IN_USE" });
    let resp = vulnerabilities::create_exemption(&client, "prof-1", req_body).await.unwrap();
    assert_eq!(resp["data"]["id"], "ex-new");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_exemption() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "id": "ex-1", "name": "Updated" } });
    let mock = server
        .mock("PATCH", "/v1/tenant/profiles/prof-1/vulnerability/exemptions/ex-1")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let req_body = serde_json::json!({ "name": "Updated" });
    let resp = vulnerabilities::update_exemption(&client, "prof-1", "ex-1", req_body).await.unwrap();
    assert_eq!(resp["data"]["name"], "Updated");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_exemption() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/tenant/profiles/prof-1/vulnerability/exemptions/ex-1")
        .with_status(204)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    vulnerabilities::delete_exemption(&client, "prof-1", "ex-1").await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_exemption_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("DELETE", "/v1/tenant/profiles/prof-1/vulnerability/exemptions/bad")
        .with_status(404)
        .with_body(r#"{"message":"Not found"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result = vulnerabilities::delete_exemption(&client, "prof-1", "bad").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_exemption_with_audit_note() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "id": "ex-new" } });
    let mock = server
        .mock("POST", "/v1/tenant/profiles/prof-1/vulnerability/exemptions")
        .match_query(mockito::Matcher::Any)
        .with_status(201)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let req_body = serde_json::json!({
        "name": "test",
        "reason": "ACCEPTED_RISK",
        "auditNote": "Reviewed by security team"
    });
    let resp = vulnerabilities::create_exemption(&client, "prof-1", req_body).await.unwrap();
    assert_eq!(resp["data"]["id"], "ex-new");
    mock.assert_async().await;
}
