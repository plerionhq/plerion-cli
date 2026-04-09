use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::iac};

#[tokio::test]
async fn test_list_iac_scans() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "scan-1",
                "artifactName": "infra.zip",
                "status": "SUCCESS",
                "createdAt": "2025-01-01T00:00:00Z",
                "updatedAt": "2025-01-01T00:01:00Z",
                "tenantId": "t-1",
                "organizationId": "o-1",
                "summary": {
                    "totalFindings": 10,
                    "totalFailedFindings": 3,
                    "totalPassedFindings": 7,
                    "totalVulnerabilities": 2
                },
                "types": ["terraform"]
            }
        ],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacScansParams::default();
    let resp = iac::list_iac_scans(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].id.as_deref(), Some("scan-1"));
    assert_eq!(resp.data[0].artifact_name.as_deref(), Some("infra.zip"));
    assert_eq!(resp.data[0].status.as_deref(), Some("SUCCESS"));
    assert_eq!(resp.data[0].types, vec!["terraform"]);
    let summary = resp.data[0].summary.as_ref().unwrap();
    assert_eq!(summary.total_findings, Some(10));
    assert_eq!(summary.total_failed_findings, Some(3));
}

#[tokio::test]
async fn test_get_iac_findings() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "id": "f-1",
                "scanId": "scan-1",
                "detectionId": "PLERION-K8S-108",
                "detectionTitle": "Ensure privileged is false",
                "type": "terraform",
                "result": "FAILED",
                "severityLevel": "HIGH",
                "file": "main.tf",
                "repositoryPath": "/repo/main.tf",
                "lineRange": [10, 42],
                "resource": "aws_s3_bucket.data",
                "dashboardURL": "https://example.com/finding/f-1"
            }
        ],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacFindingsParams::default();
    let resp = iac::get_iac_findings(&client, "scan-1", &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].severity_level.as_deref(), Some("HIGH"));
    assert_eq!(resp.data[0].file.as_deref(), Some("main.tf"));
    assert_eq!(resp.data[0].line_range.as_ref().unwrap(), &vec![10, 42]);
    assert_eq!(resp.data[0].detection_id.as_deref(), Some("PLERION-K8S-108"));
    assert_eq!(resp.data[0].resource.as_deref(), Some("aws_s3_bucket.data"));
}

#[tokio::test]
async fn test_get_iac_vulnerabilities() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{
            "id": "v-1",
            "vulnerabilityId": "CVE-2023-12345",
            "title": "Sample Vulnerability",
            "severityLevel": "HIGH",
            "hasKev": true,
            "hasExploit": false,
            "packages": [{"name": "openssl", "type": "npm", "installedVersion": "1.0.0", "fixedVersion": "1.0.1"}]
        }],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacVulnerabilitiesParams::default();
    let resp = iac::get_iac_vulnerabilities(&client, "scan-1", &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].vulnerability_id.as_deref(), Some("CVE-2023-12345"));
    assert_eq!(resp.data[0].has_kev, Some(true));
}

#[tokio::test]
async fn test_list_iac_scans_with_ids() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .match_query(mockito::Matcher::UrlEncoded("ids".to_string(), "scan-1,scan-2".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacScansParams {
        ids: Some("scan-1,scan-2".to_string()),
        ..Default::default()
    };
    let resp = iac::list_iac_scans(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_iac_findings_with_ids() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::UrlEncoded("ids".to_string(), "f-1,f-2".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacFindingsParams {
        ids: Some("f-1,f-2".to_string()),
        ..Default::default()
    };
    let resp = iac::get_iac_findings(&client, "scan-1", &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_iac_vulnerabilities_with_ids() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .match_query(mockito::Matcher::UrlEncoded("ids".to_string(), "v-1,v-2".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacVulnerabilitiesParams {
        ids: Some("v-1,v-2".to_string()),
        ..Default::default()
    };
    let resp = iac::get_iac_vulnerabilities(&client, "scan-1", &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_iac_scans_with_filters() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("statuses".to_string(), "SUCCESS".to_string()),
            mockito::Matcher::UrlEncoded("artifactNames".to_string(), "infra.zip".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "10".to_string()),
        ]))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacScansParams {
        statuses: Some("SUCCESS".to_string()),
        artifact_names: Some("infra.zip".to_string()),
        per_page: Some(10),
        ..Default::default()
    };
    let resp = iac::list_iac_scans(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_iac_findings_with_filters() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "f-1", "result": "FAILED", "severityLevel": "CRITICAL" }],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("results".to_string(), "FAILED".to_string()),
            mockito::Matcher::UrlEncoded("severityLevels".to_string(), "CRITICAL".to_string()),
        ]))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacFindingsParams {
        results: Some("FAILED".to_string()),
        severity_levels: Some("CRITICAL".to_string()),
        ..Default::default()
    };
    let resp = iac::get_iac_findings(&client, "scan-1", &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].result.as_deref(), Some("FAILED"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_iac_vulnerabilities_with_filters() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "v-1", "vulnerabilityId": "CVE-2024-1234", "severityLevel": "HIGH" }],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("severityLevels".to_string(), "HIGH".to_string()),
            mockito::Matcher::UrlEncoded("hasKevs".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacVulnerabilitiesParams {
        severity_levels: Some("HIGH".to_string()),
        has_kevs: Some(true),
        ..Default::default()
    };
    let resp = iac::get_iac_vulnerabilities(&client, "scan-1", &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_iac_findings_string_meta_values() {
    // The IaC API returns page/perPage as strings ("1", "1000") instead of integers.
    // This test ensures deserialization handles both forms.
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "f-1", "result": "FAILED" }],
        "meta": { "page": "1", "perPage": "1000", "total": 11, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let params = iac::ListIacFindingsParams::default();
    let resp = iac::get_iac_findings(&client, "scan-1", &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.meta.page, Some(1));
    assert_eq!(resp.meta.per_page, Some(1000));
    assert_eq!(resp.meta.total, Some(11));
    assert_eq!(resp.meta.has_next_page, Some(false));
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
