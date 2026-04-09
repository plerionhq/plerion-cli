use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::risks::{list_risks, ListRisksParams}};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct RisksArgs {
    #[command(subcommand)]
    pub command: RisksCommands,
}

#[derive(Subcommand, Debug)]
pub enum RisksCommands {
    List(ListRisksArgs),
}

#[derive(Args, Debug)]
pub struct ListRisksArgs {
    #[arg(long)] pub severity: Option<String>,
    #[arg(long)] pub lifecycle_state: Option<String>,
    #[arg(long)] pub integration_id: Option<String>,
    #[arg(long)] pub environment_id: Option<String>,
    #[arg(long)] pub resource_type: Option<String>,
    #[arg(long)] pub include: Option<String>,
    #[arg(long)] pub sort_by: Option<String>,
    #[arg(long)] pub sort_order: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)] pub all: bool,
}

pub async fn run(args: &RisksArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        RisksCommands::List(a) => {
            let params = ListRisksParams {
                severity_levels: a.severity.clone(),
                lifecycle_states: a.lifecycle_state.clone(),
                integration_ids: a.integration_id.clone(),
                environment_ids: a.environment_id.clone(),
                resource_types: a.resource_type.clone(),
                include: a.include.clone(),
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let p = ListRisksParams {
                        cursor: cursor.clone(),
                        per_page: Some(1000),
                        severity_levels: params.severity_levels.clone(),
                        lifecycle_states: params.lifecycle_states.clone(),
                        integration_ids: params.integration_ids.clone(),
                        environment_ids: params.environment_ids.clone(),
                        resource_types: params.resource_types.clone(),
                        include: params.include.clone(),
                        sort_by: params.sort_by.clone(),
                        sort_order: params.sort_order.clone(),
                        ..Default::default()
                    };
                    let resp = list_risks(&client, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    cursor = resp.meta.cursor.clone();
                    all_items.extend(resp.data);
                    if !has_next { break; }
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_risks(&client, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
    }
    Ok(())
}
