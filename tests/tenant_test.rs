use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::tenant};
use plerion::output::TableRenderable;

#[tokio::test]
async fn test_get_tenant() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "tenantId": "tid-123",
            "organizationId": "oid-456",
            "name": "Test Tenant",
            "createdAt": "2023-01-01T00:00:00Z",
            "updatedAt": "2023-06-01T00:00:00Z",
            "riskScore": 7.5
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::get_tenant(&client).await.unwrap();
    assert_eq!(resp.data.tenant_id, "tid-123");
    assert_eq!(resp.data.name, "Test Tenant");
    assert_eq!(resp.data.risk_score, Some(7.5));
}

#[tokio::test]
async fn test_get_tenant_usage() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": { "assets": 100, "integrations": 5 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/usage")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::get_tenant_usage(&client, None).await.unwrap();
    assert_eq!(resp.data.assets, Some(100));
    assert_eq!(resp.data.integrations, Some(5));
}

#[tokio::test]
async fn test_get_tenant_usage_with_date() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": { "assets": 100, "integrations": 5 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/usage")
        .match_query(mockito::Matcher::UrlEncoded("date".to_string(), "2025-03-01".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::get_tenant_usage(&client, Some("2025-03-01")).await.unwrap();
    assert_eq!(resp.data.assets, Some(100));
    mock.assert_async().await;
}

#[test]
fn test_tenant_usage_table_renderable() {
    use plerion::api::models::tenant::TenantUsageData;

    let usage = TenantUsageData {
        assets: Some(5000),
        integrations: Some(3),
    };

    let headers = TenantUsageData::headers();
    assert!(headers.contains(&"ASSETS"));
    assert!(headers.contains(&"INTEGRATIONS"));

    let row = usage.row();
    assert_eq!(row[0], "5000");
    assert_eq!(row[1], "3");
}

#[test]
fn test_tenant_usage_table_renderable_nones() {
    use plerion::api::models::tenant::TenantUsageData;

    let usage = TenantUsageData {
        assets: None,
        integrations: None,
    };

    let row = usage.row();
    assert_eq!(row[0], "");
    assert_eq!(row[1], "");
}

#[tokio::test]
async fn test_get_home_dashboard() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": { "tenantId": "t-1", "homeReportId": "r-1", "updatedAt": "2026-10-01T00:00:00Z" }
    });
    let mock = server
        .mock("GET", "/v1/tenant/preferences/home-dashboard")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::get_home_dashboard(&client).await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.row(), vec!["t-1", "r-1", "2026-10-01T00:00:00Z"]);
}

#[tokio::test]
async fn test_get_home_dashboard_not_set() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/preferences/home-dashboard")
        .with_status(404)
        .with_body(r#"{"errors":[{"code":"PreferenceNotFound"}]}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let err = tenant::get_home_dashboard(&client).await.unwrap_err();
    assert!(err.to_string().contains("PreferenceNotFound"));
}

#[tokio::test]
async fn test_set_home_dashboard() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "tenantId": "t-1", "homeReportId": "r-2" } });
    let mock = server
        .mock("PUT", "/v1/tenant/preferences/home-dashboard")
        .match_body(mockito::Matcher::Json(
            serde_json::json!({ "homeReportId": "r-2" }),
        ))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = tenant::set_home_dashboard(&client, "r-2").await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.home_report_id.as_deref(), Some("r-2"));
}

#[tokio::test]
async fn test_clear_home_dashboard() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/tenant/preferences/home-dashboard")
        .with_status(204)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    tenant::clear_home_dashboard(&client).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_clear_home_dashboard_not_set() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("DELETE", "/v1/tenant/preferences/home-dashboard")
        .with_status(404)
        .with_body(r#"{"errors":[{"code":"PreferenceNotFound"}]}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let err = tenant::clear_home_dashboard(&client).await.unwrap_err();
    assert!(err.to_string().contains("404"));
}

fn openapi_doc() -> serde_json::Value {
    serde_json::json!({
        "openapi": "3.1.0",
        "x-plerion-role": { "name": "Tenant read-only" },
        "paths": {
            "/v1/tenant/profiles": {
                "parameters": [],
                "get": { "operationId": "listProfiles", "summary": "List" }
            },
            "/v1/tenant/preferences/home-dashboard": {
                "delete": { "operationId": "clearTenantHomeDashboard", "summary": "Clear tenant default" },
                "get": { "operationId": "getTenantHomeDashboard" }
            }
        }
    })
}

#[tokio::test]
async fn test_discover_api_access() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/openapi")
        .match_header("Accept", "application/json")
        .with_status(200)
        .with_body(openapi_doc().to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let doc = tenant::discover_api_access(&client).await.unwrap();
    mock.assert_async().await;
    assert_eq!(doc["x-plerion-role"]["name"], "Tenant read-only");
}

#[test]
fn test_api_operations_from_openapi() {
    use plerion::api::models::tenant::ApiOperation;
    let ops = ApiOperation::from_openapi(&openapi_doc());
    let rows: Vec<Vec<String>> = ops.iter().map(|o| o.row()).collect();
    assert_eq!(
        rows,
        vec![
            vec![
                "GET",
                "/v1/tenant/preferences/home-dashboard",
                "getTenantHomeDashboard",
                ""
            ],
            vec![
                "DELETE",
                "/v1/tenant/preferences/home-dashboard",
                "clearTenantHomeDashboard",
                "Clear tenant default"
            ],
            vec!["GET", "/v1/tenant/profiles", "listProfiles", "List"],
        ]
    );
    assert!(ApiOperation::from_openapi(&serde_json::json!({})).is_empty());
}
