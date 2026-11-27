use std::io::Read;

use clap::{Args, Subcommand};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::api::client::PlerionClient;
use crate::api::endpoints::triage::{
    create_triage_decision, delete_triage_decision, get_triage_decision, list_triage_decisions,
    update_triage_decision, ListTriageDecisionsParams,
};
use crate::api::models::triage::TriageDecision;
use crate::config::Config;
use crate::error::PlerionError;
use crate::output::{self, OutputFormat};

#[derive(Args, Debug)]
pub struct TriageArgs {
    #[command(subcommand)]
    pub command: TriageCommands,
}

#[derive(Subcommand, Debug)]
pub enum TriageCommands {
    /// Triage decisions: rules that exempt matching findings or vulnerabilities
    Decisions(DecisionsArgs),
}

#[derive(Args, Debug)]
pub struct DecisionsArgs {
    #[command(subcommand)]
    pub command: DecisionsCommands,
}

#[derive(Subcommand, Debug)]
pub enum DecisionsCommands {
    /// List triage decisions, in force first, then newest first
    List(ListDecisionsArgs),
    /// Get one triage decision and its rule
    Get {
        /// Decision ID
        id: String,
    },
    /// Create a triage decision.
    ///
    /// A decision needs a subject kind, a rule, an exemption reason and a
    /// note. Pass them as flags, in a JSON file with --file, or both; flags
    /// win over the file.
    Create(CreateDecisionArgs),
    /// Edit a triage decision.
    ///
    /// An edit replaces the rule, exemption and note. Anything you do not
    /// pass is kept as it is now. The edit is refused if the decision changed
    /// after the version you send as --updated-at. Without --updated-at (on
    /// the flag or in --file), the CLI reads the decision first and uses its
    /// updatedAt, so a change made by someone else before that read is
    /// overwritten.
    Edit(EditDecisionArgs),
    /// Delete a triage decision and its history
    Delete {
        /// Decision ID
        id: String,
    },
}

#[derive(Args, Debug)]
pub struct ListDecisionsArgs {
    /// Decisions per page (1 to 500)
    #[arg(long, default_value = "100", value_parser = clap::value_parser!(u32).range(1..=500))]
    pub per_page: u32,
    /// Cursor from the last page, to read the next one
    #[arg(long, conflicts_with = "all")]
    pub cursor: Option<String>,
    /// Fetch all pages automatically
    #[arg(long)]
    pub all: bool,
}

const SUBJECT_KINDS: [&str; 2] = ["finding", "vulnerability"];
const REASONS: [&str; 3] = ["FALSE_POSITIVE", "COMPENSATING_CONTROL", "ACCEPTED_RISK"];

#[derive(Args, Debug)]
pub struct CreateDecisionArgs {
    /// JSON file holding subjectKind, rule, exemption and note, or - for stdin.
    /// The output of `get --output json` works; fields the server sets are dropped.
    #[arg(long, value_name = "PATH")]
    pub file: Option<String>,
    /// What the decision covers
    #[arg(long, value_parser = SUBJECT_KINDS)]
    pub subject_kind: Option<String>,
    /// The rule as JSON: {"version": 1, "conditions": [...]}
    #[arg(long, value_name = "JSON")]
    pub rule: Option<String>,
    /// Why the items are exempt
    #[arg(long, value_parser = REASONS)]
    pub reason: Option<String>,
    /// When the exemption ends (ISO 8601 date-time); without it, it never ends
    #[arg(long, value_name = "DATE_TIME")]
    pub until: Option<String>,
    /// Why the decision was made
    #[arg(long)]
    pub note: Option<String>,
}

