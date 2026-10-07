use crate::api::client::PlerionClient;
use crate::api::models::tenant::{
    HomeDashboardRequest, HomeDashboardResponse, TenantResponse, TenantUsageResponse,
};
use crate::error::PlerionError;

const HOME_DASHBOARD_PATH: &str = "/v1/tenant/preferences/home-dashboard";

pub async fn get_tenant(client: &PlerionClient) -> Result<TenantResponse, PlerionError> {
    client.execute(client.get("/v1/tenant")).await
}

pub async fn get_tenant_usage(
    client: &PlerionClient,
    date: Option<&str>,
) -> Result<TenantUsageResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/usage");
    if let Some(v) = date { req = req.query(&[("date", v)]); }
    client.execute(req).await
}

pub async fn get_home_dashboard(
    client: &PlerionClient,
) -> Result<HomeDashboardResponse, PlerionError> {
    client.execute(client.get(HOME_DASHBOARD_PATH)).await
}

pub async fn set_home_dashboard(
    client: &PlerionClient,
    home_report_id: &str,
) -> Result<HomeDashboardResponse, PlerionError> {
    let body = HomeDashboardRequest {
        home_report_id: home_report_id.to_string(),
    };
    client
        .execute(client.put(HOME_DASHBOARD_PATH).json(&body))
        .await
}

pub async fn clear_home_dashboard(client: &PlerionClient) -> Result<(), PlerionError> {
    client
        .execute_no_content(client.delete(HOME_DASHBOARD_PATH))
        .await
}

/// The API reference as an OpenAPI document, limited to what the calling key may call.
pub async fn discover_api_access(
    client: &PlerionClient,
) -> Result<serde_json::Value, PlerionError> {
    let req = client
        .get("/v1/tenant/openapi")
        .header("Accept", "application/json");
    client.execute(req).await
}
