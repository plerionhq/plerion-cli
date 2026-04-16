use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::alerts::{list_alerts, ListAlertsParams}};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct AlertsArgs {
    #[command(subcommand)]
    pub command: AlertsCommands,
}

#[derive(Subcommand, Debug)]
pub enum AlertsCommands {
    List(ListAlertsArgs),
}

#[derive(Args, Debug)]
pub struct ListAlertsArgs {
    #[arg(long)] pub ids: Option<String>,
    #[arg(long)] pub workflow_id: Option<String>,
    #[arg(long)] pub asset_group_id: Option<String>,
    #[arg(long)] pub resource_type: Option<String>,
    #[arg(long)] pub status: Option<String>,
    #[arg(long)] pub provider: Option<String>,
    #[arg(long)] pub alert_type: Option<String>,
    #[arg(long)] pub integration_id: Option<String>,
    #[arg(long)] pub flagged: Option<bool>,
    #[arg(long)] pub acknowledged: Option<bool>,
    /// Sort by field
    #[arg(long, value_parser = ["riskScore", "discoveredDate"])]
    pub sort_by: Option<String>,
    /// Sort order
    #[arg(long, value_parser = ["ASC", "DESC"])]
    pub sort_order: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)] pub all: bool,
}

pub async fn run(args: &AlertsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        AlertsCommands::List(a) => {
            let params = ListAlertsParams {
                ids: a.ids.clone(),
                workflow_ids: a.workflow_id.clone(),
                asset_group_ids: a.asset_group_id.clone(),
                resource_types: a.resource_type.clone(),
                statuses: a.status.clone(),
                providers: super::normalize_providers(a.provider.clone()),
                alert_types: a.alert_type.clone(),
                integration_ids: a.integration_id.clone(),
                flagged: a.flagged,
                acknowledged: a.acknowledged,
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let p = ListAlertsParams {
                        cursor: cursor.clone(),
                        per_page: Some(1000),
                        ids: params.ids.clone(),
                        workflow_ids: params.workflow_ids.clone(),
                        asset_group_ids: params.asset_group_ids.clone(),
                        resource_types: params.resource_types.clone(),
                        statuses: params.statuses.clone(),
                        providers: params.providers.clone(),
                        alert_types: params.alert_types.clone(),
                        integration_ids: params.integration_ids.clone(),
                        flagged: params.flagged,
                        acknowledged: params.acknowledged,
                        sort_by: params.sort_by.clone(),
                        sort_order: params.sort_order.clone(),
                    };
                    let resp = list_alerts(&client, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    cursor = resp.meta.cursor.clone();
                    all_items.extend(resp.data);
                    if !has_next { break; }
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_alerts(&client, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
    }
    Ok(())
}
