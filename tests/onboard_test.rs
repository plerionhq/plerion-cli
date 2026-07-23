use mockito::Server;
use plerion::api::client::PlerionClient;
use plerion::onboard::aws_api::*;
use plerion::onboard::ui::Ui;
use plerion::onboard::{run, OnboardError, OnboardOptions, OnboardOutcome};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Scripted fakes
// ---------------------------------------------------------------------------

struct MockAws {
    account: String,
    arn: String,
    alias: Option<String>,
    /// Status returned when preflight checks the stack NAME (None = no stack).
    existing_stack: Option<String>,
    roles: Vec<String>,
    /// Actions the simulator reports as denied.
    sim_denied: Vec<String>,
    sim_error: bool,
    /// Statuses returned, in order, when polling the created stack (by ID).
    poll_statuses: Mutex<VecDeque<String>>,
    failure_events: Vec<StackEvent>,
    outputs: Vec<StackOutput>,
    create_called: AtomicBool,
}

impl Default for MockAws {
    fn default() -> Self {
        Self {
            account: "111111111111".to_string(),
            arn: "arn:aws:sts::111111111111:assumed-role/AWSReservedSSO_Admin_abc/session".to_string(),
            alias: Some("acme-prod".to_string()),
            existing_stack: None,
            roles: vec![],
            sim_denied: vec![],
            sim_error: false,
            poll_statuses: Mutex::new(VecDeque::from(vec![
                "CREATE_IN_PROGRESS".to_string(),
                "CREATE_COMPLETE".to_string(),
            ])),
            failure_events: vec![],
            outputs: vec![StackOutput {
                key: "PlerionAccessRoleArn".to_string(),
                value: "arn:aws:iam::111111111111:role/uuid-PlerionAccessRole".to_string(),
            }],
            create_called: AtomicBool::new(false),
        }
    }
}

#[async_trait::async_trait]
impl AwsApi for MockAws {
    async fn caller_identity(&self) -> Result<CallerIdentity, OnboardError> {
        Ok(CallerIdentity {
            account: self.account.clone(),
            arn: self.arn.clone(),
        })
    }
    async fn account_alias(&self) -> Result<Option<String>, OnboardError> {
        Ok(self.alias.clone())
    }
    async fn find_stack(&self, name: &str) -> Result<Option<String>, OnboardError> {
        if name.starts_with("arn:") {
            // Poll loop looks the created stack up by its StackId.
            let mut q = self.poll_statuses.lock().unwrap();
            let status = q.front().cloned();
            if q.len() > 1 {
                q.pop_front();
            }
            Ok(status)
        } else {
            Ok(self.existing_stack.clone())
        }
    }
    async fn list_roles_matching(&self, _needle: &str) -> Result<Vec<String>, OnboardError> {
        Ok(self.roles.clone())
    }
    async fn role_arn(&self, role_name: &str) -> Result<String, OnboardError> {
        Ok(format!(
            "arn:aws:iam::{}:role/aws-reserved/sso.amazonaws.com/{}",
            self.account, role_name
        ))
    }
    async fn simulate(
        &self,
        _policy_source_arn: &str,
        actions: &[String],
    ) -> Result<Vec<SimResult>, OnboardError> {
        if self.sim_error {
            return Err(OnboardError::Aws("simulate unavailable".to_string()));
        }
        Ok(actions
            .iter()
            .map(|a| SimResult {
                action: a.clone(),
                allowed: !self.sim_denied.contains(a),
            })
            .collect())
    }
    async fn template_summary(&self, _template_url: &str) -> Result<TemplateSummary, OnboardError> {
        Ok(TemplateSummary {
            description: Some("Grants Plerion access".to_string()),
        })
    }
    async fn create_stack(&self, req: &CreateStackRequest) -> Result<String, OnboardError> {
        self.create_called.store(true, Ordering::SeqCst);
        assert!(
            !req.parameters.iter().any(|(_, v)| v.is_empty()),
            "no CreateStack parameter may be empty"
        );
        Ok(format!(
            "arn:aws:cloudformation:us-east-1:{}:stack/{}/uuid",
            self.account, req.stack_name
        ))
    }
    async fn stack_failure_events(&self, _name: &str) -> Result<Vec<StackEvent>, OnboardError> {
        Ok(self.failure_events.clone())
    }
    async fn stack_outputs(&self, _name: &str) -> Result<Vec<StackOutput>, OnboardError> {
        Ok(self.outputs.clone())
    }
}

struct ScriptedUi {
    interactive: bool,
    answers: VecDeque<bool>,
}

impl ScriptedUi {
    fn yes() -> Self {
        Self { interactive: true, answers: VecDeque::from(vec![true]) }
    }
    fn no() -> Self {
        Self { interactive: true, answers: VecDeque::from(vec![false]) }
    }
    fn non_interactive() -> Self {
        Self { interactive: false, answers: VecDeque::new() }
    }
}

