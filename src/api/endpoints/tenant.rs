use crate::api::client::PlerionClient;
use crate::api::models::tenant::{TenantResponse, TenantUsageResponse};
use crate::error::PlerionError;

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