#[derive(Args, Debug)]
pub struct EditDecisionArgs {
    /// Decision ID
    pub id: String,
    /// JSON file holding any of rule, exemption, note and updatedAt, or - for
    /// stdin. The output of `get --output json` works; fields the server sets
    /// are dropped.
    #[arg(long, value_name = "PATH")]
    pub file: Option<String>,
    /// The new rule as JSON: {"version": 1, "conditions": [...]}
    #[arg(long, value_name = "JSON")]
    pub rule: Option<String>,
    /// Why the items are exempt
    #[arg(long, value_parser = REASONS)]
    pub reason: Option<String>,
    /// When the exemption ends (ISO 8601 date-time)
    #[arg(long, value_name = "DATE_TIME")]
    pub until: Option<String>,
    /// Make the exemption never end
    #[arg(long, conflicts_with = "until")]
    pub clear_until: bool,
    /// Why the decision was changed
    #[arg(long)]
    pub note: Option<String>,
    /// The decision's updatedAt as you last read it. Without it, the CLI reads
    /// the decision first and uses the updatedAt it gets back.
    #[arg(long, value_name = "DATE_TIME")]
    pub updated_at: Option<String>,
}

/// Fields the server sets. They appear in `get` output and are dropped from
/// --file so that output can be edited and sent back.
const SERVER_FIELDS: &[&str] = &[
    "id", "status", "isInForce", "outcome", "createdBy", "createdAt", "updatedBy",
];

/// Reads a JSON object from a file, or stdin when `path` is `-`.
fn read_body_file(path: &str) -> anyhow::Result<Map<String, Value>> {
    let raw = if path == "-" {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read stdin: {e}"))?;
        buf
    } else {
        std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read {path}: {e}"))?
    };
    parse_body(&raw, path)
}

/// Parses a decision body from --file, dropping the fields the server sets.
pub fn parse_body(raw: &str, path: &str) -> anyhow::Result<Map<String, Value>> {
    let value: Value = serde_json::from_str(raw)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in --file {path}: {e}"))?;
    // `get --output json` prints the decision itself; a saved API response wraps it in `data`.
    let value = match value {
        Value::Object(mut map) if map.len() <= 2 && map.get("data").is_some_and(Value::is_object) => {
            map.remove("data").unwrap_or_default()
        }
        other => other,
    };
    let Value::Object(mut map) = value else {
        anyhow::bail!("--file {path} must hold a JSON object");
    };
    for key in SERVER_FIELDS {
        map.remove(*key);
    }
    Ok(map)
}

fn parse_rule(text: &str) -> anyhow::Result<Value> {
    let rule: Value = serde_json::from_str(text)
        .map_err(|e| anyhow::anyhow!("Invalid JSON for --rule: {e}"))?;
    if !rule.is_object() {
        anyhow::bail!("--rule must be a JSON object with version and conditions");
    }
    Ok(rule)
}

/// Whether any condition in a rule carries `hasHiddenValues`. The API refuses
/// such a rule, since sending it back would drop the hidden values.
fn rule_has_hidden_values(rule: &Value) -> bool {
    rule.get("conditions")
        .and_then(Value::as_array)
        .is_some_and(|conditions| conditions.iter().any(|c| c.get("hasHiddenValues").is_some()))
}

const HIDDEN_VALUES_MESSAGE: &str = "This rule has values your API key cannot see \
    (conditions with hasHiddenValues), so the decision cannot be changed with this key. \
    Use a key whose role covers every integration the decision reaches.";

/// Sets the exemption from the flags on top of `base`, keeping what they leave out.
fn merge_exemption(base: Option<Value>, reason: Option<&String>, until: UntilChange) -> Value {
    let mut exemption = match base {
        Some(Value::Object(map)) => map,
        _ => Map::new(),
    };
    if let Some(r) = reason {
        exemption.insert("reason".into(), Value::String(r.clone()));
    }
    match until {
        UntilChange::Keep => {}
        UntilChange::Set(s) => {
            exemption.insert("untilAt".into(), Value::String(s));
        }
        UntilChange::Clear => {
            exemption.insert("untilAt".into(), Value::Null);
        }
    }
    // The API needs untilAt present; null means the exemption never ends.
    exemption.entry("untilAt").or_insert(Value::Null);
    Value::Object(exemption)
}

