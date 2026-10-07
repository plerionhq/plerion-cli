use mockito::{Matcher, Server};
use plerion::api::models::profiles::{
    Profile, ReplaceDetectionExemptionsRequest, StoredDetectionExemption,
};
use plerion::api::{client::PlerionClient, endpoints::profiles};
use plerion::cli::profiles::parse_exemptions;
use plerion::output::TableRenderable;
use serde_json::json;

const EXEMPTIONS_PATH: &str =
    "/v1/tenant/profiles/default/detection-settings/PLERION-AWS-16/exemptions";

fn exemption_set(version: serde_json::Value) -> serde_json::Value {
    json!({
        "data": {
            "profileId": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
            "detectionId": "PLERION-AWS-16",
            "provider": "AWS",
            "supportsExemptions": true,
            "supportedExemptionTypes": ["NAME_EXEMPTION", "TAG_EXEMPTION"],
            "exemptions": [
                { "type": "NAME_EXEMPTION", "reason": "ACCEPTED_RISK", "condition": "starts-with", "value": "acme-sandbox-" }
            ],
            "version": version
        }
    })
}

#[tokio::test]
async fn test_list_profiles() {
    let mut server = Server::new_async().await;
    let body = json!({
        "data": [
            { "profileId": "p-1", "name": "Default", "isDefault": true, "integrations": [] },
            {
                "profileId": "p-2", "name": "Production", "description": "Prod accounts", "isDefault": false,
                "integrations": [ { "integrationId": "i-1", "name": "acme-prod" }, { "integrationId": "i-2", "name": "acme-dr" } ]
            }
        ],
        "meta": { "total": 2 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/profiles")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = profiles::list_profiles(&client).await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.meta.unwrap().total, Some(2));
    assert_eq!(
        resp.data[1].row(),
        vec!["p-2", "Production", "false", "2", "Prod accounts"]
    );
    assert_eq!(Profile::headers().len(), resp.data[1].row().len());
}

#[tokio::test]
async fn test_get_detection_exemptions() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", EXEMPTIONS_PATH)
        .with_status(200)
        .with_body(exemption_set(json!("v-123")).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = profiles::get_detection_exemptions(&client, "default", "PLERION-AWS-16")
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.version.as_deref(), Some("v-123"));
    assert!(resp.data.supports_exemptions);
    assert_eq!(resp.data.exemptions.len(), 1);
    assert_eq!(resp.data.row()[5], "1");
}

#[tokio::test]
async fn test_get_detection_exemptions_null_version() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", EXEMPTIONS_PATH)
        .with_status(200)
        .with_body(exemption_set(serde_json::Value::Null).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = profiles::get_detection_exemptions(&client, "default", "PLERION-AWS-16")
        .await
        .unwrap();
    assert_eq!(resp.data.version, None);
}

#[tokio::test]
async fn test_get_detection_exemptions_escapes_path_segments() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "GET",
            "/v1/tenant/profiles/a%2Fb/detection-settings/X%20Y/exemptions",
        )
        .with_status(200)
        .with_body(exemption_set(json!("v")).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    profiles::get_detection_exemptions(&client, "a/b", "X Y")
        .await
        .unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_detection_exemptions_rejects_dot_segment() {
    let client = PlerionClient::with_base_url("http://127.0.0.1:1", "key").unwrap();
    let err = profiles::get_detection_exemptions(&client, "..", "PLERION-AWS-16")
        .await
        .unwrap_err();
    assert_eq!(err.to_string(), "'..' is not a valid profile ID");
}

#[tokio::test]
async fn test_replace_detection_exemptions_sends_if_match() {
    let mut server = Server::new_async().await;
    let entry = json!({ "type": "REGION_EXEMPTION", "regions": ["us-west-2"] });
    let mock = server
        .mock("PUT", EXEMPTIONS_PATH)
        .match_header("If-Match", "v-123")
        .match_body(Matcher::Json(json!({ "exemptions": [entry.clone()] })))
        .with_status(200)
        .with_body(exemption_set(json!("v-124")).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let body = ReplaceDetectionExemptionsRequest {
        exemptions: vec![entry],
    };
    let resp = profiles::replace_detection_exemptions(
        &client,
        "default",
        "PLERION-AWS-16",
        Some("v-123"),
        &body,
    )
    .await
    .unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.version.as_deref(), Some("v-124"));
}

#[tokio::test]
async fn test_replace_detection_exemptions_without_if_match() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", EXEMPTIONS_PATH)
        .match_header("If-Match", Matcher::Missing)
        .match_body(Matcher::Json(json!({ "exemptions": [] })))
        .with_status(200)
        .with_body(exemption_set(json!("v-1")).to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let body = ReplaceDetectionExemptionsRequest { exemptions: vec![] };
    profiles::replace_detection_exemptions(&client, "default", "PLERION-AWS-16", None, &body)
        .await
        .unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_replace_detection_exemptions_version_mismatch() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("PUT", EXEMPTIONS_PATH)
        .with_status(412)
        .with_body(r#"{"errors":[{"code":"VersionMismatch"}]}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let body = ReplaceDetectionExemptionsRequest { exemptions: vec![] };
    let err = profiles::replace_detection_exemptions(
        &client,
        "default",
        "PLERION-AWS-16",
        Some("stale"),
        &body,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("412"));
    assert!(err.to_string().contains("VersionMismatch"));
}

#[test]
fn test_stored_exemption_rows_summarise_each_type() {
    let row = |v: serde_json::Value| StoredDetectionExemption(v).row();
    assert_eq!(
        row(
            json!({ "type": "NAME_EXEMPTION", "reason": "ACCEPTED_RISK", "condition": "starts-with", "value": "acme-" })
        ),
        vec![
            "NAME_EXEMPTION",
            "ACCEPTED_RISK",
            "starts-with \"acme-\"",
            "",
            ""
        ]
    );
    assert_eq!(
        row(
            json!({ "type": "TAG_EXEMPTION", "tagKey": { "option": "equals", "value": "env" }, "tagValue": { "option": "any", "value": "" } })
        )[2],
        "tag \"env\" with any value"
    );
    assert_eq!(
        row(
            json!({ "type": "TAG_EXEMPTION", "tagKey": { "option": "equals", "value": "env" }, "tagValue": { "option": "equals", "value": "test" } })
        )[2],
        "tag \"env\" value equals \"test\""
    );
    assert_eq!(
        row(json!({ "type": "REGION_EXEMPTION", "regions": ["us-east-1", "us-west-2"] }))[2],
        "us-east-1, us-west-2"
    );
    assert_eq!(
        row(
            json!({ "type": "ROUTE_EXEMPTION", "apiId": "a1b2", "routeKey": "GET /health", "createdBy": "jo", "createdAt": "2026-01-01" })
        ),
        vec![
            "ROUTE_EXEMPTION",
            "",
            "a1b2 GET /health",
            "jo",
            "2026-01-01"
        ]
    );
}

#[test]
fn test_stored_exemption_round_trips_unchanged() {
    let raw = json!({ "type": "ROUTE_EXEMPTION", "apiId": "a", "routeKey": "GET /", "createdBy": "jo", "extra": 1 });
    let parsed: StoredDetectionExemption = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(serde_json::to_value(&parsed).unwrap(), raw);
}

#[test]
fn test_parse_exemptions_accepts_array_or_get_output() {
    let entry = json!({ "type": "REGION_EXEMPTION", "regions": ["us-west-2"] });
    assert_eq!(
        parse_exemptions(&json!([entry.clone()]).to_string()).unwrap(),
        vec![entry.clone()]
    );
    let get_output = exemption_set(json!("v"))["data"].clone();
    assert_eq!(parse_exemptions(&get_output.to_string()).unwrap().len(), 1);
    assert!(parse_exemptions("[]").unwrap().is_empty());
}

#[test]
fn test_parse_exemptions_rejects_bad_input() {
    assert!(parse_exemptions("not json").is_err());
    assert!(parse_exemptions("{}").is_err());
    assert!(parse_exemptions("\"x\"").is_err());
    let err = parse_exemptions("[{}, 3]").unwrap_err();
    assert!(err.to_string().contains("Exemption 2"));
}
