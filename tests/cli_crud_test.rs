use mockito::Server;
use std::process::Command;

fn run_plerion(args: &[&str], api_key: &str, endpoint_url: &str) -> std::process::Output {
    let binary = env!("CARGO_BIN_EXE_plerion");
    Command::new(binary)
        .args(args)
        .env("PLERION_API_KEY", api_key)
        .env("PLERION_ENDPOINT_URL", endpoint_url)
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to execute plerion binary")
}

// --- asset-groups CRUD ---

#[tokio::test]
async fn test_cli_asset_groups_create() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "assetGroupId": "ag-new", "name": "NewGroup" } });
    let _mock = server
        .mock("POST", "/v1/tenant/asset-groups")
        .with_status(201)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["asset-groups", "create", "--name", "NewGroup", "--rules", "[]", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("NewGroup"));
}

#[tokio::test]
async fn test_cli_asset_groups_update() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "assetGroupId": "ag-1", "name": "Updated" } });
    let _mock = server
        .mock("PATCH", "/v1/tenant/asset-groups/ag-1")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["asset-groups", "update", "--id", "ag-1", "--name", "Updated", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("Updated"));
}

#[tokio::test]
async fn test_cli_asset_groups_delete() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("DELETE", "/v1/tenant/asset-groups/ag-1")
        .with_status(204)
        .create_async()
        .await;

    let output = run_plerion(
        &["asset-groups", "delete", "--id", "ag-1"],
        "key", &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

// --- vulnerability exemptions CRUD ---

#[tokio::test]
async fn test_cli_vuln_exemptions_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "ex-1", "name": "Test", "reason": "ACCEPTED_RISK" }]
    });
    let _mock = server
        .mock("GET", "/v1/tenant/profiles/prof-1/vulnerability/exemptions")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "exemptions", "list", "--profile-id", "prof-1", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("ACCEPTED_RISK"));
}

#[tokio::test]
async fn test_cli_vuln_exemptions_get() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "id": "ex-1", "name": "MyExemption" } });
    let _mock = server
        .mock("GET", "/v1/tenant/profiles/prof-1/vulnerability/exemptions/ex-1")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "exemptions", "get", "--profile-id", "prof-1", "--id", "ex-1", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("MyExemption"));
}

#[tokio::test]
async fn test_cli_vuln_exemptions_create() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "id": "ex-new" } });
    let _mock = server
        .mock("POST", "/v1/tenant/profiles/prof-1/vulnerability/exemptions")
        .with_status(201)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "exemptions", "create", "--profile-id", "prof-1",
          "--name", "New Exemption", "--reason", "NOT_IN_USE", "--conditions", "{\"vulnerabilityIds\":[\"CVE-2024-0001\"]}", "--audit-note", "Reviewed", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("ex-new"));
}

#[tokio::test]
async fn test_cli_vuln_exemptions_delete() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("DELETE", "/v1/tenant/profiles/prof-1/vulnerability/exemptions/ex-1")
        .with_status(204)
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "exemptions", "delete", "--profile-id", "prof-1", "--id", "ex-1"],
        "key", &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

// --- compliance report/download ---

#[tokio::test]
async fn test_cli_compliance_request_report() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "status": "generating" });
    let _mock = server
        .mock("POST", "/v1/tenant/integrations/int-1/frameworks/CIS/reports")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["compliance-frameworks", "request-report", "--integration-id", "int-1",
          "--framework-id", "CIS", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("generating"));
}

#[tokio::test]
async fn test_cli_compliance_download() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/integrations/int-1/compliance-frameworks/CIS/download")
        .with_status(200)
        .with_body(b"pdf-content")
        .create_async()
        .await;

    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let output = run_plerion(
        &["compliance-frameworks", "download", "--integration-id", "int-1",
          "--framework-id", "CIS", "--output-file", &path],
        "key", &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let content = std::fs::read(&path).unwrap();
    assert_eq!(content, b"pdf-content");
}

// --- well-architected report/download ---

