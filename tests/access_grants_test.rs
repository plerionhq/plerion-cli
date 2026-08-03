use mockito::Server;
use plerion::api::{
    client::PlerionClient,
    endpoints::access_grants::{
        get_access_grant, get_access_grant_stats, list_access_grant_external_principals,
        list_access_grants, update_access_grant, ListAccessGrantsParams,
    },
    models::access_grants::UpdateAccessGrantRequest,
};

fn grant_json() -> serde_json::Value {
    serde_json::json!({
        "id": "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3",
        "organizationId": "org-1",
        "tenantId": "tenant-1",
        "integrationId": "458511a1-9bc2-4fce-97a0-0e3139588e6e",
        "assetId": "prn:assets:458511a1:aws:s3:bucket:ap-southeast-2:acme-prod-exports",
        "resourceType": "AWS::S3::Bucket",
        "awsAccountId": "111122223333",
        "region": "ap-southeast-2",
        "assetName": "acme-prod-exports",
        "service": "AWS::S3",
        "grantType": "resource_policy_statement",
        "mechanism": "resource_policy",
        "principal": "arn:aws:iam::444455556666:root",
        "principalType": "aws_account",
        "principalLabel": "Vendor - External",
        "principalDetail": null,
        "principalAccountId": "444455556666",
        "grantScope": "cross-org",
        "grantOrigin": "external",
        "allowedActions": ["s3:GetObject", "s3:ListBucket"],
        "allowedNotActions": null,
        "conditions": { "StringEquals": { "aws:PrincipalOrgID": "o-abc123" } },
        "blockedByRcp": false,
        "rcpStatus": "not-applicable",
        "hasRuntimeConditions": true,
        "trustStatus": "untrusted",
        "grantOwner": "platform-team",
        "reviewDecision": "keep",
        "reviewComment": "Vendor export feed",
        "nextReviewAt": "2027-01-31T00:00:00.000Z",
        "reviewedBy": "ci-audit-key (API key)",
        "reviewedAt": "2026-07-20T05:41:09.000Z",
        "reviewHistory": [
            {
                "reviewedBy": "auditor@acme.com",
                "reviewedAt": "2026-07-20T05:41:09.000Z",
                "decision": "keep",
                "owner": "platform-team"
            }
        ],
        "firstObservedAt": "2026-05-14T02:11:47.000Z",
        "lastObservedAt": "2026-07-28T01:03:12.000Z"
    })
}

#[tokio::test]
async fn test_list_access_grants_deserializes_every_field() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [grant_json()],
        "meta": { "perPage": 50, "total": 412, "cursor": "eyJpZCI6IjZiMzFkODRhIn0=" }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/aws/access-grants")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_access_grants(&client, &ListAccessGrantsParams::default())
        .await
        .unwrap();

    assert_eq!(resp.data.len(), 1);
    let g = &resp.data[0];
    assert_eq!(g.resource_type.as_deref(), Some("AWS::S3::Bucket"));
    assert_eq!(g.aws_account_id.as_deref(), Some("111122223333"));
    assert_eq!(g.grant_origin.as_deref(), Some("external"));
    assert_eq!(g.trust_status.as_deref(), Some("untrusted"));
    assert_eq!(g.blocked_by_rcp, Some(false));
    assert_eq!(g.allowed_actions.as_ref().unwrap().len(), 2);
    assert_eq!(g.review_history.as_ref().unwrap()[0].decision.as_deref(), Some("keep"));
    assert_eq!(resp.meta.total, Some(412));
}

