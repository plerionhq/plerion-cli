use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::alerts::{list_alerts, ListAlertsParams}};
use plerion::cli::normalize_providers;

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

#[tokio::test]
async fn test_list_alerts_with_all_filters() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/alerts")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("ids".to_string(), "alert-1".to_string()),
            mockito::Matcher::UrlEncoded("workflowIds".to_string(), "wf-1".to_string()),
            mockito::Matcher::UrlEncoded("assetGroupIds".to_string(), "ag-1".to_string()),
            mockito::Matcher::UrlEncoded("resourceTypes".to_string(), "AWS::EC2::Instance".to_string()),
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
    let params = ListAlertsParams {
        ids: Some("alert-1".to_string()),
        workflow_ids: Some("wf-1".to_string()),
        asset_group_ids: Some("ag-1".to_string()),
        resource_types: Some("AWS::EC2::Instance".to_string()),
        ..Default::default()
    };
    let resp = list_alerts(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

// --- riskScore string deserialization ---

#[tokio::test]
async fn test_alert_risk_score_as_string() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "prn:alerts:str-score",
                "status": "OPEN",
                "title": "String risk score",
                "riskScore": "4.4400"
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
    assert!((a.risk_score.unwrap() - 4.44).abs() < 0.001);
}

#[tokio::test]
async fn test_alert_risk_score_as_number() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "prn:alerts:num-score",
                "status": "OPEN",
                "riskScore": 9.5
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

    assert_eq!(resp.data[0].risk_score, Some(9.5));
}

#[tokio::test]
async fn test_alert_risk_score_null() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "prn:alerts:null-score",
                "status": "OPEN",
                "riskScore": null
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

    assert_eq!(resp.data[0].risk_score, None);
}

#[tokio::test]
async fn test_alert_risk_score_empty_string() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "prn:alerts:empty-score",
                "status": "OPEN",
                "riskScore": ""
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

    assert_eq!(resp.data[0].risk_score, None);
}

// --- normalize_providers ---

#[test]
fn test_normalize_providers_lowercase() {
    assert_eq!(
        normalize_providers(Some("aws".to_string())),
        Some("AWS".to_string())
    );
}

#[test]
fn test_normalize_providers_mixed_case() {
    assert_eq!(
        normalize_providers(Some("AZURE".to_string())),
        Some("Azure".to_string())
    );
    assert_eq!(
        normalize_providers(Some("Gcp".to_string())),
        Some("GCP".to_string())
    );
}

#[test]
fn test_normalize_providers_multiple() {
    assert_eq!(
        normalize_providers(Some("aws,azure,gcp".to_string())),
        Some("AWS,Azure,GCP".to_string())
    );
}

#[test]
fn test_normalize_providers_already_correct() {
    assert_eq!(
        normalize_providers(Some("AWS".to_string())),
        Some("AWS".to_string())
    );
    assert_eq!(
        normalize_providers(Some("Kubernetes".to_string())),
        Some("Kubernetes".to_string())
    );
}

#[test]
fn test_normalize_providers_unknown_passthrough() {
    assert_eq!(
        normalize_providers(Some("SomeOther".to_string())),
        Some("SomeOther".to_string())
    );
}

#[test]
fn test_normalize_providers_none() {
    assert_eq!(normalize_providers(None), None);
}

#[test]
fn test_normalize_providers_with_spaces() {
    assert_eq!(
        normalize_providers(Some(" aws , azure ".to_string())),
        Some("AWS,Azure".to_string())
    );
}

#[test]
fn test_normalize_providers_casm_and_kubernetes() {
    assert_eq!(
        normalize_providers(Some("casm,kubernetes".to_string())),
        Some("CASM,Kubernetes".to_string())
    );
}
