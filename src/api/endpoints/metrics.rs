use crate::api::client::PlerionClient;
use crate::api::models::metrics::{QueryMetricsRequest, QueryMetricsResponse};
use crate::error::PlerionError;

pub async fn query_metrics(
    client: &PlerionClient,
    body: &QueryMetricsRequest,
) -> Result<QueryMetricsResponse, PlerionError> {
    client
        .execute(client.post("/v1/tenant/metrics").json(body))
        .await
}
