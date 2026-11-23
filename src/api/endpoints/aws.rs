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

/// Generate a temporary integration token.
///
/// With `Some(integration_id)` the token is scoped to an existing integration
/// (the `plerion aws generate-token` behavior). With `None` the request is a
/// bare POST, which mints the onboarding token used to register a NEW AWS
/// account integration via CloudFormation.
pub async fn generate_token(
    client: &PlerionClient,
    integration_id: Option<&str>,
) -> Result<serde_json::Value, PlerionError> {
    let req = client.post("/v1/tenant/integrations/token");
    let req = match integration_id {
        Some(id) => req.json(&serde_json::json!({ "integrationId": id })),
        None => req,
    };
    client.execute(req).await
}
