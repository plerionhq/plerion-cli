use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use crate::api::client::PlerionClient;
use crate::api::models::access_grants::{
    AccessGrantExternalPrincipalsResponse, AccessGrantResponse, AccessGrantStatsResponse,
    AccessGrantsResponse, UpdateAccessGrantRequest,
};
use crate::error::PlerionError;

/// Every ASCII character `encodeURIComponent` escapes, so this client and the
/// Pleri client encode the same id identically. `encodeURIComponent` leaves only
/// `A-Z a-z 0-9 - _ . ! ~ * ' ( )` unescaped; everything else is listed here.
/// Kept honest by `test_encoding_matches_encode_uri_component`.
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

/// Percent-encodes an id for use as a single path segment.
///
/// A segment of only dots is rejected rather than encoded. It is an RFC 3986
/// dot-segment, and the URL layer resolves it before the request is sent, so
/// `..` would walk up a path level and silently call a different endpoint.
/// Encoding does not help: `%2E` is decoded and normalised the same way, so
/// there is no way to express a literal dot-segment. Refusing is the only
/// correct option, and no real grant id looks like this.
fn segment(id: &str) -> Result<String, PlerionError> {
    if !id.is_empty() && id.bytes().all(|b| b == b'.') {
        return Err(PlerionError::ApiError {
            status: 400,
            message: format!("'{id}' is not a valid grant ID"),
        });
    }
    Ok(utf8_percent_encode(id, PATH_SEGMENT).to_string())
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
    client.execute(client.get(&format!("/v1/tenant/aws/access-grants/{}", segment(id)?))).await
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
        .execute(client.patch(&format!("/v1/tenant/aws/access-grants/{}", segment(id)?)).json(&body))
        .await
}

#[cfg(test)]
mod tests {
    use super::segment;

    /// The only ASCII characters `encodeURIComponent` leaves unescaped.
    const UNRESERVED: &str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.!~*'()";

    /// Pins the comment on PATH_SEGMENT: escape exactly what encodeURIComponent
    /// escapes, so this client and the Pleri client encode an id identically.
    /// A dot-only segment is the one exception, covered separately below.
    #[test]
    fn test_encoding_matches_encode_uri_component() {
        for byte in 0x20u8..0x7f {
            let ch = char::from(byte);
            if ch == '.' {
                continue;
            }
            let input = ch.to_string();
            let escaped = segment(&input).unwrap() != input;
            assert_eq!(
                escaped,
                !UNRESERVED.contains(ch),
                "{ch:?} (0x{byte:02x}) escaped={escaped}, encoded as {}",
                segment(&input).unwrap()
            );
        }
    }

    #[test]
    fn test_control_characters_are_escaped() {
        assert_eq!(segment("\n").unwrap(), "%0A");
        assert_eq!(segment("\t").unwrap(), "%09");
    }

    #[test]
    fn test_a_uuid_passes_through_unchanged() {
        let id = "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3";
        assert_eq!(segment(id).unwrap(), id);
    }

    /// A dot-only segment cannot be expressed in a URL, so it is refused rather
    /// than silently resolving to a different endpoint.
    #[test]
    fn test_dot_segments_are_rejected() {
        assert!(segment(".").is_err());
        assert!(segment("..").is_err());
        assert!(segment("...").is_err());
        // Dots inside a real value are fine.
        assert_eq!(segment("a.b").unwrap(), "a.b");
        assert_eq!(segment(".hidden").unwrap(), ".hidden");
    }
}
