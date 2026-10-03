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

fn tenant_body() -> String {
    serde_json::json!({
        "data": {
            "tenantId": "t-123",
            "organizationId": "org-456",
            "name": "Test Org",
            "plan": "Enterprise"
        }
    })
    .to_string()
}

fn findings_body() -> String {
    serde_json::json!({
        "data": [{
            "id": "finding-1",
            "detectionId": "PLERION-AWS-16",
            "status": "FAILED",
            "severityLevel": "CRITICAL",
            "provider": "AWS"
        }],
        "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
    })
    .to_string()
}

// --- tenant ---

#[tokio::test]
async fn test_cli_tenant_get_json() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant")
        .with_status(200)
        .with_body(tenant_body())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("Test Org"));
    assert!(stdout.contains("t-123"));
}

#[tokio::test]
async fn test_cli_tenant_get_table() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant")
        .with_status(200)
        .with_body(tenant_body())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get", "--output", "table", "--no-color"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("Test Org"));
}

#[tokio::test]
async fn test_cli_tenant_get_yaml() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant")
        .with_status(200)
        .with_body(tenant_body())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get", "--output", "yaml"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("Test Org"));
}

#[tokio::test]
async fn test_cli_tenant_get_text() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant")
        .with_status(200)
        .with_body(tenant_body())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get", "--output", "text"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("Test Org"));
}

#[tokio::test]
async fn test_cli_tenant_get_usage() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "assets": 5000, "integrations": 3 } });
    let _mock = server
        .mock("GET", "/v1/tenant/usage")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get-usage", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("5000"));
}

#[tokio::test]
async fn test_cli_tenant_get_usage_table() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "assets": 5000, "integrations": 3 } });
    let _mock = server
        .mock("GET", "/v1/tenant/usage")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get-usage", "--output", "table"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("ASSETS"), "Table should contain ASSETS header, got: {stdout}");
    assert!(stdout.contains("5000"), "Table should contain value 5000, got: {stdout}");
}

#[tokio::test]
async fn test_cli_tenant_get_usage_text() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "assets": 5000, "integrations": 3 } });
    let _mock = server
        .mock("GET", "/v1/tenant/usage")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get-usage", "--output", "text"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("5000"), "Text should contain value 5000, got: {stdout}");
}

// --- findings ---

#[tokio::test]
async fn test_cli_findings_list_json() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(findings_body())
        .create_async()
        .await;

    let output = run_plerion(&["findings", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("PLERION-AWS-16"));
}

#[tokio::test]
async fn test_cli_findings_list_table() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(findings_body())
        .create_async()
        .await;

    let output = run_plerion(&["findings", "list", "--output", "table", "--no-color"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("PLERION-AWS-16"));
}

#[tokio::test]
async fn test_cli_findings_list_with_query() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(findings_body())
        .create_async()
        .await;

    let output = run_plerion(
        &["findings", "list", "--output", "json", "--query", "[0].detectionId"],
        "test-key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("PLERION-AWS-16"));
}

// --- alerts ---

#[tokio::test]
async fn test_cli_alerts_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "alert-1", "status": "OPEN", "title": "Alert Title", "alertType": "FINDING" }],
        "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/alerts")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["alerts", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("Alert Title"));
}

// --- risks ---

#[tokio::test]
async fn test_cli_risks_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "risk-1", "riskTypeId": "S3_PUBLIC_ACCESS", "severityLevel": "CRITICAL", "score": 9.8 }],
        "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/risks")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["risks", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("S3_PUBLIC_ACCESS"));
}

// --- assets ---

#[tokio::test]
async fn test_cli_assets_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "asset-1", "name": "my-bucket", "provider": "AWS", "resourceType": "AWS::S3::Bucket" }],
        "meta": { "page": 1, "perPage": 50, "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["assets", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("my-bucket"));
}

#[tokio::test]
async fn test_cli_assets_get() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "id": "asset-1", "name": "my-instance" } });
    let _mock = server
        .mock("GET", "/v1/tenant/assets/asset-1")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["assets", "get", "--asset-id", "asset-1", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("my-instance"));
}

#[tokio::test]
async fn test_cli_assets_get_sbom() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "packages": [{"name": "openssl"}] } });
    let _mock = server
        .mock("GET", "/v1/tenant/assets/asset-1/sbom")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["assets", "get-sbom", "--asset-id", "asset-1", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("openssl"));
}

// --- integrations ---

#[tokio::test]
async fn test_cli_integrations_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "integrationId": "int-1", "name": "AWS Prod", "provider": "AWS", "status": "Active" }],
        "meta": { "perPage": 50 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["integrations", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("AWS Prod"));
}

// --- audit-logs ---

#[tokio::test]
async fn test_cli_audit_logs_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "log-1", "operation": "UserLogin", "operationTime": "2025-01-01T00:00:00Z" }],
        "meta": { "cursor": null, "perPage": 50 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/audit-logs")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["audit-logs", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("UserLogin"));
}

