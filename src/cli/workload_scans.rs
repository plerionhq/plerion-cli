use crate::api::{
    client::PlerionClient,
    endpoints::workload_scans::{get_workload_scan, request_workload_scan},
    models::workload_scans::RequestWorkloadScanRequest,
};
use crate::config::Config;
use crate::output;
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct WorkloadScansArgs {
    #[command(subcommand)]
    pub command: WorkloadScansCommands,
}

#[derive(Subcommand, Debug)]
pub enum WorkloadScansCommands {
    /// Scan one workload now instead of waiting for the next scheduled scan
    Request(RequestWorkloadScanArgs),
    /// Get the status of a requested workload scan
    Get {
        /// Scan ID returned by `workload-scans request`
        scan_id: String,
    },
}

#[derive(Args, Debug)]
pub struct RequestWorkloadScanArgs {
    /// Integration the workload belongs to (UUID)
    #[arg(long)]
    pub integration_id: String,
    /// Plerion asset ID of the workload; or pass --resource-type, --resource-region and --resource-id
    #[arg(long)]
    pub asset_id: Option<String>,
    /// Provider resource type, when --asset-id is not given
    #[arg(long, value_parser = ["ec2:instance", "ec2:image"])]
    pub resource_type: Option<String>,
    /// Provider region of the resource (e.g. ap-southeast-2), when --asset-id is not given
    #[arg(long)]
    pub resource_region: Option<String>,
    /// Provider resource ID (e.g. i-0123456789abcdef0), when --asset-id is not given
    #[arg(long)]
    pub resource_id: Option<String>,
}

fn non_empty(v: &Option<String>) -> Option<String> {
    v.as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// The API takes `assetId` alone, or the three resource fields together.
fn build_request(a: &RequestWorkloadScanArgs) -> anyhow::Result<RequestWorkloadScanRequest> {
    let integration_id = a.integration_id.trim().to_string();
    if integration_id.is_empty() {
        anyhow::bail!("--integration-id must not be empty.");
    }
    let asset_id = non_empty(&a.asset_id);
    let resource_type = non_empty(&a.resource_type);
    let resource_region = non_empty(&a.resource_region);
    let resource_id = non_empty(&a.resource_id);
    let any_resource =
        resource_type.is_some() || resource_region.is_some() || resource_id.is_some();
    let all_resource =
        resource_type.is_some() && resource_region.is_some() && resource_id.is_some();

    match (&asset_id, any_resource, all_resource) {
        (Some(_), false, _) | (None, true, true) => Ok(RequestWorkloadScanRequest {
            integration_id,
            asset_id,
            resource_type,
            resource_region,
            resource_id,
        }),
        (Some(_), true, _) => anyhow::bail!(
            "Pass either --asset-id, or --resource-type, --resource-region and --resource-id, not both."
        ),
        _ => anyhow::bail!(
            "Name the workload to scan: pass --asset-id, or all of --resource-type, \
             --resource-region and --resource-id."
        ),
    }
}

pub async fn run(args: &WorkloadScansArgs, config: &Config) -> anyhow::Result<()> {
    match &args.command {
        WorkloadScansCommands::Request(a) => {
            let body = build_request(a)?;
            let client = PlerionClient::new(config)?;
            let resp = request_workload_scan(&client, &body).await?;
            output::render(
                &resp,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
        WorkloadScansCommands::Get { scan_id } => {
            let client = PlerionClient::new(config)?;
            let resp = get_workload_scan(&client, scan_id).await?;
            output::render(
                &resp,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
    }
    Ok(())
}
