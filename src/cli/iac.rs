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
    ListScans(ListScansArgs),
    /// Get findings for a scan
    GetFindings(GetFindingsArgs),
    /// Get vulnerabilities for a scan
    GetVulnerabilities(GetVulnerabilitiesArgs),
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

#[derive(Args, Debug)]
pub struct ListScansArgs {
    /// Filter by IDs (comma-separated)
    #[arg(long)]
    pub ids: Option<String>,
    /// Filter by artifact names (comma-separated)
    #[arg(long)]
    pub artifact_name: Option<String>,
    /// Filter by statuses: SUCCESS, FAILURE (comma-separated)
    #[arg(long)]
    pub status: Option<String>,
    /// Sort by field
    #[arg(long, value_parser = ["id", "createdAt", "updatedAt", "artifactName", "status"])]
    pub sort_by: Option<String>,
    /// Sort order
    #[arg(long, value_parser = ["asc", "desc"])]
    pub sort_order: Option<String>,
    /// Number of results per page (max 1000)
    #[arg(long, default_value = "50")]
    pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct GetFindingsArgs {
    /// The scan ID returned from `iac scan`
    #[arg(long)]
    pub scan_id: String,
    /// Filter by IDs (comma-separated)
    #[arg(long)]
    pub ids: Option<String>,
    /// Filter by result status: PASSED, FAILED (comma-separated)
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by severity level: CRITICAL, HIGH, MEDIUM, LOW (comma-separated)
    #[arg(long)]
    pub severity: Option<String>,
    /// Filter by detection IDs (comma-separated)
    #[arg(long)]
    pub detection_id: Option<String>,
    /// Filter by types (comma-separated, e.g. helm,kubernetes)
    #[arg(long, name = "type")]
    pub finding_type: Option<String>,
    /// Filter by files (comma-separated)
    #[arg(long)]
    pub file: Option<String>,
    /// Sort by field
    #[arg(long, value_parser = ["id", "createdAt", "updatedAt", "artifactName", "status"])]
    pub sort_by: Option<String>,
    /// Sort order
    #[arg(long, value_parser = ["asc", "desc"])]
    pub sort_order: Option<String>,
    /// Number of results per page (max 1000)
    #[arg(long, default_value = "50")]
    pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct GetVulnerabilitiesArgs {
    /// The scan ID returned from `iac scan`
    #[arg(long)]
    pub scan_id: String,
    /// Filter by IDs (comma-separated)
    #[arg(long)]
    pub ids: Option<String>,
    /// Filter by severity level: CRITICAL, HIGH, MEDIUM, LOW (comma-separated)
    #[arg(long)]
    pub severity: Option<String>,
    /// Filter by vulnerability IDs (comma-separated, e.g. CVE-2022-22965)
    #[arg(long)]
    pub vulnerability_id: Option<String>,
    /// Filter by severity sources (comma-separated, e.g. nvd,github)
    #[arg(long)]
    pub severity_source: Option<String>,
    /// Filter by files (comma-separated)
    #[arg(long)]
    pub file: Option<String>,
    /// Filter by KEV status
    #[arg(long)]
    pub has_kev: bool,
    /// Filter by exploit availability
    #[arg(long)]
    pub has_exploit: bool,
    /// Sort by field
    #[arg(long, value_parser = ["id", "vulnerabilityId", "severityLevel", "severitySource",
        "hasKev", "hasExploit", "file", "createdAt", "updatedAt"])]
    pub sort_by: Option<String>,
    /// Sort order
    #[arg(long, value_parser = ["asc", "desc"])]
    pub sort_order: Option<String>,
    /// Number of results per page (max 1000)
    #[arg(long, default_value = "50")]
    pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)]
    pub all: bool,
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
        IacCommands::ListScans(a) => {
            let params = ListIacScansParams {
                ids: a.ids.clone(),
                artifact_names: a.artifact_name.clone(),
                statuses: a.status.clone(),
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            if a.all {
                let mut all_items = Vec::new();
                let mut page = 1u32;
                loop {
                    let p = ListIacScansParams {
                        page: Some(page),
                        per_page: Some(1000),
                        ids: params.ids.clone(),
                        artifact_names: params.artifact_names.clone(),
                        statuses: params.statuses.clone(),
                        sort_by: params.sort_by.clone(),
                        sort_order: params.sort_order.clone(),
                    };
                    let resp = list_iac_scans(&client, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    all_items.extend(resp.data);
                    if !has_next { break; }
                    page += 1;
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_iac_scans(&client, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
        IacCommands::GetFindings(a) => {
            let params = ListIacFindingsParams {
                ids: a.ids.clone(),
                results: a.status.clone(),
                severity_levels: a.severity.clone(),
                detection_ids: a.detection_id.clone(),
                types: a.finding_type.clone(),
                files: a.file.clone(),
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            if a.all {
                let mut all_items = Vec::new();
                let mut page = 1u32;
                loop {
                    let p = ListIacFindingsParams {
                        page: Some(page),
                        per_page: Some(1000),
                        ids: params.ids.clone(),
                        results: params.results.clone(),
                        severity_levels: params.severity_levels.clone(),
                        detection_ids: params.detection_ids.clone(),
                        types: params.types.clone(),
                        files: params.files.clone(),
                        sort_by: params.sort_by.clone(),
                        sort_order: params.sort_order.clone(),
                    };
                    let resp = get_iac_findings(&client, &a.scan_id, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    all_items.extend(resp.data);
                    if !has_next { break; }
                    page += 1;
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = get_iac_findings(&client, &a.scan_id, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
        IacCommands::GetVulnerabilities(a) => {
            let params = ListIacVulnerabilitiesParams {
                ids: a.ids.clone(),
                severity_levels: a.severity.clone(),
                vulnerability_ids: a.vulnerability_id.clone(),
                severity_sources: a.severity_source.clone(),
                files: a.file.clone(),
                has_kevs: if a.has_kev { Some(true) } else { None },
                has_exploits: if a.has_exploit { Some(true) } else { None },
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            if a.all {
                let mut all_items = Vec::new();
                let mut page = 1u32;
                loop {
                    let p = ListIacVulnerabilitiesParams {
                        page: Some(page),
                        per_page: Some(1000),
                        ids: params.ids.clone(),
                        severity_levels: params.severity_levels.clone(),
                        vulnerability_ids: params.vulnerability_ids.clone(),
                        severity_sources: params.severity_sources.clone(),
                        files: params.files.clone(),
                        has_kevs: params.has_kevs,
                        has_exploits: params.has_exploits,
                        sort_by: params.sort_by.clone(),
                        sort_order: params.sort_order.clone(),
                    };
                    let resp = get_iac_vulnerabilities(&client, &a.scan_id, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    all_items.extend(resp.data);
                    if !has_next { break; }
                    page += 1;
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = get_iac_vulnerabilities(&client, &a.scan_id, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
    }
    Ok(())
}
