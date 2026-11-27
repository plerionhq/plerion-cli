use serde::{Deserialize, Serialize};

use crate::api::models::findings::PaginationMeta;
use crate::output::TableRenderable;

/// A triage decision: a rule saying which findings or vulnerabilities it
/// covers, and what to do with them.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriageDecision {
    pub id: Option<String>,
    pub subject_kind: Option<String>,
    pub rule: Option<TriageRule>,
    pub exemption: Option<TriageExemption>,
    pub outcome: Option<String>,
    pub note: Option<String>,
    pub status: Option<String>,
    pub is_in_force: Option<bool>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriageRule {
    pub version: Option<i64>,
    #[serde(default)]
    pub conditions: Vec<TriageCondition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriageCondition {
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    pub comparison: Option<String>,
    #[serde(default)]
    pub values: Vec<serde_json::Value>,
    /// True when some values name resources outside the key's integrations
    /// and were left out. Absent on every other condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_values: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriageExemption {
    pub reason: Option<String>,
    pub until_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriageDecisionsResponse {
    pub data: Vec<TriageDecision>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriageDecisionResponse {
    pub data: TriageDecision,
}

/// A decision as a create or edit saved it.
#[derive(Debug, Serialize, Deserialize)]
pub struct WrittenTriageDecisionResponse {
    pub data: TriageDecision,
    pub meta: Option<WrittenTriageDecisionMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WrittenTriageDecisionMeta {
    /// Whether the change was queued to be applied to findings and vulnerabilities.
    pub is_apply_queued: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedTriageDecision {
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTriageDecisionResponse {
    pub data: DeletedTriageDecision,
}

fn value_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

impl TableRenderable for TriageDecision {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "SUBJECT KIND", "OUTCOME", "REASON", "UNTIL AT", "STATUS", "IN FORCE",
            "CONDITIONS", "NOTE", "CREATED BY", "UPDATED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        let exemption = self.exemption.as_ref();
        vec![
            self.id.clone().unwrap_or_default(),
            self.subject_kind.clone().unwrap_or_default(),
            self.outcome.clone().unwrap_or_default(),
            exemption.and_then(|e| e.reason.clone()).unwrap_or_default(),
            exemption.and_then(|e| e.until_at.clone()).unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.is_in_force.map(|b| b.to_string()).unwrap_or_default(),
            self.rule.as_ref().map(|r| r.conditions.len().to_string()).unwrap_or_default(),
            self.note.clone().unwrap_or_default(),
            self.created_by.clone().unwrap_or_default(),
            self.updated_at.clone().unwrap_or_default(),
        ]
    }
}

impl TableRenderable for TriageCondition {
    fn headers() -> Vec<&'static str> {
        vec!["FIELD", "KEY", "COMPARISON", "VALUES", "HAS HIDDEN VALUES"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.field.clone().unwrap_or_default(),
            self.key.clone().unwrap_or_default(),
            self.comparison.clone().unwrap_or_default(),
            self.values.iter().map(value_text).collect::<Vec<_>>().join(", "),
            self.has_hidden_values.map(|b| b.to_string()).unwrap_or_default(),
        ]
    }
}