#[derive(Debug, Clone)]
enum UntilChange {
    Keep,
    Set(String),
    Clear,
}

fn has_text(body: &Map<String, Value>, key: &str) -> bool {
    body.get(key).and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty())
}

fn has_reason(body: &Map<String, Value>) -> bool {
    body.get("exemption")
        .and_then(|e| e.get("reason"))
        .and_then(Value::as_str)
        .is_some_and(|s| !s.is_empty())
}

/// Builds the create body from --file and the flags, and checks it has
/// everything the API needs before any request is sent.
pub fn create_body(a: &CreateDecisionArgs) -> anyhow::Result<Value> {
    let mut body = match &a.file {
        Some(path) => read_body_file(path)?,
        None => Map::new(),
    };
    body.remove("updatedAt");
    if let Some(kind) = &a.subject_kind {
        body.insert("subjectKind".into(), Value::String(kind.clone()));
    }
    if let Some(rule) = &a.rule {
        body.insert("rule".into(), parse_rule(rule)?);
    }
    if let Some(note) = &a.note {
        body.insert("note".into(), Value::String(note.clone()));
    }
    let until = a.until.clone().map_or(UntilChange::Keep, UntilChange::Set);
    if a.reason.is_some() || a.until.is_some() || body.contains_key("exemption") {
        let base = body.remove("exemption");
        body.insert("exemption".into(), merge_exemption(base, a.reason.as_ref(), until));
    }

    let mut missing = Vec::new();
    if !has_text(&body, "subjectKind") { missing.push("--subject-kind"); }
    if !body.get("rule").is_some_and(Value::is_object) { missing.push("--rule"); }
    if !has_reason(&body) { missing.push("--reason"); }
    if !has_text(&body, "note") { missing.push("--note"); }
    if !missing.is_empty() {
        anyhow::bail!(
            "A decision needs {}. Pass them as flags or in --file.",
            missing.join(", ")
        );
    }
    if body.get("rule").is_some_and(rule_has_hidden_values) {
        anyhow::bail!("{HIDDEN_VALUES_MESSAGE}");
    }
    Ok(Value::Object(body))
}

/// The parts of an edit taken from --file and the flags, before the current
/// decision fills in what they leave out.
#[derive(Debug)]
pub struct EditInput {
    body: Map<String, Value>,
    updated_at: Option<String>,
    reason: Option<String>,
    until: UntilChange,
}

impl EditInput {
    pub fn from_args(a: &EditDecisionArgs) -> anyhow::Result<Self> {
        let mut body = match &a.file {
            Some(path) => read_body_file(path)?,
            None => Map::new(),
        };
        // An edit cannot change what a decision covers.
        body.remove("subjectKind");
        let file_updated_at = body.remove("updatedAt").and_then(|v| v.as_str().map(String::from));
        if let Some(rule) = &a.rule {
            body.insert("rule".into(), parse_rule(rule)?);
        }
        if let Some(note) = &a.note {
            body.insert("note".into(), Value::String(note.clone()));
        }
        let until = match (&a.until, a.clear_until) {
            (Some(s), _) => UntilChange::Set(s.clone()),
            (None, true) => UntilChange::Clear,
            (None, false) => UntilChange::Keep,
        };
        let changes_nothing = body.is_empty() && a.reason.is_none() && matches!(until, UntilChange::Keep);
        if changes_nothing {
            anyhow::bail!(
                "Nothing to change. Pass at least one of --rule, --reason, --until, \
                 --clear-until, --note or --file."
            );
        }
        Ok(Self {
            body,
            updated_at: a.updated_at.clone().or(file_updated_at),
            reason: a.reason.clone(),
            until,
        })
    }

    /// Whether the current decision must be read to fill in the version or a
    /// part this edit leaves out.
    pub fn needs_current(&self) -> bool {
        let exemption_given = self.body.contains_key("exemption")
            || (self.reason.is_some() && !matches!(self.until, UntilChange::Keep));
        self.updated_at.is_none()
            || !self.body.contains_key("rule")
            || !self.body.contains_key("note")
            || !exemption_given
    }

