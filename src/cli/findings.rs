use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::findings::{list_findings, ListFindingsParams}};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct FindingsArgs {
    #[command(subcommand)]
    pub command: FindingsCommands,
}

#[derive(Subcommand, Debug)]
pub enum FindingsCommands {
    /// List findings
    List(ListFindingsArgs),
}

#[derive(Args, Debug)]
pub struct ListFindingsArgs {
    /// Filter by finding IDs (comma-separated)
    #[arg(long)]
    pub ids: Option<String>,

    /// Filter by asset IDs (comma-separated)
    #[arg(long)]
    pub asset_id: Option<String>,

    /// Filter by severity: CRITICAL,HIGH,MEDIUM,LOW
    #[arg(long)]
    pub severity: Option<String>,

    /// Filter by status: PASSED,FAILED
    #[arg(long)]
    pub status: Option<String>,

    /// Filter by provider: AWS,Azure,GCP,Kubernetes
    #[arg(long)]
    pub provider: Option<String>,

    /// Filter by regions (comma-separated)
    #[arg(long)]
    pub region: Option<String>,

    /// Filter by resource types (comma-separated)
    #[arg(long)]
    pub resource_type: Option<String>,

    /// Filter by detection IDs (comma-separated)
    #[arg(long)]
    pub detection_id: Option<String>,

    /// Filter by integration IDs (comma-separated)
    #[arg(long)]
    pub integration_id: Option<String>,

    /// Filter by asset group IDs (comma-separated)
    #[arg(long)]
    pub asset_group_id: Option<String>,

    /// Filter by environment IDs or names (comma-separated)
    #[arg(long)]
    pub environment_id: Option<String>,

    /// Filter by services (comma-separated)
    #[arg(long)]
    pub service: Option<String>,

    /// Filter exempted findings only
    #[arg(long)]
    pub is_exempted: bool,

    /// First observed at start date (ISO 8601)
    #[arg(long)]
    pub start: Option<String>,

    /// First observed at end date (ISO 8601)
    #[arg(long)]
    pub end: Option<String>,

    /// Sort by field
    #[arg(long)]
    pub sort_by: Option<String>,

    /// Sort order: ASC or DESC
    #[arg(long)]
    pub sort_order: Option<String>,

    /// Number of results per page (max 1000)
    #[arg(long, default_value = "50")]
    pub per_page: u32,

    /// Fetch all pages automatically
    #[arg(long)]
    pub all: bool,
}

pub async fn run(args: &FindingsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        FindingsCommands::List(list_args) => {
            let params = ListFindingsParams {
                ids: list_args.ids.clone(),
                asset_ids: list_args.asset_id.clone(),
                severity_levels: list_args.severity.clone(),
                statuses: list_args.status.clone(),
                providers: list_args.provider.clone(),
                regions: list_args.region.clone(),
                resource_types: list_args.resource_type.clone(),
                detection_ids: list_args.detection_id.clone(),
                integration_ids: list_args.integration_id.clone(),
                asset_group_ids: list_args.asset_group_id.clone(),
                environment_ids: list_args.environment_id.clone(),
                services: list_args.service.clone(),
                is_exempted: if list_args.is_exempted { Some(true) } else { None },
                first_observed_at_start: list_args.start.clone(),
                first_observed_at_end: list_args.end.clone(),
                sort_by: list_args.sort_by.clone(),
                sort_order: list_args.sort_order.clone(),
                per_page: Some(list_args.per_page),
                ..Default::default()
            };

            if list_args.all {
                let mut all_findings = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let p = ListFindingsParams {
                        cursor: cursor.clone(),
                        ..ListFindingsParams {
                            ids: params.ids.clone(),
                            asset_ids: params.asset_ids.clone(),
                            severity_levels: params.severity_levels.clone(),
                            statuses: params.statuses.clone(),
                            providers: params.providers.clone(),
                            regions: params.regions.clone(),
                            resource_types: params.resource_types.clone(),
                            detection_ids: params.detection_ids.clone(),
                            integration_ids: params.integration_ids.clone(),
                            asset_group_ids: params.asset_group_ids.clone(),
                            environment_ids: params.environment_ids.clone(),
                            services: params.services.clone(),
                            is_exempted: params.is_exempted,
                            first_observed_at_start: params.first_observed_at_start.clone(),
                            first_observed_at_end: params.first_observed_at_end.clone(),
                            sort_by: params.sort_by.clone(),
                            sort_order: params.sort_order.clone(),
                            per_page: Some(1000),
                            ..Default::default()
                        }
                    };
                    let resp = list_findings(&client, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    cursor = resp.meta.cursor.clone();
                    all_findings.extend(resp.data);
                    if !has_next { break; }
                }
                output::render_list(&all_findings, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_findings(&client, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
    }
    Ok(())
}