// --- vulnerabilities ---

#[tokio::test]
async fn test_cli_vulnerabilities_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "vulnerabilityId": "CVE-2024-1234", "title": "Test Vuln", "severityLevel": "HIGH" }],
        "meta": { "page": 1, "perPage": 50, "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["vulnerabilities", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("CVE-2024-1234"));
}

// --- compliance-frameworks ---

#[tokio::test]
async fn test_cli_compliance_frameworks_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "frameworks": [{ "id": "CIS-1", "name": "CIS AWS", "version": "v1.4", "posture": 85.0 }],
            "totalPosture": 85.0
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/compliance-frameworks")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["compliance-frameworks", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("CIS AWS"));
}

// --- well-architected-frameworks ---

#[tokio::test]
async fn test_cli_well_architected_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "frameworks": [{ "id": "WAF-1", "name": "Well-Architected" }],
            "totalPosture": 90.0
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/well-architected-frameworks")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["well-architected-frameworks", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("Well-Architected"));
}

// --- aws ---

#[tokio::test]
async fn test_cli_aws_get_external_id() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "externalId": "ext-123" } });
    let _mock = server
        .mock("GET", "/v1/tenant/external-id")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["aws", "get-external-id", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("ext-123"));
}

#[tokio::test]
async fn test_cli_aws_get_cloudformation_template() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "template": "AWSTemplate" } });
    let _mock = server
        .mock("GET", "/v1/tenant/cloudformation-templates")
        .match_query(mockito::Matcher::UrlEncoded("type".to_string(), "AWSAccount".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["aws", "get-cloudformation-template", "--type", "AWSAccount", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("AWSTemplate"));
}

#[tokio::test]
async fn test_cli_aws_generate_token() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "token": "tok-xyz" } });
    let _mock = server
        .mock("POST", "/v1/tenant/integrations/token")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["aws", "generate-token", "--integration-id", "int-1", "--output", "json"],
        "test-key", &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("tok-xyz"));
}

// --- asset-groups ---

#[tokio::test]
async fn test_cli_asset_groups_list() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "assetGroupId": "ag-1", "name": "Production", "status": "Active" }],
        "meta": { "cursor": null, "perPage": 50 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/asset-groups")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["asset-groups", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("Production"));
}

#[tokio::test]
async fn test_cli_asset_groups_get() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "assetGroupId": "ag-1", "name": "Staging" } });
    let _mock = server
        .mock("GET", "/v1/tenant/asset-groups/ag-1")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["asset-groups", "get", "--id", "ag-1", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("Staging"));
}

// --- iac ---

#[tokio::test]
async fn test_cli_iac_list_scans() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "scan-1", "artifactName": "test.zip", "status": "COMPLETED", "types": [] }],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["iac", "list-scans", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("scan-1"));
}

#[tokio::test]
async fn test_cli_iac_get_findings() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "f-1", "severityLevel": "HIGH", "detectionTitle": "Insecure" }],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/shiftleft/iac/scans/scan-1/findings")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(&["iac", "get-findings", "--scan-id", "scan-1", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("Insecure"));
}

// --- provider normalization ---

#[tokio::test]
async fn test_cli_findings_normalizes_provider() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/findings")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("providers".to_string(), "Azure".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["findings", "list", "--provider", "azure", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cli_alerts_normalizes_provider() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/alerts")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("providers".to_string(), "GCP".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::json!({
            "data": [],
            "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
        }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["alerts", "list", "--provider", "gcp", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cli_alerts_risk_score_string() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{
            "id": "alert-str",
            "status": "OPEN",
            "title": "String score alert",
            "riskScore": "7.7700"
        }],
        "meta": { "cursor": null, "perPage": 50, "hasNextPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/alerts")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["alerts", "list", "--output", "json"],
        "test-key",
        &server.url(),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("7.77") || stdout.contains("7.7700"));
}

// --- error handling ---

#[tokio::test]
async fn test_cli_api_error_propagates() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant")
        .with_status(401)
        .with_body(r#"{"message":"Unauthorized"}"#)
        .create_async()
        .await;

    let output = run_plerion(&["tenant", "get", "--output", "json"], "bad-key", &server.url());
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("401") || stderr.contains("Unauthorized"));
}

// --- configure list ---

#[test]
fn test_cli_configure_list() {
    let binary = env!("CARGO_BIN_EXE_plerion");
    let output = Command::new(binary)
        .args(["configure", "list"])
        .output()
        .expect("failed to execute plerion binary");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("profile") || stdout.contains("No profiles"));
}

// --- access grants ---

#[tokio::test]
async fn test_cli_access_grants_stats() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/aws/access-grants/stats")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            serde_json::json!({
                "data": { "total": 531, "external": 337, "untrustedExternal": 0, "crossOrg": 298 }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let output = run_plerion(&["access-grants", "stats"], "k", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("531"));
    assert!(stdout.contains("CROSS ORG"));
}

