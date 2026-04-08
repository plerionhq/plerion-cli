use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::iac};

#[tokio::test]
async fn test_list_iac_scans() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "scanId": "scan-1",
                "artifactName": "infra.zip",
                "status": "completed",
                "createdAt": "2025-01-01T00:00:00Z"
            }
        ]
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = iac::list_iac_scans(&client).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].scan_id.as_deref(), Some("scan-1"));
    assert_eq!(resp.data[0].artifact_name.as_deref(), Some("infra.zip"));
}

#[tokio::test]
async fn test_get_iac_findings() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "f-1",
                "severityLevel": "HIGH",
                "resourceType": "AWS::S3::Bucket",
                "message": "Bucket is public",
                "filePath": "main.tf",
                "lineNumber": 42
            }
        ]
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = iac::get_iac_findings(&client, "scan-1").await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].severity_level.as_deref(), Some("HIGH"));
    assert_eq!(resp.data[0].line_number, Some(42));
}

#[tokio::test]
async fn test_get_iac_vulnerabilities() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": [{ "id": "v-1" }] });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = iac::get_iac_vulnerabilities(&client, "scan-1").await.unwrap();
    assert!(resp["data"].is_array());
}

#[tokio::test]
async fn test_upload_iac_scan() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": "ok",
        "meta": {
            "scanId": "scan-new",
            "artifactName": "test.zip",
            "tenantId": "t-1",
            "organizationId": "o-1"
        }
    });
    let mock = server
        .mock("POST", "/v1/tenant/shiftleft/iac/scan")
        .match_query(mockito::Matcher::UrlEncoded("artifactName".to_string(), "test.zip".to_string()))
        .with_status(202)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = iac::upload_iac_scan(&client, "test.zip", bytes::Bytes::from_static(b"PK\x03\x04"))
        .await
        .unwrap();
    assert_eq!(resp.meta.scan_id.as_deref(), Some("scan-new"));
    mock.assert_async().await;
}
