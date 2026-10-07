use mockito::{Matcher, Server};
use plerion::api::{
    client::PlerionClient,
    endpoints::custom_checks::{self, ListCustomChecksParams},
    models::custom_checks::{CustomCheck, StartCustomCheckDryRunRequest},
};
use plerion::output::TableRenderable;

const CHECK_ID: &str = "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3";
const DRY_RUN_ID: &str = "2ea262bd-aa14-48c2-81ce-fa783cca7227";

fn mock_check() -> serde_json::Value {
    serde_json::json!({
        "customCheckId": CHECK_ID,
        "slug": "s3-no-public-acl",
        "organizationId": "org-1",
        "tenantId": "tenant-1",
        "title": "S3 buckets must not have public ACLs",
        "target": { "assetType": "AWS::S3::Bucket", "scope": "batch" },
        "additionalAssets": [{ "assetType": "AWS::IAM::Role" }],
        "defaults": { "severityLevel": "HIGH", "message": "Bucket is public", "informational": false },
        "type": "rego",
        "body": "package plerion\n\ndeny[msg] { msg := \"x\" }",
        "version": 3,
        "createdBy": "alice@example.com",
        "createdAt": "2026-01-07T13:37:23.388Z",
        "updatedBy": "bob@example.com",
        "updatedAt": "2026-02-07T13:37:46.308Z"
    })
}

fn input() -> serde_json::Value {
    serde_json::json!({
        "slug": "s3-no-public-acl",
        "title": "S3 buckets must not have public ACLs",
        "target": { "assetType": "AWS::S3::Bucket", "scope": "batch" },
        "defaults": { "severityLevel": "HIGH", "message": "Bucket is public", "informational": false },
        "type": "rego",
        "body": "package plerion"
    })
}

fn client(server: &Server) -> PlerionClient {
    PlerionClient::with_base_url(&server.url(), "key").unwrap()
}

#[tokio::test]
async fn test_list_custom_checks() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/custom-checks")
        .match_query(Matcher::Missing)
        .with_status(200)
        .with_body(serde_json::json!({ "items": [mock_check()], "nextCursor": "c2" }).to_string())
        .create_async()
        .await;

    let resp =
        custom_checks::list_custom_checks(&client(&server), &ListCustomChecksParams::default())
            .await
            .unwrap();
    mock.assert_async().await;
    assert_eq!(resp.items.len(), 1);
    assert_eq!(resp.next_cursor.as_deref(), Some("c2"));
    let check = &resp.items[0];
    assert_eq!(check.check_type.as_deref(), Some("rego"));
    assert_eq!(
        check.target.as_ref().unwrap().asset_type.as_deref(),
        Some("AWS::S3::Bucket")
    );
    assert_eq!(check.version, Some(3));
}

#[tokio::test]
async fn test_list_custom_checks_sends_camel_case_params() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/custom-checks")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("assetType".into(), "AWS::S3::Bucket".into()),
            Matcher::UrlEncoded("scope".into(), "all".into()),
            Matcher::UrlEncoded("limit".into(), "25".into()),
            Matcher::UrlEncoded("cursor".into(), "abc".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::json!({ "items": [] }).to_string())
        .create_async()
        .await;

    let params = ListCustomChecksParams {
        asset_type: Some("AWS::S3::Bucket".into()),
        scope: Some("all".into()),
        limit: Some(25),
        cursor: Some("abc".into()),
    };
    let resp = custom_checks::list_custom_checks(&client(&server), &params)
        .await
        .unwrap();
    mock.assert_async().await;
    assert!(resp.next_cursor.is_none());
}

#[tokio::test]
async fn test_get_custom_check() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "GET",
            format!("/v1/tenant/custom-checks/{CHECK_ID}").as_str(),
        )
        .with_status(200)
        .with_body(mock_check().to_string())
        .create_async()
        .await;

    let check = custom_checks::get_custom_check(&client(&server), CHECK_ID)
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(check.slug.as_deref(), Some("s3-no-public-acl"));
    assert_eq!(
        check.defaults.unwrap().severity_level.as_deref(),
        Some("HIGH")
    );
}

#[tokio::test]
async fn test_get_custom_check_escapes_the_id() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/custom-checks/a%2Fb")
        .with_status(200)
        .with_body(mock_check().to_string())
        .create_async()
        .await;

    custom_checks::get_custom_check(&client(&server), "a/b")
        .await
        .unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_custom_check_rejects_a_dot_segment() {
    let server = Server::new_async().await;
    let err = custom_checks::get_custom_check(&client(&server), "..")
        .await
        .unwrap_err();
    assert!(err.to_string().contains("not a valid custom check ID"), "{err}");
}

