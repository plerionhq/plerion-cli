mod api;
mod cli;
mod config;
mod error;
mod output;

use clap::Parser;
use cli::{Cli, Commands};
use config::{Config, ConfigOverrides};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let overrides = ConfigOverrides {
        profile: cli.profile.clone(),
        api_key: cli.api_key.clone(),
        region: cli.region.clone(),
        endpoint_url: cli.endpoint_url.clone(),
        output: cli.output,
        no_color: cli.no_color,
        query: cli.query.clone(),
    };

    // Configure subcommand doesn't need an API key
    if let Commands::Configure(args) = &cli.command {
        if let Err(e) = cli::configure::run(args).await {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        return;
    }

    let config = match Config::load(overrides) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let result = match &cli.command {
        Commands::Configure(_) => unreachable!(),
        Commands::Tenant(args) => cli::tenant::run(args, &config).await,
        Commands::Findings(args) => cli::findings::run(args, &config).await,
        Commands::Assets(args) => cli::assets::run(args, &config).await,
        Commands::AssetGroups(args) => cli::asset_groups::run(args, &config).await,
        Commands::Alerts(args) => cli::alerts::run(args, &config).await,
        Commands::AuditLogs(args) => cli::audit_logs::run(args, &config).await,
        Commands::Integrations(args) => cli::integrations::run(args, &config).await,
        Commands::Risks(args) => cli::risks::run(args, &config).await,
        Commands::Vulnerabilities(args) => cli::vulnerabilities::run(args, &config).await,
        Commands::ComplianceFrameworks(args) => cli::compliance::run(args, &config).await,
        Commands::WellArchitectedFrameworks(args) => cli::well_architected::run(args, &config).await,
        Commands::Iac(args) => cli::iac::run(args, &config).await,
        Commands::Aws(args) => cli::aws::run(args, &config).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
