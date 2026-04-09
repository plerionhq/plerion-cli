use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::well_architected::*};
use crate::config::Config;
use crate::output;
use std::io::Write;

#[derive(Args, Debug)]
pub struct WellArchitectedArgs {
    #[command(subcommand)]
    pub command: WellArchitectedCommands,
}

#[derive(Subcommand, Debug)]
pub enum WellArchitectedCommands {
    List,
    RequestReport { #[arg(long)] integration_id: String, #[arg(long)] framework_id: String },
    Download(DownloadWaArgs),
}

#[derive(Args, Debug)]
pub struct DownloadWaArgs {
    #[arg(long)] pub integration_id: String,
    #[arg(long)] pub framework_id: String,
    #[arg(long)] pub output_file: Option<String>,
}

pub async fn run(args: &WellArchitectedArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        WellArchitectedCommands::List => {
            let resp = list_well_architected_frameworks(&client).await?;
            output::render_list(&resp.data.frameworks, config.output, config.query.as_deref(), config.no_color)?;
        }
        WellArchitectedCommands::RequestReport { integration_id, framework_id } => {
            let resp = request_well_architected_report(&client, integration_id, framework_id).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
        WellArchitectedCommands::Download(a) => {
            let bytes = download_well_architected_report(&client, &a.integration_id, &a.framework_id).await?;
            if let Some(path) = &a.output_file {
                std::fs::write(path, &bytes)?;
                println!("Report saved to {path}");
            } else {
                std::io::stdout().write_all(&bytes)?;
            }
        }
    }
    Ok(())
}
