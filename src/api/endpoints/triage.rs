use crate::api::client::PlerionClient;
use crate::api::endpoints::path::segment;
use crate::api::models::triage::{
    DeleteTriageDecisionResponse, TriageDecisionResponse, TriageDecisionsResponse,
    WrittenTriageDecisionResponse,
};
use crate::error::PlerionError;

const DECISIONS_PATH: &str = "/v1/tenant/triage/decisions";

#[derive(Debug, Default, Clone)]
pub struct ListTriageDecisionsParams {
    pub per_page: Option<u32>,
    pub cursor: Option<String>,
}

fn decision_path(id: &str) -> Result<String, PlerionError> {
    Ok(format!("/v1/tenant/triage/decisions/{}", segment(id, "decision ID")?))
}

pub async fn list_triage_decisions(
    client: &PlerionClient,
    params: &ListTriageDecisionsParams,
) -> Result<TriageDecisionsResponse, PlerionError> {
    let mut req = client.get(DECISIONS_PATH);
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }
    if let Some(v) = &params.cursor { req = req.query(&[("cursor", v)]); }
    client.execute(req).await
}

pub async fn get_triage_decision(
    client: &PlerionClient,
    id: &str,
) -> Result<TriageDecisionResponse, PlerionError> {
    client.execute(client.get(&decision_path(id)?)).await
}

/// `body` takes subjectKind, rule, exemption and note.
pub async fn create_triage_decision(
    client: &PlerionClient,
    body: &serde_json::Value,
) -> Result<WrittenTriageDecisionResponse, PlerionError> {
    client.execute(client.post(DECISIONS_PATH).json(body)).await
}

/// `body` takes updatedAt, rule, exemption and note. The edit is refused with
/// a 409 when updatedAt is not the decision's current version.
pub async fn update_triage_decision(
    client: &PlerionClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<WrittenTriageDecisionResponse, PlerionError> {
    client.execute(client.put(&decision_path(id)?).json(body)).await
}

pub async fn delete_triage_decision(
    client: &PlerionClient,
    id: &str,
) -> Result<DeleteTriageDecisionResponse, PlerionError> {
    client.execute(client.delete(&decision_path(id)?)).await
}
