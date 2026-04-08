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