#[tokio::test]
async fn test_cli_well_architected_request_report() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "status": "generating" });
    let _mock = server
        .mock("POST", "/v1/tenant/integrations/int-1/frameworks/WAF/reports")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["well-architected-frameworks", "request-report", "--integration-id", "int-1",
          "--framework-id", "WAF", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("generating"));
}

#[tokio::test]
async fn test_cli_well_architected_download() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/integrations/int-1/well-architected-frameworks/WAF/download")
        .with_status(200)
        .with_body(b"wa-report-bytes")
        .create_async()
        .await;

    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let output = run_plerion(
        &["well-architected-frameworks", "download", "--integration-id", "int-1",
          "--framework-id", "WAF", "--output-file", &path],
        "key", &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let content = std::fs::read(&path).unwrap();
    assert_eq!(content, b"wa-report-bytes");
}

// --- findings --all pagination ---

#[tokio::test]
async fn test_cli_findings_list_all_pagination() {
    let mut server = Server::new_async().await;

    // Single page with hasNextPage=false (tests the --all path without multi-page complexity)
    let body = serde_json::json!({
        "data": [
            { "id": "f-1", "detectionId": "DET-1", "status": "FAILED", "severityLevel": "HIGH" },
            { "id": "f-2", "detectionId": "DET-2", "status": "PASSED", "severityLevel": "LOW" }
        ],
        "meta": { "cursor": null, "perPage": 1000, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["findings", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("DET-1"));
    assert!(stdout.contains("DET-2"));
}

// --- iac get-vulnerabilities ---

#[tokio::test]
async fn test_cli_iac_get_vulnerabilities() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "vulnerabilityId": "CVE-2024-5678", "title": "IaC Vuln", "severityLevel": "MEDIUM" }],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-vulnerabilities", "--scan-id", "scan-1", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("CVE-2024-5678"));
}

// --- assets --all pagination ---

#[tokio::test]
async fn test_cli_assets_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body1 = serde_json::json!({
        "data": [{ "id": "a-1", "name": "bucket-1" }],
        "meta": { "page": 1, "perPage": 1000, "total": 2, "hasNextPage": true }
    });
    let body2 = serde_json::json!({
        "data": [{ "id": "a-2", "name": "bucket-2" }],
        "meta": { "page": 2, "perPage": 1000, "total": 2, "hasNextPage": false }
    });
    let _mock1 = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()))
        .with_status(200)
        .with_body(body1.to_string())
        .create_async()
        .await;
    let _mock2 = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::UrlEncoded("page".to_string(), "2".to_string()))
        .with_status(200)
        .with_body(body2.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["assets", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("bucket-1"));
    assert!(stdout.contains("bucket-2"));
}

// --- iac get-findings with --status filter ---

#[tokio::test]
async fn test_cli_iac_get_findings_with_status_filter() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "f-1", "result": "FAILED", "severityLevel": "HIGH" }],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("results".to_string(), "FAILED".to_string()),
            mockito::Matcher::UrlEncoded("perPage".to_string(), "50".to_string()),
        ]))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-findings", "--scan-id", "scan-1", "--status", "FAILED", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("f-1"));
}

// --- iac get-findings --all pagination ---

#[tokio::test]
async fn test_cli_iac_get_findings_all_pagination() {
    let mut server = Server::new_async().await;
    let body1 = serde_json::json!({
        "data": [{ "id": "f-1", "result": "FAILED" }],
        "meta": { "page": 1, "perPage": 1000, "total": 2, "hasNextPage": true }
    });
    let body2 = serde_json::json!({
        "data": [{ "id": "f-2", "result": "PASSED" }],
        "meta": { "page": 2, "perPage": 1000, "total": 2, "hasNextPage": false }
    });
    let _mock1 = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()))
        .with_status(200)
        .with_body(body1.to_string())
        .create_async()
        .await;
    let _mock2 = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::UrlEncoded("page".to_string(), "2".to_string()))
        .with_status(200)
        .with_body(body2.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-findings", "--scan-id", "scan-1", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("f-1"));
    assert!(stdout.contains("f-2"));
}

// --- iac list-scans --all pagination ---

#[tokio::test]
async fn test_cli_iac_list_scans_all_pagination() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "scan-1", "artifactName": "test.zip", "status": "SUCCESS", "types": [] }],
        "meta": { "page": 1, "perPage": 1000, "total": 1, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "list-scans", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("scan-1"));
}

