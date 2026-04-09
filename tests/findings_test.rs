use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::findings::{list_findings, ListFindingsParams}};

fn mock_finding_response() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {
                "id": "prn:findings:abc123",
                "detectionId": "PLERION-AWS-16",
                "status": "FAILED",
                "severityLevel": "CRITICAL",
                "provider": "AWS",
                "resourceType": "AWS::IAM::Policy",
                "region": "us-east-1",
                "service": "IAM",
                "firstObservedAt": "2023-02-04T06:02:40.594Z",
                "isExempted": false
            }
        ],
        "meta": {
            "cursor": null,
            "perPage": 50,
            "total": 1,
            "page": 1,
            "hasNextPage": false,
            "hasPreviousPage": false
        }
    })
}

#[tokio::test]
async fn test_list_findings_deserializes_correctly() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/findings")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(mock_finding_response().to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListFindingsParams::default();
    let resp = list_findings(&client, &params).await.unwrap();

    assert_eq!(resp.data.len(), 1);
    let f = &resp.data[0];
    assert_eq!(f.detection_id.as_deref(), Some("PLERION-AWS-16"));
    assert_eq!(f.status.as_deref(), Some("FAILED"));
    assert_eq!(f.severity_level.as_deref(), Some("CRITICAL"));
    assert_eq!(f.provider.as_deref(), Some("AWS"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_findings_with_severity_filter() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::UrlEncoded(
            "severityLevels".to_string(),
            "CRITICAL".to_string(),
        ))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(mock_finding_response().to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListFindingsParams {
        severity_levels: Some("CRITICAL".to_string()),
        ..Default::default()
    };
    let resp = list_findings(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_findings_api_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/findings")
        .with_status(401)
        .with_body(r#"{"message":"Unauthorized"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "bad_key").unwrap();
    let result = list_findings(&client, &ListFindingsParams::default()).await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("401"));
}

#[tokio::test]
async fn test_list_findings_pagination_meta() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": {
            "cursor": "next_cursor_token",
            "perPage": 50,
            "hasNextPage": true,
            "hasPreviousPage": false
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/findings")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_findings(&client, &ListFindingsParams::default()).await.unwrap();

    assert_eq!(resp.meta.cursor.as_deref(), Some("next_cursor_token"));
    assert_eq!(resp.meta.has_next_page, Some(true));
}

#[tokio::test]
async fn test_list_findings_with_ids_and_asset_id() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("ids".to_string(), "f-1,f-2".to_string()),
            mockito::Matcher::UrlEncoded("assetIds".to_string(), "a-1".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "cursor": null, "perPage": 50, "total": 0, "hasNextPage": false }
        }).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListFindingsParams {
        ids: Some("f-1,f-2".to_string()),
        asset_ids: Some("a-1".to_string()),
        ..Default::default()
    };
    let resp = list_findings(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}
