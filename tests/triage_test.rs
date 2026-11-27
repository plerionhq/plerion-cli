use mockito::{Matcher, Server};
use plerion::api::endpoints::triage::{self, ListTriageDecisionsParams};
use plerion::api::models::triage::{TriageCondition, TriageDecision};
use plerion::api::client::PlerionClient;
use plerion::cli::triage::{
    create_body, explain_error, parse_body, CreateDecisionArgs, EditDecisionArgs, EditInput,
};
use plerion::error::PlerionError;
use plerion::output::TableRenderable;
use serde_json::json;

const DECISION_ID: &str = "3f2b8c1e-7d4a-4e9b-a1c2-5d6e7f8a9b0c";

fn rule_json() -> serde_json::Value {
    json!({
        "version": 1,
        "conditions": [
            { "field": "check", "comparison": "isAnyOf", "values": ["PLERION-AWS-16"] },
            { "field": "integration", "comparison": "isAnyOf", "values": ["int-1", "int-2"] }
        ]
    })
}

fn decision_json() -> serde_json::Value {
    json!({
        "id": DECISION_ID,
        "subjectKind": "finding",
        "rule": rule_json(),
        "exemption": { "reason": "ACCEPTED_RISK", "untilAt": "2027-01-01T00:00:00.000Z" },
        "outcome": "exempt",
        "note": "Staging only",
        "status": "active",
        "isInForce": true,
        "createdBy": "key-1",
        "createdAt": "2026-11-01T00:00:00.000Z",
        "updatedBy": "key-1",
        "updatedAt": "2026-11-02T00:00:00.000Z"
    })
}

fn decision() -> TriageDecision {
    serde_json::from_value(decision_json()).unwrap()
}

fn client(server: &Server) -> PlerionClient {
    PlerionClient::with_base_url(&server.url(), "key").unwrap()
}

fn create_args() -> CreateDecisionArgs {
    CreateDecisionArgs {
        file: None,
        subject_kind: None,
        rule: None,
        reason: None,
        until: None,
        note: None,
    }
}

fn edit_args() -> EditDecisionArgs {
    EditDecisionArgs {
        id: DECISION_ID.into(),
        file: None,
        rule: None,
        reason: None,
        until: None,
        clear_until: false,
        note: None,
        updated_at: None,
    }
}

fn write_temp_json(name: &str, value: &serde_json::Value) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("plerion-triage-test-{}-{name}.json", std::process::id()));
    std::fs::write(&path, value.to_string()).unwrap();
    path
}

// --- endpoints ---

#[tokio::test]
async fn test_list_sends_per_page_and_cursor() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/triage/decisions")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("perPage".into(), "2".into()),
            Matcher::UrlEncoded("cursor".into(), "abc".into()),
        ]))
        .with_status(200)
        .with_body(json!({ "data": [decision_json()], "meta": { "perPage": 2, "cursor": "next-1" } }).to_string())
        .create_async()
        .await;

    let params = ListTriageDecisionsParams { per_page: Some(2), cursor: Some("abc".into()) };
    let resp = triage::list_triage_decisions(&client(&server), &params).await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.meta.per_page, Some(2));
    assert_eq!(resp.meta.cursor.as_deref(), Some("next-1"));
}

#[tokio::test]
async fn test_list_last_page_has_null_cursor() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/triage/decisions")
        .with_status(200)
        .with_body(json!({ "data": [], "meta": { "perPage": 100, "cursor": null } }).to_string())
        .create_async()
        .await;

    let resp = triage::list_triage_decisions(&client(&server), &ListTriageDecisionsParams::default())
        .await
        .unwrap();
    assert!(resp.meta.cursor.is_none());
}

#[tokio::test]
async fn test_get_decision() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", format!("/v1/tenant/triage/decisions/{DECISION_ID}").as_str())
        .with_status(200)
        .with_body(json!({ "data": decision_json() }).to_string())
        .create_async()
        .await;

    let resp = triage::get_triage_decision(&client(&server), DECISION_ID).await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.subject_kind.as_deref(), Some("finding"));
    assert_eq!(resp.data.rule.as_ref().unwrap().conditions.len(), 2);
    assert_eq!(resp.data.is_in_force, Some(true));
}

#[tokio::test]
async fn test_create_posts_the_body() {
    let mut server = Server::new_async().await;
    let body = json!({
        "subjectKind": "finding",
        "rule": rule_json(),
        "exemption": { "reason": "ACCEPTED_RISK", "untilAt": null },
        "note": "Staging only"
    });
    let mock = server
        .mock("POST", "/v1/tenant/triage/decisions")
        .match_body(Matcher::Json(body.clone()))
        .with_status(201)
        .with_body(json!({ "data": decision_json(), "meta": { "isApplyQueued": true } }).to_string())
        .create_async()
        .await;

    let resp = triage::create_triage_decision(&client(&server), &body).await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.id.as_deref(), Some(DECISION_ID));
    assert_eq!(resp.meta.unwrap().is_apply_queued, Some(true));
}

