use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use crate::api::client::PlerionClient;
use crate::api::models::access_grants::{
    AccessGrantExternalPrincipalsResponse, AccessGrantResponse, AccessGrantStatsResponse,
    AccessGrantsResponse, UpdateAccessGrantRequest,
};
use crate::error::PlerionError;

/// Matches what encodeURIComponent escapes, so an id can never inject a path
/// segment or query. The Pleri client encodes the same id the same way.
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'#')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b'%')
    .add(b'=')
    .add(b'&')
    .add(b'+');

fn segment(id: &str) -> String {
    utf8_percent_encode(id, PATH_SEGMENT).to_string()
}

#[derive(Debug, Default, Clone)]
pub struct ListAccessGrantsParams {
    pub ids: Option<String>,
    pub grant_origin: Option<String>,
    pub trust_statuses: Option<String>,
    pub grant_scopes: Option<String>,
    pub mechanisms: Option<String>,
    pub principal_types: Option<String>,
    pub resource_types: Option<String>,
    pub integration_ids: Option<String>,
    pub aws_account_ids: Option<String>,
    pub principal_account_ids: Option<String>,
    pub asset_name: Option<String>,
    pub principal: Option<String>,
    pub search: Option<String>,
    pub review_decisions: Option<String>,
    pub grant_owner: Option<String>,
    pub next_review_at_end: Option<String>,
    pub cursor: Option<String>,
    pub per_page: Option<u32>,
}

pub async fn list_access_grants(
    client: &PlerionClient,
    params: &ListAccessGrantsParams,
) -> Result<AccessGrantsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/aws/access-grants");

    if let Some(v) = &params.ids { req = req.query(&[("ids", v)]); }
    if let Some(v) = &params.grant_origin { req = req.query(&[("grantOrigin", v)]); }
    if let Some(v) = &params.trust_statuses { req = req.query(&[("trustStatuses", v)]); }
    if let Some(v) = &params.grant_scopes { req = req.query(&[("grantScopes", v)]); }
    if let Some(v) = &params.mechanisms { req = req.query(&[("mechanisms", v)]); }
    if let Some(v) = &params.principal_types { req = req.query(&[("principalTypes", v)]); }
    if let Some(v) = &params.resource_types { req = req.query(&[("resourceTypes", v)]); }
    if let Some(v) = &params.integration_ids { req = req.query(&[("integrationIds", v)]); }
    if let Some(v) = &params.aws_account_ids { req = req.query(&[("awsAccountIds", v)]); }
    if let Some(v) = &params.principal_account_ids { req = req.query(&[("principalAccountIds", v)]); }
    if let Some(v) = &params.asset_name { req = req.query(&[("assetName", v)]); }
    if let Some(v) = &params.principal { req = req.query(&[("principal", v)]); }
    if let Some(v) = &params.search { req = req.query(&[("search", v)]); }
    if let Some(v) = &params.review_decisions { req = req.query(&[("reviewDecisions", v)]); }
    if let Some(v) = &params.grant_owner { req = req.query(&[("grantOwner", v)]); }
    if let Some(v) = &params.next_review_at_end { req = req.query(&[("nextReviewAtEnd", v)]); }
    if let Some(v) = &params.cursor { req = req.query(&[("cursor", v)]); }
    if let Some(v) = params.per_page { req = req.query(&[("perPage", v)]); }

    client.execute(req).await
}

pub async fn get_access_grant(
    client: &PlerionClient,
    id: &str,
) -> Result<AccessGrantResponse, PlerionError> {
    client.execute(client.get(&format!("/v1/tenant/aws/access-grants/{}", segment(id)))).await
}

pub async fn get_access_grant_stats(
    client: &PlerionClient,
) -> Result<AccessGrantStatsResponse, PlerionError> {
    client.execute(client.get("/v1/tenant/aws/access-grants/stats")).await
}

pub async fn list_access_grant_external_principals(
    client: &PlerionClient,
) -> Result<AccessGrantExternalPrincipalsResponse, PlerionError> {
    client
        .execute(client.get("/v1/tenant/aws/access-grants/external-principals"))
        .await
}

pub async fn update_access_grant(
    client: &PlerionClient,
    id: &str,
    body: UpdateAccessGrantRequest,
) -> Result<AccessGrantResponse, PlerionError> {
    client
        .execute(client.patch(&format!("/v1/tenant/aws/access-grants/{}", segment(id))).json(&body))
        .await
}