#[tokio::test]
async fn test_list_access_grants_sends_every_filter() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/aws/access-grants")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("ids".into(), "g-1".into()),
            mockito::Matcher::UrlEncoded("grantOrigin".into(), "external".into()),
            mockito::Matcher::UrlEncoded("trustStatuses".into(), "untrusted".into()),
            mockito::Matcher::UrlEncoded("grantScopes".into(), "cross-org".into()),
            mockito::Matcher::UrlEncoded("mechanisms".into(), "resource_policy".into()),
            mockito::Matcher::UrlEncoded("principalTypes".into(), "aws_account".into()),
            mockito::Matcher::UrlEncoded("resourceTypes".into(), "AWS::S3::Bucket".into()),
            mockito::Matcher::UrlEncoded("integrationIds".into(), "int-1".into()),
            mockito::Matcher::UrlEncoded("awsAccountIds".into(), "111122223333".into()),
            mockito::Matcher::UrlEncoded("principalAccountIds".into(), "444455556666".into()),
            mockito::Matcher::UrlEncoded("assetName".into(), "exports".into()),
            mockito::Matcher::UrlEncoded("principal".into(), "root".into()),
            mockito::Matcher::UrlEncoded("search".into(), "acme".into()),
            mockito::Matcher::UrlEncoded("reviewDecisions".into(), "keep".into()),
            mockito::Matcher::UrlEncoded("grantOwner".into(), "platform-team".into()),
            mockito::Matcher::UrlEncoded("nextReviewAtEnd".into(), "2026-12-31T00:00:00Z".into()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            serde_json::json!({ "data": [], "meta": { "perPage": 50, "total": 0, "cursor": null } })
                .to_string(),
        )
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListAccessGrantsParams {
        ids: Some("g-1".into()),
        grant_origin: Some("external".into()),
        trust_statuses: Some("untrusted".into()),
        grant_scopes: Some("cross-org".into()),
        mechanisms: Some("resource_policy".into()),
        principal_types: Some("aws_account".into()),
        resource_types: Some("AWS::S3::Bucket".into()),
        integration_ids: Some("int-1".into()),
        aws_account_ids: Some("111122223333".into()),
        principal_account_ids: Some("444455556666".into()),
        asset_name: Some("exports".into()),
        principal: Some("root".into()),
        search: Some("acme".into()),
        review_decisions: Some("keep".into()),
        grant_owner: Some("platform-team".into()),
        next_review_at_end: Some("2026-12-31T00:00:00Z".into()),
        ..Default::default()
    };
    let resp = list_access_grants(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_access_grant() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/aws/access-grants/0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::json!({ "data": grant_json() }).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = get_access_grant(&client, "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3")
        .await
        .unwrap();
    assert_eq!(resp.data.asset_name.as_deref(), Some("acme-prod-exports"));
}

#[tokio::test]
async fn test_get_access_grant_stats() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/aws/access-grants/stats")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            serde_json::json!({
                "data": { "total": 1284, "external": 412, "untrustedExternal": 37, "crossOrg": 289 }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = get_access_grant_stats(&client).await.unwrap();
    assert_eq!(resp.data.total, Some(1284));
    assert_eq!(resp.data.untrusted_external, Some(37));
    assert_eq!(resp.data.cross_org, Some(289));
}

#[tokio::test]
async fn test_list_external_principals() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/aws/access-grants/external-principals")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            serde_json::json!({
                "data": [
                    { "principal": "arn:aws:iam::444455556666:root", "principalType": "aws_account",
                      "principalAccountId": "444455556666", "grantCount": 42 },
                    { "principal": "*", "principalType": "public",
                      "principalAccountId": null, "grantCount": 7 }
                ],
                "meta": { "total": 214 }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_access_grant_external_principals(&client).await.unwrap();
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].grant_count, Some(42));
    assert_eq!(resp.data[1].principal_account_id, None);
    assert_eq!(resp.meta.total, Some(214));
}

#[tokio::test]
async fn test_update_access_grant_sends_only_supplied_fields() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/tenant/aws/access-grants/0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "reviewDecision": "review_later"
        })))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::json!({ "data": grant_json() }).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let body = UpdateAccessGrantRequest {
        review_decision: Some(serde_json::Value::String("review_later".into())),
        ..Default::default()
    };
    update_access_grant(&client, "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3", body)
        .await
        .unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_access_grant_sends_explicit_null_to_clear() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/tenant/aws/access-grants/g-1")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "reviewComment": null
        })))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(serde_json::json!({ "data": grant_json() }).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let body = UpdateAccessGrantRequest {
        review_comment: Some(serde_json::Value::Null),
        ..Default::default()
    };
    update_access_grant(&client, "g-1", body).await.unwrap();
    mock.assert_async().await;
}

#[test]
fn test_update_request_is_empty_detects_a_no_op_body() {
    assert!(UpdateAccessGrantRequest::default().is_empty());
    let with_owner = UpdateAccessGrantRequest {
        grant_owner: Some(serde_json::Value::String("team".into())),
        ..Default::default()
    };
    assert!(!with_owner.is_empty());
    // Clearing a field is a real change, not an empty body.
    let clearing = UpdateAccessGrantRequest {
        grant_owner: Some(serde_json::Value::Null),
        ..Default::default()
    };
    assert!(!clearing.is_empty());
}
