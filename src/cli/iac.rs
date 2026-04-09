use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::iac::*};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct IacArgs {
    #[command(subcommand)]
    pub command: IacCommands,
}

#[derive(Subcommand, Debug)]
pub enum IacCommands {
    /// Upload a zip file for IaC scanning
    Scan(ScanArgs),
    /// List IaC scans
    ListScans,
    /// Get findings for a scan
    GetFindings {
        /// The scan ID returned from `iac scan`
        #[arg(long)]
        scan_id: String,
    },
    /// Get vulnerabilities for a scan
    GetVulnerabilities {
        /// The scan ID returned from `iac scan`
        #[arg(long)]
        scan_id: String,
    },
}

#[derive(Args, Debug)]
pub struct ScanArgs {
    /// Path to the zip file to scan
    #[arg(long)]
    pub file: String,
    /// Artifact name (e.g. my-iac.zip)
    #[arg(long)]
    pub name: String,
}

pub async fn run(args: &IacArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        IacCommands::Scan(a) => {
            let bytes = std::fs::read(&a.file)
                .map_err(|e| anyhow::anyhow!("Failed to read {}: {e}", a.file))?;
            let resp = upload_iac_scan(&client, &a.name, bytes::Bytes::from(bytes)).await?;
            println!("Scan submitted. Scan ID: {}", resp.meta.scan_id.unwrap_or_default());
        }
        IacCommands::ListScans => {
            let resp = list_iac_scans(&client).await?;
            output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        IacCommands::GetFindings { scan_id } => {
            let resp = get_iac_findings(&client, scan_id).await?;
            output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        IacCommands::GetVulnerabilities { scan_id } => {
            let resp = get_iac_vulnerabilities(&client, scan_id).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
    }
    Ok(())
}