    /// The PUT body: this edit on top of `current`, the decision as read now.
    pub fn into_body(self, current: Option<&TriageDecision>) -> anyhow::Result<Value> {
        let mut body = self.body;
        let current_value = current.map(serde_json::to_value).transpose()?;
        let from_current = |key: &str| current_value.as_ref().and_then(|c| c.get(key)).cloned();

        if !body.contains_key("rule") {
            if let Some(rule) = from_current("rule").filter(|r| !r.is_null()) {
                body.insert("rule".into(), rule);
            }
        }
        if !body.contains_key("note") {
            if let Some(note) = from_current("note").filter(|n| !n.is_null()) {
                body.insert("note".into(), note);
            }
        }
        let base = body.remove("exemption").or_else(|| from_current("exemption").filter(|e| !e.is_null()));
        body.insert("exemption".into(), merge_exemption(base, self.reason.as_ref(), self.until));

        let updated_at = self.updated_at.or_else(|| current.and_then(|c| c.updated_at.clone()));
        let Some(updated_at) = updated_at else {
            anyhow::bail!("The decision has no updatedAt to send. Pass --updated-at.");
        };
        body.insert("updatedAt".into(), Value::String(updated_at));

        if body.get("rule").is_some_and(rule_has_hidden_values) {
            anyhow::bail!("{HIDDEN_VALUES_MESSAGE}");
        }
        let mut missing = Vec::new();
        if !body.get("rule").is_some_and(Value::is_object) { missing.push("--rule"); }
        if !has_reason(&body) { missing.push("--reason"); }
        if !has_text(&body, "note") { missing.push("--note"); }
        if !missing.is_empty() {
            anyhow::bail!(
                "The edit needs {}, and the decision as read does not hold them. Pass them as flags or in --file.",
                missing.join(", ")
            );
        }
        Ok(Value::Object(body))
    }
}

#[derive(Deserialize)]
struct ErrorBody {
    errors: Option<Vec<ErrorItem>>,
    message: Option<String>,
}

#[derive(Deserialize)]
struct ErrorItem {
    code: Option<String>,
    message: Option<String>,
}

/// Turns an API refusal into a message that says what happened and what to
/// do next. `decision_id` is the decision the request named, if any.
pub fn explain_error(err: PlerionError, decision_id: Option<&str>) -> anyhow::Error {
    let PlerionError::ApiError { status, message } = &err else {
        return err.into();
    };
    let status = *status;
    let parsed: Option<ErrorBody> = serde_json::from_str(message).ok();
    let items: Vec<(String, String)> = parsed
        .as_ref()
        .and_then(|b| b.errors.as_ref())
        .map(|errors| {
            errors
                .iter()
                .map(|e| (e.code.clone().unwrap_or_default(), e.message.clone().unwrap_or_default()))
                .collect()
        })
        .unwrap_or_default();
    let has_code = |code: &str| items.iter().any(|(c, _)| c == code);
    let codes: Vec<&str> = items.iter().map(|(c, _)| c.as_str()).filter(|c| !c.is_empty()).collect();
    let prefix = if codes.is_empty() {
        format!("HTTP error {status}")
    } else {
        format!("HTTP error {status} ({})", codes.join(", "))
    };
    let server_message = if items.is_empty() {
        parsed.and_then(|b| b.message).unwrap_or_default()
    } else {
        items.iter().map(|(_, m)| m.as_str()).collect::<Vec<_>>().join(" ")
    };

    let hint = match status {
        409 if has_code("TenantNotOnTriage") => Some(
            "This tenant still manages exemptions with `plerion profiles detection-exemptions` \
             and `plerion vulnerabilities exemptions`."
                .to_string(),
        ),
        409 if has_code("DecisionChanged") => Some(match decision_id {
            Some(id) => format!(
                "Run `plerion triage decisions get {id}` to see the current version, then edit again."
            ),
            None => "Read the decision again, then edit again.".to_string(),
        }),
        404 => Some(match decision_id {
            Some(id) => format!(
                "Triage decision '{id}' was not found. Run `plerion triage decisions list` to see \
                 the decisions this key can read."
            ),
            None => "Check that the endpoint serves the triage decision routes.".to_string(),
        }),
        403 if items.is_empty() => Some(
            "This API key's role does not allow this. Reading decisions needs Finding:Read or \
             Vulnerability:Read, and changing them needs Finding:Triage or Vulnerability:Triage."
                .to_string(),
        ),
        _ => None,
    };

    if server_message.is_empty() && hint.is_none() {
        return err.into();
    }
    let text = [server_message, hint.unwrap_or_default()]
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| if s.ends_with(['.', '!', '?']) { s } else { format!("{s}.") })
        .collect::<Vec<_>>()
        .join(" ");
    anyhow::anyhow!("{prefix}: {text}")
}

