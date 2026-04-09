pub mod alerts;
pub mod asset_groups;
pub mod assets;
pub mod audit_logs;
pub mod aws;
pub mod compliance;
pub mod configure;
pub mod findings;
pub mod iac;
pub mod integrations;
pub mod risks;
pub mod tenant;
pub mod vulnerabilities;
pub mod well_architected;

use clap::{Parser, Subcommand};
use crate::output::OutputFormat;

#[derive(Parser, Debug)]
#[command(
    name = "plerion",
    version,
    about = "Plerion CLI — manage cloud security findings, assets, risks and more",
    long_about = None
)]
pub struct Cli {
    /// AWS CLI-style profile name from ~/.plerion/credentials
    #[arg(long, global = true, env = "PLERION_PROFILE")]
    pub profile: Option<String>,

    /// Override the API region (au, sg1, in1, us1)
    #[arg(long, global = true, env = "PLERION_REGION")]
    pub region: Option<String>,

    /// Override the API key
    #[arg(long, global = true, env = "PLERION_API_KEY")]
    pub api_key: Option<String>,

    /// Override the API base URL (e.g. https://au.develop2.plerionaut.com)
    #[arg(long, global = true, env = "PLERION_ENDPOINT_URL")]
    pub endpoint_url: Option<String>,

    /// Output format: table (default), json, yaml, text
    #[arg(long, global = true, value_parser = parse_output_format)]
    pub output: Option<OutputFormat>,

    /// JMESPath query to filter/transform output
    #[arg(long, global = true)]
    pub query: Option<String>,

    /// Disable color output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

fn parse_output_format(s: &str) -> Result<OutputFormat, String> {
    s.parse()
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure Plerion CLI credentials and settings
    Configure(configure::ConfigureArgs),
    /// Tenant operations
    Tenant(tenant::TenantArgs),
    /// List and filter security findings
    Findings(findings::FindingsArgs),
    /// Asset inventory operations
    Assets(assets::AssetsArgs),
    /// Asset group management
    AssetGroups(asset_groups::AssetGroupsArgs),
    /// Alert management
    Alerts(alerts::AlertsArgs),
    /// Audit log operations
    AuditLogs(audit_logs::AuditLogsArgs),
    /// Integration management
    Integrations(integrations::IntegrationsArgs),
    /// Risk management
    Risks(risks::RisksArgs),
    /// Vulnerability management
    Vulnerabilities(vulnerabilities::VulnerabilitiesArgs),
    /// Compliance framework operations
    ComplianceFrameworks(compliance::ComplianceArgs),
    /// Well-Architected Framework operations
    WellArchitectedFrameworks(well_architected::WellArchitectedArgs),
    /// Infrastructure as Code scanning
    Iac(iac::IacArgs),
    /// AWS integration helpers
    Aws(aws::AwsArgs),
}
