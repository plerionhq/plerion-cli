use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::tenant, models::tenant::ApiOperation};
use crate::config::Config;
use crate::output::{self, OutputFormat};

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
    /// Get, set or clear the tenant default home dashboard
    HomeDashboard(HomeDashboardArgs),
    /// List the API operations this API key may call.
    ///
    /// The table shows one row per operation. `--output json` or `yaml` returns
    /// the full OpenAPI document, limited to the key's role.
    ApiAccess,
}

#[derive(Args, Debug)]
pub struct HomeDashboardArgs {
    #[command(subcommand)]
    pub command: HomeDashboardCommands,
}

#[derive(Subcommand, Debug)]
pub enum HomeDashboardCommands {
    /// Get the tenant default home dashboard
    Get,
    /// Set the tenant default home dashboard, replacing any current default
    Set {
        /// Custom dashboard ID to use as the tenant default
        #[arg(long)]
        home_report_id: String,
    },
    /// Clear the tenant default, so users fall back to the built-in home page
    Clear,
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
        TenantCommands::HomeDashboard(h) => match &h.command {
            HomeDashboardCommands::Get => {
                let resp = tenant::get_home_dashboard(&client).await?;
                output::render(
                    &resp.data,
                    config.output,
                    config.query.as_deref(),
                    config.no_color,
                )?;
            }
            HomeDashboardCommands::Set { home_report_id } => {
                if home_report_id.trim().is_empty() {
                    anyhow::bail!("--home-report-id must not be empty. Use `tenant home-dashboard clear` to remove the default.");
                }
                let resp = tenant::set_home_dashboard(&client, home_report_id).await?;
                output::render(
                    &resp.data,
                    config.output,
                    config.query.as_deref(),
                    config.no_color,
                )?;
            }
            HomeDashboardCommands::Clear => {
                tenant::clear_home_dashboard(&client).await?;
                println!("Tenant default home dashboard cleared.");
            }
        },
        TenantCommands::ApiAccess => {
            let doc = tenant::discover_api_access(&client).await?;
            let raw = config.query.is_some()
                || matches!(config.output, OutputFormat::Json | OutputFormat::Yaml);
            if raw {
                output::render_json_value(&doc, config.output, config.query.as_deref())?;
            } else {
                let ops = ApiOperation::from_openapi(&doc);
                output::render_list(&ops, config.output, None, config.no_color)?;
            }
        }
    }
    Ok(())
}