// --- iac get-vulnerabilities --all pagination ---

#[tokio::test]
async fn test_cli_iac_get_vulnerabilities_all_pagination() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "v-1", "vulnerabilityId": "CVE-2024-5678", "severityLevel": "HIGH" }],
        "meta": { "page": 1, "perPage": 1000, "total": 1, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-vulnerabilities", "--scan-id", "scan-1", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("CVE-2024-5678"));
}

// --- alerts --all cursor pagination ---

#[tokio::test]
async fn test_cli_alerts_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body1 = serde_json::json!({
        "data": [{ "id": "al-1", "title": "Alert 1" }],
        "meta": { "cursor": "cursor-1", "perPage": 1000, "hasNextPage": true }
    });
    let body2 = serde_json::json!({
        "data": [{ "id": "al-2", "title": "Alert 2" }],
        "meta": { "cursor": null, "perPage": 1000, "hasNextPage": false }
    });
    // First call returns page 1, second call (with cursor) returns page 2
    // mockito matches in reverse order, so the more specific match goes last
    let _mock2 = server
        .mock("GET", "/v1/tenant/alerts")
        .match_query(mockito::Matcher::UrlEncoded("cursor".to_string(), "cursor-1".to_string()))
        .with_status(200)
        .with_body(body2.to_string())
        .create_async()
        .await;
    let _mock1 = server
        .mock("GET", "/v1/tenant/alerts")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body1.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["alerts", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("al-1"));
    assert!(stdout.contains("al-2"));
}

// --- risks --all cursor pagination ---

#[tokio::test]
async fn test_cli_risks_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "r-1", "title": "Risk 1" }],
        "meta": { "cursor": null, "perPage": 1000, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/risks")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["risks", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("r-1"));
}

// --- audit-logs --all cursor pagination ---

#[tokio::test]
async fn test_cli_audit_logs_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "log-1", "operation": "LOGIN" }],
        "meta": { "cursor": null, "perPage": 1000, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/audit-logs")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["audit-logs", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("log-1"));
}

// --- vulnerabilities --all page pagination ---

#[tokio::test]
async fn test_cli_vulnerabilities_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body1 = serde_json::json!({
        "data": [{ "id": "v-1", "vulnerabilityId": "CVE-1" }],
        "meta": { "page": 1, "perPage": 1000, "total": 2, "hasNextPage": true }
    });
    let body2 = serde_json::json!({
        "data": [{ "id": "v-2", "vulnerabilityId": "CVE-2" }],
        "meta": { "page": 2, "perPage": 1000, "total": 2, "hasNextPage": false }
    });
    let _mock1 = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::UrlEncoded("page".to_string(), "1".to_string()))
        .with_status(200)
        .with_body(body1.to_string())
        .create_async()
        .await;
    let _mock2 = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::UrlEncoded("page".to_string(), "2".to_string()))
        .with_status(200)
        .with_body(body2.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("CVE-1"));
    assert!(stdout.contains("CVE-2"));
}

// --- integrations --all cursor pagination ---

#[tokio::test]
async fn test_cli_integrations_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "integrationId": "int-1", "name": "AWS Prod" }],
        "meta": { "cursor": null, "perPage": 1000, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["integrations", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("int-1"));
}

// --- asset-groups --all cursor pagination ---

#[tokio::test]
async fn test_cli_asset_groups_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "assetGroupId": "ag-1", "name": "Production" }],
        "meta": { "cursor": null, "perPage": 1000, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/asset-groups")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["asset-groups", "list", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("ag-1"));
}

// --- vuln exemptions --all cursor pagination ---

#[tokio::test]
async fn test_cli_vuln_exemptions_list_all_pagination() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "ex-1", "name": "Test", "reason": "ACCEPTED_RISK" }],
        "meta": { "hasNext": false, "nextCursor": null, "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/profiles/prof-1/vulnerability/exemptions")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "exemptions", "list", "--profile-id", "prof-1", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("ACCEPTED_RISK"));
}

// --- string meta values regression tests ---

