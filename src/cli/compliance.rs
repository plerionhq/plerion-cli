use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::compliance::*};
use crate::config::Config;
use crate::output;
use std::io::Write;

#[derive(Args, Debug)]
pub struct ComplianceArgs {
    #[command(subcommand)]
    pub command: ComplianceCommands,
}

#[derive(Subcommand, Debug)]
pub enum ComplianceCommands {
    List(ListComplianceArgs),
    RequestReport { #[arg(long)] integration_id: String, #[arg(long)] framework_id: String },
    Download(DownloadComplianceArgs),
}

#[derive(Args, Debug)]
pub struct ListComplianceArgs {
    #[arg(long)] pub custom: Option<bool>,
}

#[derive(Args, Debug)]
pub struct DownloadComplianceArgs {
    #[arg(long)] pub integration_id: String,
    #[arg(long)] pub framework_id: String,
    #[arg(long)] pub output_file: Option<String>,
}

pub async fn run(args: &ComplianceArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        ComplianceCommands::List(a) => {
            let resp = list_compliance_frameworks(&client, a.custom).await?;
            output::render_list(&resp.data.frameworks, config.output, config.query.as_deref(), config.no_color)?;
        }
        ComplianceCommands::RequestReport { integration_id, framework_id } => {
            let resp = request_compliance_report(&client, integration_id, framework_id).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
        ComplianceCommands::Download(a) => {
            let bytes = download_compliance_report(&client, &a.integration_id, &a.framework_id).await?;
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
