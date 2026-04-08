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
    #[arg(long)] pub status: Option<String>,
    #[arg(long)] pub provider: Option<String>,
    #[arg(long)] pub alert_type: Option<String>,
    #[arg(long)] pub integration_id: Option<String>,
    #[arg(long)] pub flagged: Option<bool>,
    #[arg(long)] pub acknowledged: Option<bool>,
    #[arg(long)] pub sort_by: Option<String>,
    #[arg(long)] pub sort_order: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
}

pub async fn run(args: &AlertsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        AlertsCommands::List(a) => {
            let params = ListAlertsParams {
                statuses: a.status.clone(),
                providers: a.provider.clone(),
                alert_types: a.alert_type.clone(),
                integration_ids: a.integration_id.clone(),
                flagged: a.flagged,
                acknowledged: a.acknowledged,
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            let resp = list_alerts(&client, &params).await?;
            output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
    }
    Ok(())
}
