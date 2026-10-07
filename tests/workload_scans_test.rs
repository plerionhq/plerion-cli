use mockito::{Matcher, Server};
use plerion::api::{
    client::PlerionClient,
    endpoints::workload_scans::{get_workload_scan, request_workload_scan},
    models::workload_scans::RequestWorkloadScanRequest,
};

const INTEGRATION: &str = "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3";

#[tokio::test]
async fn test_request_workload_scan_by_asset_id() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/tenant/workload/scans")
        .match_body(Matcher::Json(serde_json::json!({
            "integrationId": INTEGRATION,
            "assetId": "prn:assets:x:aws:ec2:instance:ap-southeast-2:i-1"
        })))
        .with_status(200)
        .with_body(r#"{"scanId":"scan-1","status":"PENDING","executionId":"exec-1"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let body = RequestWorkloadScanRequest {
        integration_id: INTEGRATION.into(),
        asset_id: Some("prn:assets:x:aws:ec2:instance:ap-southeast-2:i-1".into()),
        ..Default::default()
    };
    let resp = request_workload_scan(&client, &body).await.unwrap();
    assert_eq!(resp.scan_id.as_deref(), Some("scan-1"));
    assert_eq!(resp.status.as_deref(), Some("PENDING"));
    assert_eq!(resp.execution_id.as_deref(), Some("exec-1"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_request_workload_scan_by_resource() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/tenant/workload/scans")
        .match_body(Matcher::Json(serde_json::json!({
            "integrationId": INTEGRATION,
            "resourceType": "ec2:instance",
            "resourceRegion": "ap-southeast-2",
            "resourceId": "i-0123456789abcdef0"
        })))
        .with_status(200)
        .with_body(r#"{"scanId":"scan-2","status":"PENDING"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let body = RequestWorkloadScanRequest {
        integration_id: INTEGRATION.into(),
        resource_type: Some("ec2:instance".into()),
        resource_region: Some("ap-southeast-2".into()),
        resource_id: Some("i-0123456789abcdef0".into()),
        ..Default::default()
    };
    let resp = request_workload_scan(&client, &body).await.unwrap();
    assert_eq!(resp.scan_id.as_deref(), Some("scan-2"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_request_workload_scan_surfaces_422() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("POST", "/v1/tenant/workload/scans")
        .with_status(422)
        .with_body(r#"{"errors":[{"code":"InvalidResource","message":"Resource not supported for scanning"}]}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let body = RequestWorkloadScanRequest {
        integration_id: INTEGRATION.into(),
        asset_id: Some("a".into()),
        ..Default::default()
    };
    let err = request_workload_scan(&client, &body)
        .await
        .unwrap_err()
        .to_string();
    assert!(err.contains("422"), "{err}");
    assert!(err.contains("InvalidResource"), "{err}");
}

#[tokio::test]
async fn test_get_workload_scan() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "id": "scan-1",
        "organizationId": "o-1",
        "tenantId": "t-1",
        "integrationId": INTEGRATION,
        "assetId": "prn:assets:x",
        "applianceId": "app-1",
        "executionId": "exec-1",
        "type": "WORKLOAD",
        "status": "COMPLETED",
        "statusMessage": "done",
        "startedAt": "2026-08-01T00:00:00Z",
        "completedAt": "2026-08-01T00:10:00Z",
        "updatedAt": "2026-08-01T00:10:00Z"
    });
    let mock = server
        .mock("GET", "/v1/tenant/workload/scans/scan-1")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let scan = get_workload_scan(&client, "scan-1").await.unwrap();
    assert_eq!(scan.status.as_deref(), Some("COMPLETED"));
    assert_eq!(scan.scan_type.as_deref(), Some("WORKLOAD"));
    assert_eq!(scan.appliance_id.as_deref(), Some("app-1"));
    assert_eq!(scan.error_code, None);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_workload_scan_escapes_the_id() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/workload/scans/a%2Fb%3Fc")
        .with_status(404)
        .with_body(r#"{"errors":[{"code":"NotFound","message":"Scan record not found"}]}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let err = get_workload_scan(&client, "a/b?c")
        .await
        .unwrap_err()
        .to_string();
    assert!(err.contains("404"), "{err}");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_workload_scan_rejects_a_dot_segment() {
    let client = PlerionClient::with_base_url("http://127.0.0.1:1", "k").unwrap();
    let err = get_workload_scan(&client, "..")
        .await
        .unwrap_err()
        .to_string();
    assert_eq!(err, "'..' is not a valid scan ID");
}
