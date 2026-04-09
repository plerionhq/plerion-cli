use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::integrations::list_integrations};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct IntegrationsArgs {
    #[command(subcommand)]
    pub command: IntegrationsCommands,
}

#[derive(Subcommand, Debug)]
pub enum IntegrationsCommands {
    List(ListIntegrationsArgs),
}

#[derive(Args, Debug)]
pub struct ListIntegrationsArgs {
    #[arg(long, default_value = "50")] pub per_page: u32,
    #[arg(long)] pub include_total: bool,
}

pub async fn run(args: &IntegrationsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        IntegrationsCommands::List(a) => {
            let resp = list_integrations(&client, Some(a.per_page), None, a.include_total).await?;
            output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
    }
    Ok(())
}
