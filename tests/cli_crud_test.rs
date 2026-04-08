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
        &["asset-groups", "create", "--name", "NewGroup", "--output", "json"],
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
        &["asset-groups", "update", "ag-1", "--name", "Updated", "--output", "json"],
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
        &["asset-groups", "delete", "ag-1"],
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
        &["vulnerabilities", "exemptions", "get", "--profile-id", "prof-1", "ex-1", "--output", "json"],
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
          "--name", "New Exemption", "--reason", "NOT_IN_USE", "--output", "json"],
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
        &["vulnerabilities", "exemptions", "delete", "--profile-id", "prof-1", "ex-1"],
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
        "data": [{ "vulnerabilityId": "CVE-2024-5678", "title": "IaC Vuln", "severityLevel": "MEDIUM" }]
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-vulnerabilities", "scan-1", "--output", "json"],
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
