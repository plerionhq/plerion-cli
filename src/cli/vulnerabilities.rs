use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::vulnerabilities::*};

use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct VulnerabilitiesArgs {
    #[command(subcommand)]
    pub command: VulnerabilitiesCommands,
}

#[derive(Subcommand, Debug)]
pub enum VulnerabilitiesCommands {
    /// List vulnerabilities
    List(ListVulnArgs),
    /// Manage vulnerability exemptions
    Exemptions(ExemptionsArgs),
}

#[derive(Args, Debug)]
pub struct ListVulnArgs {
    #[arg(long)] pub severity: Option<String>,
    #[arg(long)] pub provider: Option<String>,
    #[arg(long)] pub has_kev: bool,
    #[arg(long)] pub has_exploit: bool,
    #[arg(long)] pub has_vendor_fix: bool,
    #[arg(long)] pub integration_id: Option<String>,
    #[arg(long)] pub asset_id: Option<String>,
    #[arg(long)] pub region: Option<String>,
    #[arg(long)] pub sort_by: Option<String>,
    #[arg(long)] pub sort_order: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
    #[arg(long)] pub all: bool,
}

#[derive(Args, Debug)]
pub struct ExemptionsArgs {
    #[command(subcommand)]
    pub command: ExemptionsCommands,
}

#[derive(Subcommand, Debug)]
pub enum ExemptionsCommands {
    List { #[arg(long)] profile_id: String },
    Get { #[arg(long)] profile_id: String, id: String },
    Create {
        #[arg(long)] profile_id: String,
        #[arg(long)] name: String,
        #[arg(long)] reason: String,
        #[arg(long)] rules: Option<String>,
    },
    Update {
        #[arg(long)] profile_id: String,
        id: String,
        #[arg(long)] name: Option<String>,
        #[arg(long)] reason: Option<String>,
    },
    Delete { #[arg(long)] profile_id: String, id: String },
}

pub async fn run(args: &VulnerabilitiesArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        VulnerabilitiesCommands::List(a) => {
            let params = ListVulnerabilitiesParams {
                severity_levels: a.severity.clone(),
                providers: a.provider.clone(),
                has_kev: if a.has_kev { Some(true) } else { None },
                has_exploit: if a.has_exploit { Some(true) } else { None },
                has_vendor_fix: if a.has_vendor_fix { Some(true) } else { None },
                integration_ids: a.integration_id.clone(),
                asset_ids: a.asset_id.clone(),
                regions: a.region.clone(),
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            let resp = list_vulnerabilities(&client, &params).await?;
            output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        VulnerabilitiesCommands::Exemptions(e) => match &e.command {
            ExemptionsCommands::List { profile_id } => {
                let resp = list_exemptions(&client, profile_id).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
            ExemptionsCommands::Get { profile_id, id } => {
                let resp = get_exemption(&client, profile_id, id).await?;
                output::render_json_value(&resp, config.output, config.query.as_deref())?;
            }
            ExemptionsCommands::Create { profile_id, name, reason, rules } => {
                let rules_val: Option<serde_json::Value> = rules.as_ref()
                    .map(|r| serde_json::from_str(r))
                    .transpose()
                    .map_err(|e| anyhow::anyhow!("Invalid JSON for --rules: {e}"))?;
                let body = serde_json::json!({
                    "name": name,
                    "reason": reason,
                    "rules": rules_val
                });
                let resp = create_exemption(&client, profile_id, body).await?;
                output::render_json_value(&resp, config.output, config.query.as_deref())?;
            }
            ExemptionsCommands::Update { profile_id, id, name, reason } => {
                let mut body = serde_json::Map::new();
                if let Some(n) = name { body.insert("name".to_string(), serde_json::json!(n)); }
                if let Some(r) = reason { body.insert("reason".to_string(), serde_json::json!(r)); }
                let resp = update_exemption(&client, profile_id, id, serde_json::Value::Object(body)).await?;
                output::render_json_value(&resp, config.output, config.query.as_deref())?;
            }
            ExemptionsCommands::Delete { profile_id, id } => {
                delete_exemption(&client, profile_id, id).await?;
                println!("Exemption '{id}' deleted.");
            }
        },
    }
    Ok(())
}
