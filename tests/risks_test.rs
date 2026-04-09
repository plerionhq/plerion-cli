use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::risks::{list_risks, ListRisksParams}};

#[tokio::test]
async fn test_list_risks_deserializes_correctly() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "risk-123",
                "riskTypeId": "PLERION-RISK-1",
                "score": 9.5,
                "severityLevel": "CRITICAL",
                "lifecycleState": "OPEN",
                "discoveredAt": "2025-02-18T08:00:00Z"
            }
        ],
        "meta": { "perPage": 50, "total": 1, "cursor": null }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/risks")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_risks(&client, &ListRisksParams::default()).await.unwrap();

    assert_eq!(resp.data.len(), 1);
    let r = &resp.data[0];
    assert_eq!(r.severity_level.as_deref(), Some("CRITICAL"));
    assert_eq!(r.score, Some(9.5));
}

#[tokio::test]
async fn test_list_risks_with_fields() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/risks")
        .match_query(mockito::Matcher::UrlEncoded(
            "fields".to_string(),
            "id,name,severityLevel".to_string(),
        ))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "cursor": null, "perPage": 50, "total": 0, "hasNextPage": false }
        }).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListRisksParams {
        fields: Some("id,name,severityLevel".to_string()),
        ..Default::default()
    };
    let resp = list_risks(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_risks_with_id_and_date_filters() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/risks")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("ids".to_string(), "r-1".to_string()),
            mockito::Matcher::UrlEncoded("riskTypeIds".to_string(), "rt-1".to_string()),
            mockito::Matcher::UrlEncoded("primaryAssetIds".to_string(), "a-1".to_string()),
            mockito::Matcher::UrlEncoded("discoveredAtStart".to_string(), "2025-01-01T00:00:00Z".to_string()),
            mockito::Matcher::UrlEncoded("discoveredAtEnd".to_string(), "2025-06-01T00:00:00Z".to_string()),
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
    let params = ListRisksParams {
        ids: Some("r-1".to_string()),
        risk_type_ids: Some("rt-1".to_string()),
        primary_asset_ids: Some("a-1".to_string()),
        discovered_at_start: Some("2025-01-01T00:00:00Z".to_string()),
        discovered_at_end: Some("2025-06-01T00:00:00Z".to_string()),
        ..Default::default()
    };
    let resp = list_risks(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}
