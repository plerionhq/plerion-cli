use mockito::Server;
use std::process::Command;

/// Helper: run the plerion CLI binary with the given args and env overrides.
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

/// Run the plerion binary without a server (for clap-level validation failures).
fn run_plerion_no_server(args: &[&str]) -> std::process::Output {
    let binary = env!("CARGO_BIN_EXE_plerion");
    Command::new(binary)
        .args(args)
        .env("PLERION_API_KEY", "test-key")
        .env("PLERION_ENDPOINT_URL", "http://127.0.0.1:1") // won't be reached
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to execute plerion binary")
}

// ============================================================
// Findings: --sort-by / --sort-order validation
// ============================================================

#[test]
fn test_findings_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["findings", "list", "--sort-by", "invalid_field"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[test]
fn test_findings_rejects_invalid_sort_order() {
    let output = run_plerion_no_server(&["findings", "list", "--sort-order", "ASCENDING"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[tokio::test]
async fn test_findings_accepts_valid_sort_by() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sortBy".to_string(), "severityLevel".to_string()),
            mockito::Matcher::UrlEncoded("sortOrder".to_string(), "desc".to_string()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["findings", "list", "--sort-by", "severityLevel", "--sort-order", "desc", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

// ============================================================
// Alerts: --sort-by / --sort-order validation
// ============================================================

#[test]
fn test_alerts_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["alerts", "list", "--sort-by", "name"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[tokio::test]
async fn test_alerts_accepts_valid_sort_by() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/alerts")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sortBy".to_string(), "riskScore".to_string()),
            mockito::Matcher::UrlEncoded("sortOrder".to_string(), "asc".to_string()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["alerts", "list", "--sort-by", "riskScore", "--sort-order", "asc", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

// ============================================================
// Risks: --sort-by / --sort-order validation (uses asc/desc)
// ============================================================

#[test]
fn test_risks_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["risks", "list", "--sort-by", "name"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[test]
fn test_risks_rejects_wrong_case_sort_order() {
    let output = run_plerion_no_server(&["risks", "list", "--sort-order", "ASC"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[tokio::test]
async fn test_risks_accepts_valid_sort_by() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/risks")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sortBy".to_string(), "score".to_string()),
            mockito::Matcher::UrlEncoded("sortOrder".to_string(), "desc".to_string()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["risks", "list", "--sort-by", "score", "--sort-order", "desc", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

// ============================================================
// Assets: --sort-by / --sort-order validation
// ============================================================

#[test]
fn test_assets_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["assets", "list", "--sort-by", "invalid"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[tokio::test]
async fn test_assets_accepts_valid_sort_by() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sortBy".to_string(), "riskScore".to_string()),
            mockito::Matcher::UrlEncoded("sortOrder".to_string(), "desc".to_string()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "page": 1, "perPage": 50, "total": 0 }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["assets", "list", "--sort-by", "riskScore", "--sort-order", "desc", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

// ============================================================
// Vulnerabilities: --sort-by / --sort-order validation
// ============================================================

#[test]
fn test_vulnerabilities_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["vulnerabilities", "list", "--sort-by", "name"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[tokio::test]
async fn test_vulnerabilities_accepts_valid_sort_by() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sortBy".to_string(), "severityLevelValue".to_string()),
            mockito::Matcher::UrlEncoded("sortOrder".to_string(), "asc".to_string()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "page": 1, "perPage": 50, "total": 0 }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "list", "--sort-by", "severityLevelValue", "--sort-order", "asc", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

// ============================================================
// IaC list-scans: --sort-by / --sort-order validation (uses asc/desc)
// ============================================================

#[test]
fn test_iac_list_scans_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["iac", "list-scans", "--sort-by", "invalid"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[tokio::test]
async fn test_iac_list_scans_accepts_valid_sort_by() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sortBy".to_string(), "createdAt".to_string()),
            mockito::Matcher::UrlEncoded("sortOrder".to_string(), "desc".to_string()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "list-scans", "--sort-by", "createdAt", "--sort-order", "desc", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

// ============================================================
// IaC get-findings: --sort-by / --sort-order validation
// ============================================================

#[test]
fn test_iac_get_findings_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["iac", "get-findings", "--scan-id", "s-1", "--sort-by", "invalid"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

// ============================================================
// IaC get-vulnerabilities: --sort-by / --sort-order validation
// ============================================================

#[test]
fn test_iac_get_vulnerabilities_rejects_invalid_sort_by() {
    let output = run_plerion_no_server(&["iac", "get-vulnerabilities", "--scan-id", "s-1", "--sort-by", "invalid"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid value") || stderr.contains("possible values"),
        "Expected clap validation error, got: {stderr}"
    );
}

#[tokio::test]
async fn test_iac_get_vulnerabilities_accepts_valid_sort_by() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/vulnerabilities")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("sortBy".to_string(), "vulnerabilityId".to_string()),
            mockito::Matcher::UrlEncoded("sortOrder".to_string(), "asc".to_string()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["iac", "get-vulnerabilities", "--scan-id", "scan-1", "--sort-by", "vulnerabilityId", "--sort-order", "asc", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

// ============================================================
// Verify --help shows possible values
// ============================================================

#[test]
fn test_findings_help_shows_sort_values() {
    let output = run_plerion_no_server(&["findings", "list", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("severityLevel") && stdout.contains("provider"),
        "Help should list possible sort-by values, got: {stdout}"
    );
    assert!(
        stdout.contains("asc") && stdout.contains("desc"),
        "Help should list possible sort-order values, got: {stdout}"
    );
}

#[test]
fn test_risks_help_shows_sort_values() {
    let output = run_plerion_no_server(&["risks", "list", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("discoveredAt") && stdout.contains("score"),
        "Help should list possible sort-by values, got: {stdout}"
    );
}

#[test]
fn test_iac_list_scans_help_shows_sort_values() {
    let output = run_plerion_no_server(&["iac", "list-scans", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("createdAt") && stdout.contains("artifactName"),
        "Help should list possible sort-by values, got: {stdout}"
    );
}