#[tokio::test]
async fn test_cli_iac_get_findings_string_meta_values() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "f-1", "result": "FAILED", "severityLevel": "HIGH" }],
        "meta": { "page": "1", "perPage": "1000", "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-findings", "--scan-id", "scan-1", "--status", "FAILED", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("f-1"));
}

#[tokio::test]
async fn test_cli_iac_get_vulnerabilities_string_meta_values() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "v-1", "vulnerabilityId": "CVE-2024-9999", "severityLevel": "CRITICAL" }],
        "meta": { "page": "1", "perPage": "1000", "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-vulnerabilities", "--scan-id", "scan-1", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("CVE-2024-9999"));
}

#[tokio::test]
async fn test_cli_iac_list_scans_string_meta_values() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "scan-99", "artifactName": "test.zip", "status": "SUCCESS", "types": ["terraform"] }],
        "meta": { "page": "1", "perPage": "50", "total": "1", "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "list-scans", "--all", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("scan-99"));
}

// --- custom-checks ---

const CC_ID: &str = "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3";

fn custom_check_json() -> serde_json::Value {
    serde_json::json!({
        "customCheckId": CC_ID,
        "slug": "s3-no-public-acl",
        "organizationId": "org-1",
        "tenantId": "tenant-1",
        "title": "No public S3",
        "target": { "assetType": "AWS::S3::Bucket", "scope": "batch" },
        "defaults": { "severityLevel": "HIGH", "message": "Public", "informational": false },
        "type": "rego",
        "body": "package plerion",
        "version": 2,
        "createdBy": "alice",
        "createdAt": "2026-01-01T00:00:00Z",
        "updatedBy": "bob",
        "updatedAt": "2026-01-02T00:00:00Z"
    })
}

/// What a write should send for `custom_check_json()`: server-set fields and the id dropped.
fn custom_check_input_json() -> serde_json::Value {
    serde_json::json!({
        "slug": "s3-no-public-acl",
        "title": "No public S3",
        "target": { "assetType": "AWS::S3::Bucket", "scope": "batch" },
        "defaults": { "severityLevel": "HIGH", "message": "Public", "informational": false },
        "type": "rego",
        "body": "package plerion"
    })
}

fn write_temp_json(name: &str, value: &serde_json::Value) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("plerion-test-{}-{name}.json", std::process::id()));
    std::fs::write(&path, value.to_string()).unwrap();
    path
}

