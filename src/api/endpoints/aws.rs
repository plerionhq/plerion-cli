use crate::api::client::PlerionClient;
use crate::error::PlerionError;

pub async fn get_external_id(client: &PlerionClient) -> Result<serde_json::Value, PlerionError> {
    client.execute(client.get("/v1/tenant/external-id")).await
}

pub async fn get_cloudformation_template(
    client: &PlerionClient,
    template_type: &str,
) -> Result<serde_json::Value, PlerionError> {
    let req = client.get("/v1/tenant/cloudformation-templates")
        .query(&[("type", template_type)]);
    client.execute(req).await
}

pub async fn generate_token(
    client: &PlerionClient,
    integration_id: &str,
) -> Result<serde_json::Value, PlerionError> {
    client
        .execute(
            client
                .post("/v1/tenant/integrations/token")
                .json(&serde_json::json!({ "integrationId": integration_id })),
        )
        .await
}
