use serde::{Deserialize, Serialize};

/// Typed envelopes for the AWS integration endpoints, used by the onboarding
/// orchestrator. The raw `plerion aws ...` subcommands keep returning
/// `serde_json::Value` so their passthrough output is unchanged.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIdResponse {
    pub data: Option<ExternalIdData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIdData {
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfnTemplateResponse {
    pub data: Option<CfnTemplateData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CfnTemplateData {
    #[serde(rename = "templateURL")]
    pub template_url: Option<String>,
    pub template_version: Option<String>,
    /// Not returned by the API today; picked up automatically if/when the
    /// tenant's Plerion-managed CWPP service account ID is exposed here.
    pub service_account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub data: Option<TokenData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub token: Option<String>,
}