#[tokio::test]
async fn test_cli_access_grants_list_renders_renamed_columns() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/aws/access-grants")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            serde_json::json!({
                "data": [{
                    "id": "g-1",
                    "resourceType": "AWS::S3::Bucket",
                    "awsAccountId": "111122223333",
                    "assetName": "acme-prod-exports",
                    "grantOrigin": "external"
                }],
                "meta": { "perPage": 50, "total": 1, "cursor": null }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let output = run_plerion(&["access-grants", "list", "--per-page", "1"], "k", &server.url());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("RESOURCE TYPE"));
    assert!(stdout.contains("AWS ACCOUNT ID"));
    assert!(stdout.contains("acme-prod-exports"));
}

/// A body with no review fields is rejected locally, before any HTTP call.
#[test]
fn test_cli_access_grants_update_requires_a_field() {
    let binary = env!("CARGO_BIN_EXE_plerion");
    let output = Command::new(binary)
        .args(["access-grants", "update", "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3"])
        .env("PLERION_API_KEY", "k")
        .env("PLERION_ENDPOINT_URL", "http://127.0.0.1:1")
        .output()
        .expect("failed to execute plerion binary");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(stderr.contains("Nothing to update"));
}

/// Setting and clearing the same field is a clap-level conflict.
#[test]
fn test_cli_access_grants_update_rejects_set_and_clear_together() {
    let binary = env!("CARGO_BIN_EXE_plerion");
    let output = Command::new(binary)
        .args([
            "access-grants", "update", "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3",
            "--review-decision", "keep", "--clear-review-decision",
        ])
        .env("PLERION_API_KEY", "k")
        .output()
        .expect("failed to execute plerion binary");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(stderr.contains("cannot be used with"), "stderr was: {stderr}");
}

/// A non-UUID id is caught locally, so it never reaches the API as a path
/// segment where it could resolve to a sibling route.
#[test]
fn test_cli_access_grants_rejects_a_non_uuid_id() {
    let binary = env!("CARGO_BIN_EXE_plerion");
    for args in [
        vec!["access-grants", "get", "stats"],
        vec!["access-grants", "get", "external-principals"],
        vec!["access-grants", "update", "stats", "--review-decision", "keep"],
    ] {
        let output = Command::new(binary)
            .args(&args)
            .env("PLERION_API_KEY", "k")
            .env("PLERION_ENDPOINT_URL", "http://127.0.0.1:1")
            .output()
            .expect("failed to execute plerion binary");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(!output.status.success(), "expected failure for {args:?}");
        assert!(
            stderr.contains("is not a valid grant ID"),
            "for {args:?} stderr was: {stderr}"
        );
    }
}

#[tokio::test]
async fn test_cli_vulnerabilities_epss_filters_drop_bound_at_scale_end() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::Exact("epssScoreLte=0.5&hasEpssScore=true&perPage=50".to_string()))
        .with_status(200)
        .with_body(serde_json::json!({ "data": [], "meta": { "page": 1, "perPage": 50, "total": 0 } }).to_string())
        .create_async()
        .await;

    let output = run_plerion(
        &["vulnerabilities", "list", "--epss-score-gte", "0", "--epss-score-lte", "0.5", "--has-epss-score", "true", "--output", "json"],
        "test-key",
        &server.url(),
    );
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cli_vulnerabilities_table_shows_epss_percent() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "vulnerabilityId": "CVE-2021-44228", "epssScore": 0.943, "epssScoreDate": "2026-10-04" }],
        "meta": { "page": 1, "perPage": 50, "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let table = run_plerion(&["vulnerabilities", "list", "--output", "table"], "test-key", &server.url());
    assert!(table.status.success(), "stderr: {}", String::from_utf8_lossy(&table.stderr));
    assert!(String::from_utf8(table.stdout).unwrap().contains("94.3%"));

    let json = run_plerion(&["vulnerabilities", "list", "--output", "json"], "test-key", &server.url());
    let stdout = String::from_utf8(json.stdout).unwrap();
    assert!(stdout.contains("\"epssScore\": 0.943"), "{stdout}");
    assert!(stdout.contains("\"epssScoreDate\": \"2026-10-04\""), "{stdout}");
}

#[test]
fn test_cli_vulnerabilities_rejects_bad_epss_input() {
    let cases: [&[&str]; 4] = [
        &["--epss-score-gte", "1.5"],
        &["--epss-score-lte", "abc"],
        &["--epss-score-gte", "0.6", "--epss-score-lte", "0.5"],
        &["--epss-score-gte", "0.1", "--has-epss-score", "false"],
    ];
    for extra in cases {
        let mut args = vec!["vulnerabilities", "list"];
        args.extend_from_slice(extra);
        // The endpoint is unreachable, so a request would also fail; check the message names the problem.
        let output = run_plerion(&args, "test-key", "http://127.0.0.1:1");
        assert!(!output.status.success(), "{extra:?} should fail");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("epss") || stderr.contains("probability"), "{extra:?}: {stderr}");
    }
}
