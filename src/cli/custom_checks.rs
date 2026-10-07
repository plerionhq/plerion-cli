use std::io::Read;

use clap::{Args, Subcommand};

use crate::api::client::PlerionClient;
use crate::api::endpoints::custom_checks::{
    create_custom_check, delete_custom_check, get_custom_check, get_custom_check_dry_run_status,
    list_custom_checks, start_custom_check_dry_run, update_custom_check, ListCustomChecksParams,
};
use crate::api::models::custom_checks::StartCustomCheckDryRunRequest;
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct CustomChecksArgs {
    #[command(subcommand)]
    pub command: CustomChecksCommands,
}

#[derive(Subcommand, Debug)]
pub enum CustomChecksCommands {
    /// List custom checks
    List(ListCustomChecksArgs),
    /// Get a custom check by ID
    Get {
        /// Custom check ID
        #[arg(long)]
        id: String,
    },
    /// Create a custom check from a JSON definition
    Create {
        /// JSON file with the check definition (slug, title, target, defaults, type, body), or - for stdin
        #[arg(long)]
        file: String,
    },
    /// Replace a custom check in full from a JSON definition
    Update {
        /// Custom check ID
        #[arg(long)]
        id: String,
        /// JSON file with the full check definition, or - for stdin
        #[arg(long)]
        file: String,
    },
    /// Delete a custom check
    Delete {
        /// Custom check ID
        #[arg(long)]
        id: String,
    },
    /// Run a check against live assets without saving findings
    DryRun {
        /// Integration whose assets the check runs against
        #[arg(long)]
        integration_id: String,
        /// JSON file with the check to run (needs customCheckId, target and body), or - for stdin
        #[arg(long)]
        file: String,
    },
    /// Get a dry run's status and findings count; findings are in --output json
    DryRunStatus {
        /// Dry run ID returned by dry-run
        #[arg(long)]
        id: String,
    },
}

#[derive(Args, Debug)]
pub struct ListCustomChecksArgs {
    /// Asset type the check targets (e.g. AWS::S3::Bucket)
    #[arg(long)]
    pub asset_type: Option<String>,
    /// Target scope
    #[arg(long, value_parser = ["batch", "all"])]
    pub scope: Option<String>,
    /// Checks per page (1 to 200)
    #[arg(long, default_value = "50", value_parser = clap::value_parser!(u32).range(1..=200))]
    pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)]
    pub all: bool,
}

/// Fields the server sets. They appear in `get` output but are rejected on
/// write, so they are dropped to let that output be edited and sent back.
const SERVER_FIELDS: &[&str] = &[
    "organizationId",
    "tenantId",
    "version",
    "createdBy",
    "createdAt",
    "updatedBy",
    "updatedAt",
];

/// Reads a JSON object from a file, or stdin when `path` is `-`.
fn read_definition(path: &str) -> anyhow::Result<serde_json::Map<String, serde_json::Value>> {
    let raw = if path == "-" {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read stdin: {e}"))?;
        buf
    } else {
        std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read {path}: {e}"))?
    };
    let value: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|e| anyhow::anyhow!("Invalid JSON in --file {path}: {e}"))?;
    match value {
        serde_json::Value::Object(map) if !map.is_empty() => Ok(map),
        serde_json::Value::Object(_) => {
            anyhow::bail!(
                "--file {path} is an empty object; it must hold a custom check definition"
            )
        }
        _ => anyhow::bail!("--file {path} must hold a JSON object"),
    }
}

/// The body for create and update: the definition minus server-set fields and
/// the ID, which travels in the path.
fn check_input(path: &str) -> anyhow::Result<serde_json::Value> {
    let mut map = read_definition(path)?;
    for key in SERVER_FIELDS {
        map.remove(*key);
    }
    map.remove("customCheckId");
    Ok(serde_json::Value::Object(map))
}

/// The check for a dry run keeps its ID, which labels the results.
fn dry_run_check(path: &str) -> anyhow::Result<serde_json::Value> {
    let mut map = read_definition(path)?;
    for key in SERVER_FIELDS {
        map.remove(*key);
    }
    Ok(serde_json::Value::Object(map))
}

pub async fn run(args: &CustomChecksArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        CustomChecksCommands::List(a) => {
            let params = ListCustomChecksParams {
                asset_type: a.asset_type.clone(),
                scope: a.scope.clone(),
                limit: Some(a.per_page),
                cursor: None,
            };
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let p = ListCustomChecksParams {
                        limit: Some(200),
                        cursor: cursor.clone(),
                        ..params.clone()
                    };
                    let resp = list_custom_checks(&client, &p).await?;
                    all_items.extend(resp.items);
                    // nextCursor is absent on the last page.
                    cursor = resp.next_cursor;
                    if cursor.is_none() {
                        break;
                    }
                }
                output::render_list(
                    &all_items,
                    config.output,
                    config.query.as_deref(),
                    config.no_color,
                )?;
            } else {
                let resp = list_custom_checks(&client, &params).await?;
                output::render_list(
                    &resp.items,
                    config.output,
                    config.query.as_deref(),
                    config.no_color,
                )?;
            }
        }
        CustomChecksCommands::Get { id } => {
            let resp = get_custom_check(&client, id).await?;
            output::render(
                &resp,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
        CustomChecksCommands::Create { file } => {
            let body = check_input(file)?;
            let resp = create_custom_check(&client, &body).await?;
            output::render(
                &resp,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
        CustomChecksCommands::Update { id, file } => {
            let body = check_input(file)?;
            let resp = update_custom_check(&client, id, &body).await?;
            output::render(
                &resp,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
        CustomChecksCommands::Delete { id } => {
            delete_custom_check(&client, id).await?;
            println!("Custom check '{id}' deleted. Its findings are cleaned up in the background.");
        }
        CustomChecksCommands::DryRun {
            integration_id,
            file,
        } => {
            let body = StartCustomCheckDryRunRequest {
                integration_id: integration_id.clone(),
                check: dry_run_check(file)?,
            };
            let resp = start_custom_check_dry_run(&client, &body).await?;
            output::render(
                &resp,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
        CustomChecksCommands::DryRunStatus { id } => {
            let resp = get_custom_check_dry_run_status(&client, id).await?;
            output::render(
                &resp,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
    }
    Ok(())
}