#[tokio::test]
async fn test_cli_custom_checks_list_table() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/custom-checks")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("assetType".into(), "AWS::S3::Bucket".into()),
            mockito::Matcher::UrlEncoded("scope".into(), "batch".into()),
            mockito::Matcher::UrlEncoded("limit".into(), "50".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({ "items": [custom_check_json()] }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["custom-checks", "list", "--asset-type", "AWS::S3::Bucket", "--scope", "batch"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
    assert!(stdout.contains("s3-no-public-acl"));
    assert!(stdout.contains("SEVERITY"));
}

#[tokio::test]
async fn test_cli_custom_checks_list_all_follows_next_cursor() {
    let mut server = Server::new_async().await;
    let page1 = server
        .mock("GET", "/v1/tenant/custom-checks")
        .match_query(mockito::Matcher::Exact("limit=200".into()))
        .with_status(200)
        .with_body(serde_json::json!({ "items": [{ "customCheckId": "cc-1" }], "nextCursor": "c2" }).to_string())
        .create_async()
        .await;
    let page2 = server
        .mock("GET", "/v1/tenant/custom-checks")
        .match_query(mockito::Matcher::Exact("limit=200&cursor=c2".into()))
        .with_status(200)
        .with_body(serde_json::json!({ "items": [{ "customCheckId": "cc-2" }] }).to_string())
        .create_async()
        .await;

    let output = run_plerion(&["custom-checks", "list", "--all", "--output", "json"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    page1.assert_async().await;
    page2.assert_async().await;
    assert!(stdout.contains("cc-1") && stdout.contains("cc-2"));
}

#[tokio::test]
async fn test_cli_custom_checks_get() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", format!("/v1/tenant/custom-checks/{CC_ID}").as_str())
        .with_status(200)
        .with_body(custom_check_json().to_string())
        .create_async()
        .await;

    let output = run_plerion(&["custom-checks", "get", "--id", CC_ID, "--output", "json"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
    assert!(stdout.contains("package plerion"));
}

/// `get` output fed straight back to `create` sends only the writable fields.
#[tokio::test]
async fn test_cli_custom_checks_create_from_file_drops_server_fields() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/tenant/custom-checks")
        .match_body(mockito::Matcher::Json(custom_check_input_json()))
        .with_status(200)
        .with_body(custom_check_json().to_string())
        .create_async()
        .await;

    let path = write_temp_json("create", &custom_check_json());
    let output = run_plerion(
        &["custom-checks", "create", "--file", path.to_str().unwrap(), "--output", "json"],
        "key", &server.url(),
    );
    std::fs::remove_file(&path).ok();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cli_custom_checks_update_from_stdin() {
    use std::io::Write;
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", format!("/v1/tenant/custom-checks/{CC_ID}").as_str())
        .match_body(mockito::Matcher::Json(custom_check_input_json()))
        .with_status(200)
        .with_body(custom_check_json().to_string())
        .create_async()
        .await;

    let mut child = Command::new(env!("CARGO_BIN_EXE_plerion"))
        .args(["custom-checks", "update", "--id", CC_ID, "--file", "-", "--output", "json"])
        .env("PLERION_API_KEY", "key")
        .env("PLERION_ENDPOINT_URL", server.url())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(custom_check_input_json().to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

/// An empty or non-object definition fails before any HTTP call.
#[test]
fn test_cli_custom_checks_rejects_an_empty_definition() {
    for (name, body) in [("empty", serde_json::json!({})), ("array", serde_json::json!([]))] {
        let path = write_temp_json(name, &body);
        let output = run_plerion(
            &["custom-checks", "create", "--file", path.to_str().unwrap()],
            "key", "http://127.0.0.1:1",
        );
        std::fs::remove_file(&path).ok();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(!output.status.success(), "expected failure for {name}");
        assert!(stderr.contains("--file"), "for {name} stderr was: {stderr}");
    }
}

#[tokio::test]
async fn test_cli_custom_checks_delete() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", format!("/v1/tenant/custom-checks/{CC_ID}").as_str())
        .with_status(202)
        .with_body(r#"{"accepted":true}"#)
        .create_async()
        .await;

    let output = run_plerion(&["custom-checks", "delete", "--id", CC_ID], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
    assert!(stdout.contains("deleted"));
}

/// A dry run keeps the check id, which labels its results.
#[tokio::test]
async fn test_cli_custom_checks_dry_run() {
    let mut server = Server::new_async().await;
    let mut check = custom_check_input_json();
    check["customCheckId"] = serde_json::json!(CC_ID);
    let mock = server
        .mock("POST", "/v1/tenant/custom-check-dry-runs")
        .match_body(mockito::Matcher::Json(serde_json::json!({ "integrationId": "int-1", "check": check })))
        .with_status(202)
        .with_body(
            serde_json::json!({ "dryRunId": "dr-1", "status": "RUNNING", "pollUrl": "/v1/tenant/custom-check-dry-runs/dr-1" })
                .to_string(),
        )
        .create_async()
        .await;

    let path = write_temp_json("dry-run", &custom_check_json());
    let output = run_plerion(
        &["custom-checks", "dry-run", "--integration-id", "int-1", "--file", path.to_str().unwrap()],
        "key", &server.url(),
    );
    std::fs::remove_file(&path).ok();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
    assert!(stdout.contains("dr-1") && stdout.contains("RUNNING"));
}

#[tokio::test]
async fn test_cli_custom_checks_dry_run_status() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/custom-check-dry-runs/dr-1")
        .with_status(200)
        .with_body(
            serde_json::json!({ "dryRunId": "dr-1", "status": "SUCCEEDED", "output": { "findings": [{ "assetId": "a-1" }] } })
                .to_string(),
        )
        .create_async()
        .await;

    let output = run_plerion(
        &["custom-checks", "dry-run-status", "--id", "dr-1", "--output", "json"],
        "key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
    assert!(stdout.contains("a-1"));
}

// --- custom-reports ---

#[tokio::test]
async fn test_cli_custom_reports_list() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/custom-reports")
        .match_query(mockito::Matcher::Exact("perPage=20".into()))
        .with_status(200)
        .with_body(
            serde_json::json!({ "data": [{ "id": "r-1", "name": "Posture" }], "meta": { "perPage": 20, "total": 1, "cursor": null } })
                .to_string(),
        )
        .create_async()
        .await;

    let output = run_plerion(&["custom-reports", "list", "--per-page", "20"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
    assert!(stdout.contains("Posture"));
}

/// No hasNextPage on this endpoint: --all follows meta.cursor and stops on an
/// empty page even if a cursor comes back with it.
#[tokio::test]
async fn test_cli_custom_reports_list_all_follows_cursor() {
    let mut server = Server::new_async().await;
    let page1 = server
        .mock("GET", "/v1/tenant/custom-reports")
        .match_query(mockito::Matcher::Exact("perPage=100".into()))
        .with_status(200)
        .with_body(serde_json::json!({ "data": [{ "id": "r-1" }], "meta": { "cursor": "c2" } }).to_string())
        .create_async()
        .await;
    let page2 = server
        .mock("GET", "/v1/tenant/custom-reports")
        .match_query(mockito::Matcher::Exact("perPage=100&cursor=c2".into()))
        .with_status(200)
        .with_body(serde_json::json!({ "data": [{ "id": "r-2" }], "meta": { "cursor": "c3" } }).to_string())
        .create_async()
        .await;
    let page3 = server
        .mock("GET", "/v1/tenant/custom-reports")
        .match_query(mockito::Matcher::Exact("perPage=100&cursor=c3".into()))
        .with_status(200)
        .with_body(serde_json::json!({ "data": [], "meta": { "cursor": "c4" } }).to_string())
        .create_async()
        .await;

    let output = run_plerion(&["custom-reports", "list", "--all", "--output", "json"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    page1.assert_async().await;
    page2.assert_async().await;
    page3.assert_async().await;
    assert!(stdout.contains("r-1") && stdout.contains("r-2"));
}

#[test]
fn test_cli_custom_reports_rejects_per_page_over_100() {
    let output = run_plerion(&["custom-reports", "list", "--per-page", "101"], "key", "http://127.0.0.1:1");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("101"));
}

// --- profiles ---

fn exemption_set_body() -> String {
    serde_json::json!({
        "data": {
            "profileId": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
            "detectionId": "PLERION-AWS-16",
            "provider": "AWS",
            "supportsExemptions": true,
            "supportedExemptionTypes": ["NAME_EXEMPTION", "REGION_EXEMPTION"],
            "exemptions": [ { "type": "REGION_EXEMPTION", "regions": ["us-west-2"] } ],
            "version": "v-123"
        }
    })
    .to_string()
}

const DETECTION_EXEMPTIONS_PATH: &str =
    "/v1/tenant/profiles/default/detection-settings/PLERION-AWS-16/exemptions";

#[tokio::test]
async fn test_cli_profiles_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [ { "profileId": "p-1", "name": "Production", "isDefault": false,
                    "integrations": [ { "integrationId": "i-1", "name": "acme-prod" } ] } ],
        "meta": { "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/profiles")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["profiles", "list"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("PROFILE ID"));
    assert!(stdout.contains("Production"));
}

#[tokio::test]
async fn test_cli_profiles_detection_exemptions_get() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", DETECTION_EXEMPTIONS_PATH)
        .with_status(200)
        .with_body(exemption_set_body())
        .create_async()
        .await;

    let output = run_plerion(
        &[
            "profiles",
            "detection-exemptions",
            "get",
            "--profile-id",
            "default",
            "--detection-id",
            "PLERION-AWS-16",
        ],
        "key",
        &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("v-123"), "the version is shown: {stdout}");
    assert!(stdout.contains("REGION_EXEMPTION"));
    assert!(
        stdout.contains("us-west-2"),
        "the exemptions are listed: {stdout}"
    );
}

#[tokio::test]
async fn test_cli_profiles_detection_exemptions_replace() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", DETECTION_EXEMPTIONS_PATH)
        .match_header("If-Match", "v-123")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "exemptions": [ { "type": "REGION_EXEMPTION", "regions": ["us-west-2"] } ]
        })))
        .with_status(200)
        .with_body(exemption_set_body())
        .create_async()
        .await;

    // The output of `get --output json` is accepted as input.
    let get_output = serde_json::from_str::<serde_json::Value>(&exemption_set_body()).unwrap()
        ["data"]
        .to_string();
    let output = run_plerion(
        &[
            "profiles",
            "detection-exemptions",
            "replace",
            "--profile-id",
            "default",
            "--detection-id",
            "PLERION-AWS-16",
            "--exemptions",
            &get_output,
            "--if-match",
            "v-123",
            "--output",
            "json",
        ],
        "key",
        &server.url(),
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cli_profiles_detection_exemptions_replace_from_file() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", DETECTION_EXEMPTIONS_PATH)
        .match_header("If-Match", mockito::Matcher::Missing)
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "exemptions": [ { "type": "NAME_EXEMPTION", "condition": "equals", "value": "x" } ]
        })))
        .with_status(200)
        .with_body(exemption_set_body())
        .create_async()
        .await;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("set.json");
    std::fs::write(
        &path,
        r#"[{"type":"NAME_EXEMPTION","condition":"equals","value":"x"}]"#,
    )
    .unwrap();
    let output = run_plerion(
        &[
            "profiles",
            "detection-exemptions",
            "replace",
            "--profile-id",
            "default",
            "--detection-id",
            "PLERION-AWS-16",
            "--file",
            path.to_str().unwrap(),
        ],
        "key",
        &server.url(),
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cli_profiles_detection_exemptions_clear() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", DETECTION_EXEMPTIONS_PATH)
        .match_body(mockito::Matcher::Json(
            serde_json::json!({ "exemptions": [] }),
        ))
        .with_status(200)
        .with_body(exemption_set_body())
        .create_async()
        .await;

    let output = run_plerion(
        &[
            "profiles",
            "detection-exemptions",
            "replace",
            "--profile-id",
            "default",
            "--detection-id",
            "PLERION-AWS-16",
            "--clear",
        ],
        "key",
        &server.url(),
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    mock.assert_async().await;
}

/// No input, or an empty set without --clear, is refused before any HTTP call.
#[test]
fn test_cli_profiles_detection_exemptions_replace_requires_input() {
    let base = [
        "profiles",
        "detection-exemptions",
        "replace",
        "--profile-id",
        "default",
        "--detection-id",
        "X",
    ];
    let output = run_plerion(&base, "key", "http://127.0.0.1:1");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(
        stderr.contains("--exemptions or --file"),
        "stderr was: {stderr}"
    );

    let mut args = base.to_vec();
    args.extend(["--exemptions", "[]"]);
    let output = run_plerion(&args, "key", "http://127.0.0.1:1");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(stderr.contains("--clear"), "stderr was: {stderr}");
}

#[test]
fn test_cli_profiles_detection_exemptions_clear_conflicts_with_input() {
    let output = run_plerion(
        &[
            "profiles",
            "detection-exemptions",
            "replace",
            "--profile-id",
            "default",
            "--detection-id",
            "X",
            "--clear",
            "--exemptions",
            "[{}]",
        ],
        "key",
        "http://127.0.0.1:1",
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(
        stderr.contains("cannot be used with"),
        "stderr was: {stderr}"
    );
}

// --- integrations set-tags ---

#[tokio::test]
async fn test_cli_integrations_set_tags() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", "/v1/tenant/integrations/int-1/user-defined-tags")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "tags": [ { "key": "Division", "value": "payments" }, { "key": "Owner", "value": "a=b" } ]
        })))
        .with_status(200)
        .with_body(r#"{"data":{"tags":[{"key":"Division","value":"payments","source":"user-defined"}]}}"#)
        .create_async()
        .await;

    let output = run_plerion(
        &[
            "integrations",
            "set-tags",
            "int-1",
            "--tag",
            "Division=payments",
            "--tag",
            "Owner=a=b",
        ],
        "key",
        &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    mock.assert_async().await;
    assert!(stdout.contains("SOURCE"));
    assert!(stdout.contains("user-defined"));
}

#[tokio::test]
async fn test_cli_integrations_set_tags_clear() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", "/v1/tenant/integrations/int-1/user-defined-tags")
        .match_body(mockito::Matcher::Json(serde_json::json!({ "tags": [] })))
        .with_status(200)
        .with_body(r#"{"data":{"tags":[]}}"#)
        .create_async()
        .await;

    let output = run_plerion(
        &["integrations", "set-tags", "int-1", "--clear"],
        "key",
        &server.url(),
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    mock.assert_async().await;
}

#[test]
fn test_cli_integrations_set_tags_requires_a_tag() {
    let output = run_plerion(
        &["integrations", "set-tags", "int-1"],
        "key",
        "http://127.0.0.1:1",
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(stderr.contains("--tag KEY=VALUE"), "stderr was: {stderr}");

    let output = run_plerion(
        &[
            "integrations",
            "set-tags",
            "int-1",
            "--tag",
            "a=1",
            "--clear",
        ],
        "key",
        "http://127.0.0.1:1",
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(
        stderr.contains("cannot be used with"),
        "stderr was: {stderr}"
    );
}

// --- tenant home-dashboard and api-access ---

#[tokio::test]
async fn test_cli_tenant_home_dashboard_get() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/preferences/home-dashboard")
        .with_status(200)
        .with_body(r#"{"data":{"tenantId":"t-1","homeReportId":"r-1","updatedAt":"2026-10-01T00:00:00Z"}}"#)
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "home-dashboard", "get"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("HOME REPORT ID"));
    assert!(stdout.contains("r-1"));
}

#[tokio::test]
async fn test_cli_tenant_home_dashboard_set() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", "/v1/tenant/preferences/home-dashboard")
        .match_body(mockito::Matcher::Json(
            serde_json::json!({ "homeReportId": "r-2" }),
        ))
        .with_status(200)
        .with_body(r#"{"data":{"tenantId":"t-1","homeReportId":"r-2"}}"#)
        .create_async()
        .await;

    let output = run_plerion(
        &[
            "tenant",
            "home-dashboard",
            "set",
            "--home-report-id",
            "r-2",
            "--output",
            "json",
        ],
        "key",
        &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    mock.assert_async().await;
    assert!(stdout.contains("\"homeReportId\": \"r-2\""));
}

#[test]
fn test_cli_tenant_home_dashboard_set_rejects_empty_id() {
    let output = run_plerion(
        &["tenant", "home-dashboard", "set", "--home-report-id", ""],
        "key",
        "http://127.0.0.1:1",
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(stderr.contains("--home-report-id"), "stderr was: {stderr}");
}

#[tokio::test]
async fn test_cli_tenant_home_dashboard_clear() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/tenant/preferences/home-dashboard")
        .with_status(204)
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "home-dashboard", "clear"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    mock.assert_async().await;
    assert!(stdout.contains("cleared"));
}

fn api_access_body() -> String {
    serde_json::json!({
        "openapi": "3.1.0",
        "x-plerion-role": { "name": "Tenant read-only" },
        "paths": {
            "/v1/tenant/profiles": { "get": { "operationId": "listProfiles", "summary": "List" } }
        }
    })
    .to_string()
}

#[tokio::test]
async fn test_cli_tenant_api_access_table() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/openapi")
        .with_status(200)
        .with_body(api_access_body())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "api-access"], "key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("OPERATION ID"));
    assert!(stdout.contains("listProfiles"));
    assert!(stdout.contains("/v1/tenant/profiles"));
}

#[tokio::test]
async fn test_cli_tenant_api_access_json_returns_document() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/openapi")
        .with_status(200)
        .with_body(api_access_body())
        .create_async()
        .await;

    let output = run_plerion(
        &["tenant", "api-access", "--output", "json"],
        "key",
        &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["x-plerion-role"]["name"], "Tenant read-only");
    assert_eq!(doc["openapi"], "3.1.0");
}
