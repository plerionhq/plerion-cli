use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::tenant};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct TenantArgs {
    #[command(subcommand)]
    pub command: TenantCommands,
}

#[derive(Subcommand, Debug)]
pub enum TenantCommands {
    /// Get tenant details
    Get,
    /// Get tenant usage details
    GetUsage {
        /// Usage date in yyyy-MM-dd format (defaults to today)
        #[arg(long)]
        date: Option<String>,
    },
}

pub async fn run(args: &TenantArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        TenantCommands::Get => {
            let resp = tenant::get_tenant(&client).await?;
            output::render(
                &resp.data,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
        TenantCommands::GetUsage { date } => {
            let resp = tenant::get_tenant_usage(&client, date.as_deref()).await?;
            output::render(
                &resp.data,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
    }
    Ok(())
}
