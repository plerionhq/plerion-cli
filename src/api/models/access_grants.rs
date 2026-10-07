use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;
use crate::api::models::findings::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessGrant {
    pub id: Option<String>,
    pub organization_id: Option<String>,
    pub tenant_id: Option<String>,
    pub integration_id: Option<String>,
    pub asset_id: Option<String>,
    pub resource_type: Option<String>,
    pub aws_account_id: Option<String>,
    pub region: Option<String>,
    pub asset_name: Option<String>,
    pub service: Option<String>,
    pub grant_type: Option<String>,
    pub mechanism: Option<String>,
    pub principal: Option<String>,
    pub principal_type: Option<String>,
    pub principal_label: Option<String>,
    pub principal_detail: Option<String>,
    pub principal_account_id: Option<String>,
    pub grant_scope: Option<String>,
    pub grant_origin: Option<String>,
    pub allowed_actions: Option<Vec<String>>,
    pub allowed_not_actions: Option<Vec<String>>,
    pub conditions: Option<serde_json::Value>,
    pub blocked_by_rcp: Option<bool>,
    pub rcp_status: Option<String>,
    pub has_runtime_conditions: Option<bool>,
    pub trust_status: Option<String>,
    pub trusted_until: Option<String>,
    pub trust_lapse_reason: Option<String>,
    pub grantee: Option<String>,
    pub review_decision: Option<String>,
    pub review_comment: Option<String>,
    pub next_review_at: Option<String>,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<String>,
    pub review_history: Option<Vec<AccessGrantReview>>,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessGrantReview {
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<String>,
    pub decision: Option<String>,
    pub grantee: Option<String>,
    pub comment: Option<String>,
    pub next_review_at: Option<String>,
    pub trusted_until: Option<String>,
    pub trust_lapse_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessGrantsResponse {
    pub data: Vec<AccessGrant>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessGrantResponse {
    pub data: AccessGrant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessGrantStats {
    pub total: Option<u32>,
    pub external: Option<u32>,
    pub untrusted_external: Option<u32>,
    pub cross_org: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessGrantStatsResponse {
    pub data: AccessGrantStats,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessGrantExternalPrincipal {
    pub principal: Option<String>,
    pub principal_type: Option<String>,
    pub principal_account_id: Option<String>,
    pub grant_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessGrantExternalPrincipalsResponse {
    pub data: Vec<AccessGrantExternalPrincipal>,
    pub meta: PaginationMeta,
}

/// Review fields on a PATCH. Every field is optional: omitted leaves the value
/// untouched, explicit null clears it, and a body with none of them is a 400.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccessGrantRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_decision: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_review_at: Option<serde_json::Value>,
}

impl UpdateAccessGrantRequest {
    pub fn is_empty(&self) -> bool {
        self.review_decision.is_none()
            && self.review_comment.is_none()
            && self.grantee.is_none()
            && self.next_review_at.is_none()
    }
}

fn bool_cell(value: Option<bool>) -> String {
    value.map(|b| b.to_string()).unwrap_or_default()
}

impl TableRenderable for AccessGrant {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID", "RESOURCE TYPE", "ASSET NAME", "AWS ACCOUNT ID", "REGION", "SERVICE",
            "MECHANISM", "PRINCIPAL", "PRINCIPAL TYPE", "SCOPE", "ORIGIN", "TRUST STATUS",
            "BLOCKED BY RCP", "REVIEW DECISION", "GRANTEE", "NEXT REVIEW AT",
            "REVIEWED BY", "REVIEWED AT", "FIRST OBSERVED AT", "LAST OBSERVED AT",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.resource_type.clone().unwrap_or_default(),
            self.asset_name.clone().unwrap_or_default(),
            self.aws_account_id.clone().unwrap_or_default(),
            self.region.clone().unwrap_or_default(),
            self.service.clone().unwrap_or_default(),
            self.mechanism.clone().unwrap_or_default(),
            self.principal.clone().unwrap_or_default(),
            self.principal_type.clone().unwrap_or_default(),
            self.grant_scope.clone().unwrap_or_default(),
            self.grant_origin.clone().unwrap_or_default(),
            self.trust_status.clone().unwrap_or_default(),
            bool_cell(self.blocked_by_rcp),
            self.review_decision.clone().unwrap_or_default(),
            self.grantee.clone().unwrap_or_default(),
            self.next_review_at.clone().unwrap_or_default(),
            self.reviewed_by.clone().unwrap_or_default(),
            self.reviewed_at.clone().unwrap_or_default(),
            self.first_observed_at.clone().unwrap_or_default(),
            self.last_observed_at.clone().unwrap_or_default(),
        ]
    }
}

impl TableRenderable for AccessGrantStats {
    fn headers() -> Vec<&'static str> {
        vec!["TOTAL", "EXTERNAL", "UNTRUSTED EXTERNAL", "CROSS ORG"]
    }

    fn row(&self) -> Vec<String> {
        let num = |v: Option<u32>| v.map(|n| n.to_string()).unwrap_or_default();
        vec![
            num(self.total),
            num(self.external),
            num(self.untrusted_external),
            num(self.cross_org),
        ]
    }
}

impl TableRenderable for AccessGrantExternalPrincipal {
    fn headers() -> Vec<&'static str> {
        vec!["PRINCIPAL", "PRINCIPAL TYPE", "PRINCIPAL ACCOUNT ID", "GRANT COUNT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.principal.clone().unwrap_or_default(),
            self.principal_type.clone().unwrap_or_default(),
            self.principal_account_id.clone().unwrap_or_default(),
            self.grant_count.map(|n| n.to_string()).unwrap_or_default(),
        ]
    }
}