/// Prints a decision, and its rule's conditions as a second table.
fn render_decision(decision: &TriageDecision, config: &Config) -> anyhow::Result<()> {
    let query = config.query.as_deref();
    output::render(decision, config.output, query, config.no_color)?;
    let tabular = matches!(config.output, OutputFormat::Table | OutputFormat::Text);
    let conditions = decision.rule.as_ref().map(|r| r.conditions.as_slice()).unwrap_or_default();
    if query.is_none() && tabular && !conditions.is_empty() {
        println!();
        output::render_list(conditions, config.output, None, config.no_color)?;
    }
    Ok(())
}

pub async fn run(args: &TriageArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    let TriageCommands::Decisions(decisions) = &args.command;
    match &decisions.command {
        DecisionsCommands::List(a) => {
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let params = ListTriageDecisionsParams { per_page: Some(500), cursor: cursor.clone() };
                    let resp = list_triage_decisions(&client, &params)
                        .await
                        .map_err(|e| explain_error(e, None))?;
                    all_items.extend(resp.data);
                    // meta.cursor is null on the last page and is the only end signal.
                    // A page can hold fewer than perPage and still have a next one.
                    cursor = resp.meta.cursor;
                    if cursor.is_none() { break; }
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let params = ListTriageDecisionsParams { per_page: Some(a.per_page), cursor: a.cursor.clone() };
                let resp = list_triage_decisions(&client, &params)
                    .await
                    .map_err(|e| explain_error(e, None))?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
                if let Some(next) = resp.meta.cursor {
                    eprintln!("More decisions follow. Next page: --cursor {next}");
                }
            }
        }
        DecisionsCommands::Get { id } => {
            let resp = get_triage_decision(&client, id).await.map_err(|e| explain_error(e, Some(id)))?;
            render_decision(&resp.data, config)?;
        }
        DecisionsCommands::Create(a) => {
            let body = create_body(a)?;
            let resp = create_triage_decision(&client, &body).await.map_err(|e| explain_error(e, None))?;
            render_decision(&resp.data, config)?;
        }
        DecisionsCommands::Edit(a) => {
            let input = EditInput::from_args(a)?;
            let current = if input.needs_current() {
                let resp = get_triage_decision(&client, &a.id)
                    .await
                    .map_err(|e| explain_error(e, Some(&a.id)))?;
                Some(resp.data)
            } else {
                None
            };
            let body = input.into_body(current.as_ref())?;
            let resp = update_triage_decision(&client, &a.id, &body)
                .await
                .map_err(|e| explain_error(e, Some(&a.id)))?;
            render_decision(&resp.data, config)?;
        }
        DecisionsCommands::Delete { id } => {
            delete_triage_decision(&client, id).await.map_err(|e| explain_error(e, Some(id)))?;
            println!("Triage decision '{id}' deleted.");
        }
    }
    Ok(())
}