#[tokio::test]
async fn test_update_puts_the_body() {
    let mut server = Server::new_async().await;
    let body = json!({ "updatedAt": "2026-11-02T00:00:00.000Z", "note": "x" });
    let mock = server
        .mock("PUT", format!("/v1/tenant/triage/decisions/{DECISION_ID}").as_str())
        .match_body(Matcher::Json(body.clone()))
        .with_status(200)
        .with_body(json!({ "data": decision_json(), "meta": { "isApplyQueued": false } }).to_string())
        .create_async()
        .await;

    let resp = triage::update_triage_decision(&client(&server), DECISION_ID, &body).await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.updated_at.as_deref(), Some("2026-11-02T00:00:00.000Z"));
}

#[tokio::test]
async fn test_delete_decision() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", format!("/v1/tenant/triage/decisions/{DECISION_ID}").as_str())
        .with_status(200)
        .with_body(json!({ "data": { "id": DECISION_ID } }).to_string())
        .create_async()
        .await;

    let resp = triage::delete_triage_decision(&client(&server), DECISION_ID).await.unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.id.as_deref(), Some(DECISION_ID));
}

#[tokio::test]
async fn test_dot_only_id_is_refused_before_the_request() {
    let server = Server::new_async().await;
    let err = triage::get_triage_decision(&client(&server), "..").await.unwrap_err();
    assert_eq!(err.to_string(), "'..' is not a valid decision ID");
}

// --- models ---

#[test]
fn test_decision_row_matches_headers() {
    let d = decision();
    assert_eq!(TriageDecision::headers().len(), d.row().len());
    assert_eq!(
        d.row(),
        vec![
            DECISION_ID, "finding", "exempt", "ACCEPTED_RISK", "2027-01-01T00:00:00.000Z",
            "active", "true", "2", "Staging only", "key-1", "2026-11-02T00:00:00.000Z",
        ]
    );
}

#[test]
fn test_condition_row_joins_values_and_shows_hidden() {
    let c: TriageCondition = serde_json::from_value(json!({
        "field": "integration", "comparison": "isAnyOf", "values": ["int-1", 2, true], "hasHiddenValues": true
    }))
    .unwrap();
    assert_eq!(TriageCondition::headers().len(), c.row().len());
    assert_eq!(c.row(), vec!["integration", "", "isAnyOf", "int-1, 2, true", "true"]);
}

#[test]
fn test_condition_without_hidden_values_serializes_without_the_key() {
    let c: TriageCondition =
        serde_json::from_value(json!({ "field": "check", "comparison": "isAnyOf", "values": ["A"] })).unwrap();
    let v = serde_json::to_value(&c).unwrap();
    assert!(v.get("hasHiddenValues").is_none());
    assert!(v.get("key").is_none());
}

// --- create body ---

#[test]
fn test_create_body_from_flags() {
    let args = CreateDecisionArgs {
        subject_kind: Some("vulnerability".into()),
        rule: Some(rule_json().to_string()),
        reason: Some("FALSE_POSITIVE".into()),
        until: Some("2027-01-01T00:00:00Z".into()),
        note: Some("Not reachable".into()),
        ..create_args()
    };
    assert_eq!(
        create_body(&args).unwrap(),
        json!({
            "subjectKind": "vulnerability",
            "rule": rule_json(),
            "exemption": { "reason": "FALSE_POSITIVE", "untilAt": "2027-01-01T00:00:00Z" },
            "note": "Not reachable"
        })
    );
}

#[test]
fn test_create_body_sends_null_until_when_not_given() {
    let args = CreateDecisionArgs {
        subject_kind: Some("finding".into()),
        rule: Some(rule_json().to_string()),
        reason: Some("ACCEPTED_RISK".into()),
        note: Some("n".into()),
        ..create_args()
    };
    let body = create_body(&args).unwrap();
    assert_eq!(body["exemption"], json!({ "reason": "ACCEPTED_RISK", "untilAt": null }));
}

#[test]
fn test_create_body_from_get_output_drops_server_fields_and_flags_win() {
    let path = write_temp_json("create-from-get", &decision_json());
    let args = CreateDecisionArgs {
        file: Some(path.to_string_lossy().into()),
        note: Some("Copied".into()),
        ..create_args()
    };
    let body = create_body(&args).unwrap();
    std::fs::remove_file(path).ok();
    assert_eq!(
        body,
        json!({
            "subjectKind": "finding",
            "rule": rule_json(),
            "exemption": { "reason": "ACCEPTED_RISK", "untilAt": "2027-01-01T00:00:00.000Z" },
            "note": "Copied"
        })
    );
}

