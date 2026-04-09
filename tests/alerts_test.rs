use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::alerts::{list_alerts, ListAlertsParams}};

#[tokio::test]
async fn test_list_alerts_deserializes_correctly() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "prn:alerts:abc123",
                "status": "OPEN",
                "title": "Asset with risk score ≥ 9",
                "alertType": "ASSET",
                "riskScore": 9.5,
                "flagged": false,
                "acknowledged": false,
                "createdAt": "2023-02-04T06:07:09.092Z"
            }
        ],
        "meta": { "cursor": null, "perPage": 50 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/alerts")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_alerts(&client, &ListAlertsParams::default()).await.unwrap();

    assert_eq!(resp.data.len(), 1);
    let a = &resp.data[0];
    assert_eq!(a.status.as_deref(), Some("OPEN"));
    assert_eq!(a.alert_type.as_deref(), Some("ASSET"));
    assert_eq!(a.risk_score, Some(9.5));
}
