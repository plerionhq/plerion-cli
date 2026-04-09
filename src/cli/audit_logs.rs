use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::audit_logs::list_audit_logs};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct AuditLogsArgs {
    #[command(subcommand)]
    pub command: AuditLogsCommands,
}

#[derive(Subcommand, Debug)]
pub enum AuditLogsCommands {
    List(ListAuditLogsArgs),
}

#[derive(Args, Debug)]
pub struct ListAuditLogsArgs {
    #[arg(long)] pub start: Option<String>,
    #[arg(long)] pub end: Option<String>,
    #[arg(long)] pub user_id: Option<String>,
    #[arg(long)] pub operation: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)] pub all: bool,
}

pub async fn run(args: &AuditLogsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        AuditLogsCommands::List(a) => {
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let resp = list_audit_logs(
                        &client,
                        a.start.as_deref(),
                        a.end.as_deref(),
                        a.user_id.as_deref(),
                        a.operation.as_deref(),
                        Some(1000),
                        cursor.as_deref(),
                    ).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    cursor = resp.meta.cursor.clone();
                    all_items.extend(resp.data);
                    if !has_next { break; }
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_audit_logs(
                    &client,
                    a.start.as_deref(),
                    a.end.as_deref(),
                    a.user_id.as_deref(),
                    a.operation.as_deref(),
                    Some(a.per_page),
                    None,
                ).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
    }
    Ok(())
}