#[test]
fn test_create_body_names_every_missing_flag() {
    let err = create_body(&create_args()).unwrap_err().to_string();
    assert_eq!(
        err,
        "A decision needs --subject-kind, --rule, --reason, --note. Pass them as flags or in --file."
    );
}

#[test]
fn test_create_body_rejects_a_blank_note() {
    let args = CreateDecisionArgs {
        subject_kind: Some("finding".into()),
        rule: Some(rule_json().to_string()),
        reason: Some("ACCEPTED_RISK".into()),
        note: Some("   ".into()),
        ..create_args()
    };
    assert!(create_body(&args).unwrap_err().to_string().contains("--note"));
}

#[test]
fn test_create_body_rejects_invalid_rule_json() {
    let args = CreateDecisionArgs { rule: Some("{not json".into()), ..create_args() };
    assert!(create_body(&args).unwrap_err().to_string().starts_with("Invalid JSON for --rule"));
    let args = CreateDecisionArgs { rule: Some("[]".into()), ..create_args() };
    assert!(create_body(&args).unwrap_err().to_string().contains("JSON object"));
}

#[test]
fn test_create_body_refuses_hidden_values() {
    let mut rule = rule_json();
    rule["conditions"][1]["hasHiddenValues"] = json!(true);
    let args = CreateDecisionArgs {
        subject_kind: Some("finding".into()),
        rule: Some(rule.to_string()),
        reason: Some("ACCEPTED_RISK".into()),
        note: Some("n".into()),
        ..create_args()
    };
    assert!(create_body(&args).unwrap_err().to_string().contains("cannot see"));
}

#[test]
fn test_parse_body_unwraps_a_saved_api_response() {
    let raw = json!({ "data": decision_json(), "meta": { "isApplyQueued": true } }).to_string();
    let body = parse_body(&raw, "f.json").unwrap();
    assert_eq!(body["subjectKind"], "finding");
    assert!(body.get("id").is_none());
    assert!(body.get("status").is_none());
}

#[test]
fn test_parse_body_rejects_a_non_object() {
    assert_eq!(parse_body("[1]", "f.json").unwrap_err().to_string(), "--file f.json must hold a JSON object");
}

// --- edit body ---

#[test]
fn test_edit_with_nothing_to_change_is_refused() {
    let err = EditInput::from_args(&edit_args()).unwrap_err().to_string();
    assert!(err.starts_with("Nothing to change."), "{err}");
}

#[test]
fn test_edit_note_only_reads_the_current_decision_and_keeps_the_rest() {
    let input = EditInput::from_args(&EditDecisionArgs { note: Some("Renewed".into()), ..edit_args() }).unwrap();
    assert!(input.needs_current());
    let body = input.into_body(Some(&decision())).unwrap();
    assert_eq!(
        body,
        json!({
            "rule": rule_json(),
            "exemption": { "reason": "ACCEPTED_RISK", "untilAt": "2027-01-01T00:00:00.000Z" },
            "note": "Renewed",
            "updatedAt": "2026-11-02T00:00:00.000Z"
        })
    );
}

#[test]
fn test_edit_clear_until_and_new_reason() {
    let args = EditDecisionArgs {
        reason: Some("COMPENSATING_CONTROL".into()),
        clear_until: true,
        ..edit_args()
    };
    let body = EditInput::from_args(&args).unwrap().into_body(Some(&decision())).unwrap();
    assert_eq!(body["exemption"], json!({ "reason": "COMPENSATING_CONTROL", "untilAt": null }));
    assert_eq!(body["note"], "Staging only");
}

#[test]
fn test_edit_with_every_part_and_updated_at_needs_no_read() {
    let args = EditDecisionArgs {
        rule: Some(rule_json().to_string()),
        reason: Some("ACCEPTED_RISK".into()),
        until: Some("2027-06-01T00:00:00Z".into()),
        note: Some("n".into()),
        updated_at: Some("2026-11-02T00:00:00.000Z".into()),
        ..edit_args()
    };
    let input = EditInput::from_args(&args).unwrap();
    assert!(!input.needs_current());
    let body = input.into_body(None).unwrap();
    assert_eq!(body["updatedAt"], "2026-11-02T00:00:00.000Z");
    assert_eq!(body["exemption"]["untilAt"], "2027-06-01T00:00:00Z");
}

#[test]
fn test_edit_keeps_the_updated_at_the_user_passed() {
    let args = EditDecisionArgs {
        note: Some("n".into()),
        updated_at: Some("2026-10-01T00:00:00.000Z".into()),
        ..edit_args()
    };
    let input = EditInput::from_args(&args).unwrap();
    assert!(input.needs_current());
    let body = input.into_body(Some(&decision())).unwrap();
    assert_eq!(body["updatedAt"], "2026-10-01T00:00:00.000Z");
}