impl Ui for ScriptedUi {
    fn is_interactive(&self) -> bool {
        self.interactive
    }
    fn confirm(&mut self, _prompt: &str) -> bool {
        self.answers.pop_front().unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Plerion-side mock server
// ---------------------------------------------------------------------------

async fn plerion_server(with_existing_integration: bool) -> (mockito::ServerGuard, PlerionClient) {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/v1/tenant/external-id")
        .with_status(200)
        .with_body(r#"{"data":{"externalId":"ext-123"}}"#)
        .create_async()
        .await;
    server
        .mock("GET", "/v1/tenant/cloudformation-templates")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(r#"{"data":{"templateURL":"https://s3.example/template.yaml","templateVersion":"v33"}}"#)
        .create_async()
        .await;
    let integrations = if with_existing_integration {
        r#"{"data":[{"integrationId":"int-1","awsAccountId":"111111111111"}],"meta":{"cursor":null,"hasNextPage":false}}"#
    } else {
        r#"{"data":[],"meta":{"cursor":null,"hasNextPage":false}}"#
    };
    server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body(integrations)
        .create_async()
        .await;
    server
        .mock("POST", "/v1/tenant/integrations/token")
        .with_status(200)
        .with_body(r#"{"data":{"token":"tmp-onboarding-token"}}"#)
        .create_async()
        .await;
    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    (server, client)
}

fn fast_opts() -> OnboardOptions {
    OnboardOptions {
        service_account_id: Some("222222222222".to_string()),
        poll_interval_secs: 0,
        wait_timeout_secs: 5,
        ..OnboardOptions::default()
    }
}

// ---------------------------------------------------------------------------
// Scenarios
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_happy_path_completes_with_outputs() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::yes();

    let outcome = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap();
    match outcome {
        OnboardOutcome::Completed(v) => {
            assert_eq!(v["accountId"], "111111111111");
            assert_eq!(v["status"], "CREATE_COMPLETE");
            assert_eq!(v["templateVersion"], "v33");
            assert!(v["outputs"]["PlerionAccessRoleArn"]
                .as_str()
                .unwrap()
                .contains("PlerionAccessRole"));
        }
        _ => panic!("expected Completed"),
    }
    assert!(aws.create_called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_expect_account_id_mismatch_fails_preflight() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::yes();
    let opts = OnboardOptions {
        expect_account_id: Some("999999999999".to_string()),
        ..fast_opts()
    };

    let err = run(&client, &aws, &mut ui, &opts).await.unwrap_err();
    assert!(matches!(err, OnboardError::PreflightFailed(_)), "{err}");
    assert_eq!(err.exit_code(), 2);
    assert!(!aws.create_called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_rollback_complete_stack_gets_delete_hint() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws {
        existing_stack: Some("ROLLBACK_COMPLETE".to_string()),
        ..MockAws::default()
    };
    let mut ui = ScriptedUi::yes();

    let err = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap_err();
    assert_eq!(err.exit_code(), 2);
    assert!(err.to_string().contains("delete-stack"), "{err}");
}

#[tokio::test]
async fn test_existing_roles_abort_without_allow_existing() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws {
        roles: vec!["uuid-PlerionAccessRole".to_string()],
        ..MockAws::default()
    };
    let mut ui = ScriptedUi::yes();

    let err = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap_err();
    assert!(matches!(err, OnboardError::AlreadyOnboarded(_)), "{err}");
    assert_eq!(err.exit_code(), 4);
}

#[tokio::test]
async fn test_existing_integration_detected_plerion_side() {
    let (_server, client) = plerion_server(true).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::yes();

    let err = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap_err();
    assert!(matches!(err, OnboardError::AlreadyOnboarded(_)), "{err}");
    assert_eq!(err.exit_code(), 4);
}

#[tokio::test]
async fn test_allow_existing_proceeds_past_duplicates() {
    let (_server, client) = plerion_server(true).await;
    let aws = MockAws {
        roles: vec!["uuid-PlerionAccessRole".to_string()],
        ..MockAws::default()
    };
    let mut ui = ScriptedUi::yes();
    let opts = OnboardOptions { allow_existing: true, ..fast_opts() };

    let outcome = run(&client, &aws, &mut ui, &opts).await.unwrap();
    assert!(matches!(outcome, OnboardOutcome::Completed(_)));
}

#[tokio::test]
async fn test_simulation_denials_are_advisory_by_default() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws {
        sim_denied: vec!["cloudformation:CreateStack".to_string()],
        ..MockAws::default()
    };
    let mut ui = ScriptedUi::yes();

    let outcome = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap();
    assert!(matches!(outcome, OnboardOutcome::Completed(_)));
}

#[tokio::test]
async fn test_simulation_denials_fatal_with_strict_preflight() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws {
        sim_denied: vec!["cloudformation:CreateStack".to_string()],
        ..MockAws::default()
    };
    let mut ui = ScriptedUi::yes();
    let opts = OnboardOptions { strict_preflight: true, ..fast_opts() };

    let err = run(&client, &aws, &mut ui, &opts).await.unwrap_err();
    assert!(matches!(err, OnboardError::PreflightFailed(_)), "{err}");
    assert_eq!(err.exit_code(), 2);
}

#[tokio::test]
async fn test_simulation_unavailable_skips_check() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws { sim_error: true, ..MockAws::default() };
    let mut ui = ScriptedUi::yes();

    let outcome = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap();
    assert!(matches!(outcome, OnboardOutcome::Completed(_)));
}

#[tokio::test]
async fn test_validate_only_stops_before_confirmation_and_deploy() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    // A UI that would panic if asked: validate-only must never confirm.
    let mut ui = ScriptedUi::non_interactive();
    let opts = OnboardOptions { validate_only: true, ..fast_opts() };

    let outcome = run(&client, &aws, &mut ui, &opts).await.unwrap();
    match outcome {
        OnboardOutcome::Validated(report) => {
            assert_eq!(report["accountId"], "111111111111");
            assert_eq!(report["templateVersion"], "v33");
            assert_eq!(report["existingIntegration"], false);
        }
        _ => panic!("expected Validated"),
    }
    assert!(!aws.create_called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_non_interactive_without_yes_aborts() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::non_interactive();

    let err = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap_err();
    assert!(matches!(err, OnboardError::Aborted(_)), "{err}");
    assert_eq!(err.exit_code(), 2);
    assert!(!aws.create_called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_confirmation_declined_aborts() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::no();

    let err = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap_err();
    assert!(matches!(err, OnboardError::Aborted(_)), "{err}");
    assert!(!aws.create_called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_yes_skips_confirmation() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::non_interactive();
    let opts = OnboardOptions { yes: true, ..fast_opts() };

    let outcome = run(&client, &aws, &mut ui, &opts).await.unwrap();
    assert!(matches!(outcome, OnboardOutcome::Completed(_)));
}

#[tokio::test]
async fn test_deploy_failure_dumps_events_with_token_hint() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws {
        poll_statuses: Mutex::new(VecDeque::from(vec![
            "CREATE_IN_PROGRESS".to_string(),
            "ROLLBACK_COMPLETE".to_string(),
        ])),
        failure_events: vec![StackEvent {
            logical_id: "PlerionAPICall".to_string(),
            status: "CREATE_FAILED".to_string(),
            reason: Some("Received response status [FAILED]".to_string()),
        }],
        ..MockAws::default()
    };
    let mut ui = ScriptedUi::yes();

    let err = run(&client, &aws, &mut ui, &fast_opts()).await.unwrap_err();
    assert!(matches!(err, OnboardError::DeployFailed(_)), "{err}");
    assert_eq!(err.exit_code(), 3);
    let msg = err.to_string();
    assert!(msg.contains("PlerionAPICall"), "{msg}");
    assert!(msg.contains("token"), "{msg}");
}

#[tokio::test]
async fn test_wait_timeout() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws {
        poll_statuses: Mutex::new(VecDeque::from(vec!["CREATE_IN_PROGRESS".to_string()])),
        ..MockAws::default()
    };
    let mut ui = ScriptedUi::yes();
    let opts = OnboardOptions { wait_timeout_secs: 0, ..fast_opts() };

    let err = run(&client, &aws, &mut ui, &opts).await.unwrap_err();
    assert!(matches!(err, OnboardError::DeployFailed(_)), "{err}");
    assert!(err.to_string().contains("timed out"), "{err}");
}

#[tokio::test]
async fn test_stack_name_prefix_rule() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::yes();
    let opts = OnboardOptions {
        stack_name: "MyStack".to_string(),
        ..fast_opts()
    };

    let err = run(&client, &aws, &mut ui, &opts).await.unwrap_err();
    assert!(matches!(err, OnboardError::InvalidOptions(_)), "{err}");
    assert_eq!(err.exit_code(), 1);

    // Allowed when auto-update is off.
    let opts = OnboardOptions {
        stack_name: "MyStack".to_string(),
        auto_update: false,
        ..fast_opts()
    };
    let outcome = run(&client, &aws, &mut ui, &opts).await;
    assert!(outcome.is_ok());
}

#[tokio::test]
async fn test_missing_service_account_id_gives_guidance() {
    let (_server, client) = plerion_server(false).await;
    let aws = MockAws::default();
    let mut ui = ScriptedUi::yes();
    let opts = OnboardOptions {
        service_account_id: None,
        ..fast_opts()
    };

    let err = run(&client, &aws, &mut ui, &opts).await.unwrap_err();
    assert!(matches!(err, OnboardError::InvalidOptions(_)), "{err}");
    assert!(err.to_string().contains("--service-account-id"), "{err}");
}