#[tokio::test]
async fn test_get_custom_check_not_found() {
    let mut server = Server::new_async().await;
    server
        .mock(
            "GET",
            format!("/v1/tenant/custom-checks/{CHECK_ID}").as_str(),
        )
        .with_status(404)
        .with_body(r#"{"message":"Not found"}"#)
        .create_async()
        .await;

    let err = custom_checks::get_custom_check(&client(&server), CHECK_ID)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("404"), "{err}");
}

#[tokio::test]
async fn test_create_custom_check() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/tenant/custom-checks")
        .match_body(Matcher::Json(input()))
        .with_status(200)
        .with_body(mock_check().to_string())
        .create_async()
        .await;

    let check = custom_checks::create_custom_check(&client(&server), &input())
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(check.custom_check_id.as_deref(), Some(CHECK_ID));
}

#[tokio::test]
async fn test_update_custom_check_uses_put() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "PUT",
            format!("/v1/tenant/custom-checks/{CHECK_ID}").as_str(),
        )
        .match_body(Matcher::Json(input()))
        .with_status(200)
        .with_body(mock_check().to_string())
        .create_async()
        .await;

    let check = custom_checks::update_custom_check(&client(&server), CHECK_ID, &input())
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(check.version, Some(3));
}

#[tokio::test]
async fn test_delete_custom_check() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "DELETE",
            format!("/v1/tenant/custom-checks/{CHECK_ID}").as_str(),
        )
        .with_status(202)
        .with_body(r#"{"accepted":true}"#)
        .create_async()
        .await;

    let resp = custom_checks::delete_custom_check(&client(&server), CHECK_ID)
        .await
        .unwrap();
    mock.assert_async().await;
    assert!(resp.accepted);
}

#[tokio::test]
async fn test_start_custom_check_dry_run() {
    let mut server = Server::new_async().await;
    let check = serde_json::json!({
        "customCheckId": CHECK_ID,
        "target": { "assetType": "AWS::S3::Bucket", "scope": "all" },
        "body": "package plerion"
    });
    let mock = server
        .mock("POST", "/v1/tenant/custom-check-dry-runs")
        .match_body(Matcher::Json(
            serde_json::json!({ "integrationId": "int-1", "check": check }),
        ))
        .with_status(202)
        .with_body(
            serde_json::json!({
                "dryRunId": DRY_RUN_ID,
                "status": "RUNNING",
                "pollUrl": format!("/v1/tenant/custom-check-dry-runs/{DRY_RUN_ID}")
            })
            .to_string(),
        )
        .create_async()
        .await;

    let body = StartCustomCheckDryRunRequest {
        integration_id: "int-1".into(),
        check,
    };
    let resp = custom_checks::start_custom_check_dry_run(&client(&server), &body)
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(resp.dry_run_id.as_deref(), Some(DRY_RUN_ID));
    assert_eq!(resp.status.as_deref(), Some("RUNNING"));
}

#[tokio::test]
async fn test_get_custom_check_dry_run_status() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "GET",
            format!("/v1/tenant/custom-check-dry-runs/{DRY_RUN_ID}").as_str(),
        )
        .with_status(200)
        .with_body(
            serde_json::json!({
                "dryRunId": DRY_RUN_ID,
                "status": "SUCCEEDED",
                "startedAt": "2026-01-07T13:37:23.388Z",
                "completedAt": "2026-01-07T13:38:23.388Z",
                "output": { "findings": [{ "assetId": "a-1", "status": "FAILED" }] }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let resp = custom_checks::get_custom_check_dry_run_status(&client(&server), DRY_RUN_ID)
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(resp.status.as_deref(), Some("SUCCEEDED"));
    assert_eq!(resp.output.unwrap()["findings"][0]["assetId"], "a-1");
}

#[test]
fn test_custom_check_table_row_is_scalar() {
    let check: CustomCheck = serde_json::from_value(mock_check()).unwrap();
    let row = check.row();
    assert_eq!(row.len(), CustomCheck::headers().len());
    assert_eq!(row[3], "AWS::S3::Bucket");
    assert_eq!(row[4], "batch");
    assert_eq!(row[5], "HIGH");
    assert_eq!(row[6], "3");
    assert!(row.iter().all(|c| !c.contains('\u{1b}')));
}

/// Absent optional fields stay absent, so `get` output can be sent back.
#[test]
fn test_custom_check_serialization_omits_absent_optional_fields() {
    let check: CustomCheck = serde_json::from_value(mock_check()).unwrap();
    let json = serde_json::to_value(&check).unwrap();
    assert!(json.get("description").is_none());
    assert!(json["defaults"].get("remediation").is_none());
    assert!(json["defaults"].get("meta").is_none());
    assert_eq!(json["type"], "rego");
}