#[test]
fn test_edit_from_get_output_uses_its_updated_at_and_drops_subject_kind() {
    let mut edited = decision_json();
    edited["note"] = json!("Edited in a file");
    let path = write_temp_json("edit-from-get", &edited);
    let input = EditInput::from_args(&EditDecisionArgs { file: Some(path.to_string_lossy().into()), ..edit_args() }).unwrap();
    std::fs::remove_file(path).ok();
    assert!(!input.needs_current());
    let body = input.into_body(None).unwrap();
    assert_eq!(
        body,
        json!({
            "rule": rule_json(),
            "exemption": { "reason": "ACCEPTED_RISK", "untilAt": "2027-01-01T00:00:00.000Z" },
            "note": "Edited in a file",
            "updatedAt": "2026-11-02T00:00:00.000Z"
        })
    );
}

#[test]
fn test_edit_refuses_a_decision_with_hidden_values() {
    let mut current = decision_json();
    current["rule"]["conditions"][1]["hasHiddenValues"] = json!(true);
    current["note"] = json!(null);
    let current: TriageDecision = serde_json::from_value(current).unwrap();
    let input = EditInput::from_args(&EditDecisionArgs { reason: Some("FALSE_POSITIVE".into()), ..edit_args() }).unwrap();
    let err = input.into_body(Some(&current)).unwrap_err().to_string();
    assert!(err.contains("cannot see"), "{err}");
}

// --- errors ---

fn api_error(status: u16, body: &str) -> PlerionError {
    PlerionError::ApiError { status, message: body.into() }
}

#[test]
fn test_explain_decision_changed() {
    let body = r#"{"errors":[{"code":"DecisionChanged","message":"Someone else changed this decision after you opened it. Reload it to see their change, then try again."}]}"#;
    let text = explain_error(api_error(409, body), Some(DECISION_ID)).to_string();
    assert!(text.starts_with("HTTP error 409 (DecisionChanged): Someone else changed"), "{text}");
    assert!(text.ends_with(&format!("Run `plerion triage decisions get {DECISION_ID}` to see the current version, then edit again.")));
}

#[test]
fn test_explain_tenant_not_on_triage() {
    let body = r#"{"errors":[{"code":"TenantNotOnTriage","message":"This tenant still uses exemptions. Triage decisions are not available to it yet."}]}"#;
    let text = explain_error(api_error(409, body), None).to_string();
    assert!(text.starts_with("HTTP error 409 (TenantNotOnTriage): This tenant still uses exemptions."), "{text}");
    assert!(text.contains("plerion vulnerabilities exemptions"));
}

#[test]
fn test_explain_not_found_names_the_decision() {
    let body = r#"{"errors":[{"code":"DecisionNotFound","message":"Triage decision not found"}]}"#;
    let text = explain_error(api_error(404, body), Some(DECISION_ID)).to_string();
    assert_eq!(
        text,
        format!(
            "HTTP error 404 (DecisionNotFound): Triage decision not found. Triage decision '{DECISION_ID}' \
             was not found. Run `plerion triage decisions list` to see the decisions this key can read."
        )
    );
}

#[test]
fn test_explain_gateway_forbidden_names_the_permissions() {
    let text = explain_error(api_error(403, r#"{"message":"Forbidden"}"#), None).to_string();
    assert!(text.starts_with("HTTP error 403: Forbidden. This API key's role does not allow this."), "{text}");
    assert!(text.contains("Finding:Triage or Vulnerability:Triage"));
}

#[test]
fn test_explain_forbidden_with_a_code_keeps_the_server_message() {
    let body = r#"{"errors":[{"code":"IntegrationAccessDenied","message":"You do not have access to this integration"}]}"#;
    let text = explain_error(api_error(403, body), Some(DECISION_ID)).to_string();
    assert_eq!(text, "HTTP error 403 (IntegrationAccessDenied): You do not have access to this integration.");
}

#[test]
fn test_explain_bad_request() {
    let body = r#"{"errors":[{"code":"RuleNeedsCheck","message":"A rule for findings must name one check."}]}"#;
    let text = explain_error(api_error(400, body), None).to_string();
    assert_eq!(text, "HTTP error 400 (RuleNeedsCheck): A rule for findings must name one check.");
}

#[test]
fn test_explain_leaves_other_errors_alone() {
    let text = explain_error(api_error(500, "boom"), None).to_string();
    assert_eq!(text, "HTTP error 500: boom");
    let text = explain_error(PlerionError::InvalidArgument("x".into()), None).to_string();
    assert_eq!(text, "x");
}
