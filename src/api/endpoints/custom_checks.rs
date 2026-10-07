use crate::api::client::PlerionClient;
use crate::api::endpoints::path::segment;
use crate::api::models::custom_checks::{
    CustomCheck, CustomCheckDryRunStarted, CustomCheckDryRunStatus, CustomChecksResponse,
    DeleteCustomCheckResponse, StartCustomCheckDryRunRequest,
};
use crate::error::PlerionError;

#[derive(Debug, Default, Clone)]
pub struct ListCustomChecksParams {
    pub asset_type: Option<String>,
    pub scope: Option<String>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

pub async fn list_custom_checks(
    client: &PlerionClient,
    params: &ListCustomChecksParams,
) -> Result<CustomChecksResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/custom-checks");
    if let Some(v) = &params.asset_type {
        req = req.query(&[("assetType", v)]);
    }
    if let Some(v) = &params.scope {
        req = req.query(&[("scope", v)]);
    }
    if let Some(v) = params.limit {
        req = req.query(&[("limit", v)]);
    }
    if let Some(v) = &params.cursor {
        req = req.query(&[("cursor", v)]);
    }
    client.execute(req).await
}

pub async fn get_custom_check(
    client: &PlerionClient,
    id: &str,
) -> Result<CustomCheck, PlerionError> {
    client
        .execute(client.get(&format!("/v1/tenant/custom-checks/{}", segment(id, "custom check ID")?)))
        .await
}

pub async fn create_custom_check(
    client: &PlerionClient,
    body: &serde_json::Value,
) -> Result<CustomCheck, PlerionError> {
    client
        .execute(client.post("/v1/tenant/custom-checks").json(body))
        .await
}

pub async fn update_custom_check(
    client: &PlerionClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<CustomCheck, PlerionError> {
    client
        .execute(
            client
                .put(&format!("/v1/tenant/custom-checks/{}", segment(id, "custom check ID")?))
                .json(body),
        )
        .await
}

pub async fn delete_custom_check(
    client: &PlerionClient,
    id: &str,
) -> Result<DeleteCustomCheckResponse, PlerionError> {
    client
        .execute(client.delete(&format!("/v1/tenant/custom-checks/{}", segment(id, "custom check ID")?)))
        .await
}

pub async fn start_custom_check_dry_run(
    client: &PlerionClient,
    body: &StartCustomCheckDryRunRequest,
) -> Result<CustomCheckDryRunStarted, PlerionError> {
    client
        .execute(client.post("/v1/tenant/custom-check-dry-runs").json(body))
        .await
}

pub async fn get_custom_check_dry_run_status(
    client: &PlerionClient,
    dry_run_id: &str,
) -> Result<CustomCheckDryRunStatus, PlerionError> {
    client
        .execute(client.get(&format!(
            "/v1/tenant/custom-check-dry-runs/{}",
            segment(dry_run_id, "dry run ID")?
        )))
        .await
}
