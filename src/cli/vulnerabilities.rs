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
    List(Box<ListVulnArgs>),
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
    #[arg(long)] pub vulnerability_id: Option<String>,
    #[arg(long)] pub asset_group_id: Option<String>,
    #[arg(long)] pub environment_id: Option<String>,
    #[arg(long)] pub package_name: Option<String>,
    #[arg(long)] pub is_exempted: Option<bool>,
    #[arg(long)] pub is_exploitable: bool,
    #[arg(long)] pub start: Option<String>,
    #[arg(long)] pub end: Option<String>,
    #[arg(long)] pub execution_id: Option<String>,
    #[arg(long)] pub target_name: Option<String>,
    #[arg(long)] pub target_type: Option<String>,
    #[arg(long)] pub target_class: Option<String>,
}

#[derive(Args, Debug)]
pub struct ExemptionsArgs {
    #[command(subcommand)]
    pub command: ExemptionsCommands,
}

#[derive(Subcommand, Debug)]
pub enum ExemptionsCommands {
    List {
        #[arg(long)] profile_id: String,
        /// Number of results per page (max 1000)
        #[arg(long, default_value = "100")]
        per_page: u32,
        /// Fetch all pages automatically
        #[arg(long)]
        all: bool,
    },
    Get {
        #[arg(long)] profile_id: String,
        /// Exemption ID
        #[arg(long)] id: String,
    },
    Create {
        #[arg(long)] profile_id: String,
        #[arg(long)] name: String,
        #[arg(long)] reason: String,
        #[arg(long)] rules: Option<String>,
        #[arg(long)] audit_note: String,
    },
    Update {
        #[arg(long)] profile_id: String,
        /// Exemption ID
        #[arg(long)] id: String,
        #[arg(long)] name: Option<String>,
        #[arg(long)] reason: Option<String>,
        #[arg(long)] audit_note: Option<String>,
        #[arg(long)] conditions: Option<String>,
    },
    Delete {
        #[arg(long)] profile_id: String,
        /// Exemption ID
        #[arg(long)] id: String,
    },
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
                vulnerability_ids: a.vulnerability_id.clone(),
                asset_group_ids: a.asset_group_id.clone(),
                environment_ids: a.environment_id.clone(),
                package_name: a.package_name.clone(),
                is_exempted: a.is_exempted,
                is_exploitable: if a.is_exploitable { Some(true) } else { None },
                first_observed_at_start: a.start.clone(),
                first_observed_at_end: a.end.clone(),
                execution_ids: a.execution_id.clone(),
                target_name: a.target_name.clone(),
                target_type: a.target_type.clone(),
                target_class: a.target_class.clone(),
                ..Default::default()
            };
            if a.all {
                let mut all_items = Vec::new();
                let mut page = 1u32;
                loop {
                    let p = ListVulnerabilitiesParams {
                        page: Some(page),
                        per_page: Some(1000),
                        severity_levels: params.severity_levels.clone(),
                        providers: params.providers.clone(),
                        has_kev: params.has_kev,
                        has_exploit: params.has_exploit,
                        has_vendor_fix: params.has_vendor_fix,
                        integration_ids: params.integration_ids.clone(),
                        asset_ids: params.asset_ids.clone(),
                        regions: params.regions.clone(),
                        sort_by: params.sort_by.clone(),
                        sort_order: params.sort_order.clone(),
                        vulnerability_ids: params.vulnerability_ids.clone(),
                        asset_group_ids: params.asset_group_ids.clone(),
                        environment_ids: params.environment_ids.clone(),
                        package_name: params.package_name.clone(),
                        is_exempted: params.is_exempted,
                        is_exploitable: params.is_exploitable,
                        first_observed_at_start: params.first_observed_at_start.clone(),
                        first_observed_at_end: params.first_observed_at_end.clone(),
                        execution_ids: params.execution_ids.clone(),
                        target_name: params.target_name.clone(),
                        target_type: params.target_type.clone(),
                        target_class: params.target_class.clone(),
                    };
                    let resp = list_vulnerabilities(&client, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    all_items.extend(resp.data);
                    if !has_next { break; }
                    page += 1;
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_vulnerabilities(&client, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
        VulnerabilitiesCommands::Exemptions(e) => match &e.command {
            ExemptionsCommands::List { profile_id, per_page, all } => {
                if *all {
                    let mut all_items = Vec::new();
                    let mut cursor: Option<String> = None;
                    loop {
                        let resp = list_exemptions(&client, profile_id, Some(1000), cursor.as_deref()).await?;
                        let has_next = resp.meta.has_next.unwrap_or(false);
                        cursor = resp.meta.next_cursor.clone();
                        all_items.extend(resp.data);
                        if !has_next { break; }
                    }
                    output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
                } else {
                    let resp = list_exemptions(&client, profile_id, Some(*per_page), None).await?;
                    output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
                }
            }
            ExemptionsCommands::Get { profile_id, id } => {
                let resp = get_exemption(&client, profile_id, id).await?;
                output::render_json_value(&resp, config.output, config.query.as_deref())?;
            }
            ExemptionsCommands::Create { profile_id, name, reason, rules, audit_note } => {
                let rules_val: Option<serde_json::Value> = rules.as_ref()
                    .map(|r| serde_json::from_str(r))
                    .transpose()
                    .map_err(|e| anyhow::anyhow!("Invalid JSON for --rules: {e}"))?;
                let body = serde_json::json!({
                    "name": name,
                    "reason": reason,
                    "rules": rules_val,
                    "auditNote": audit_note
                });
                let resp = create_exemption(&client, profile_id, body).await?;
                output::render_json_value(&resp, config.output, config.query.as_deref())?;
            }
            ExemptionsCommands::Update { profile_id, id, name, reason, audit_note, conditions } => {
                let mut body = serde_json::Map::new();
                if let Some(n) = name { body.insert("name".to_string(), serde_json::json!(n)); }
                if let Some(r) = reason { body.insert("reason".to_string(), serde_json::json!(r)); }
                if let Some(a) = audit_note { body.insert("auditNote".to_string(), serde_json::json!(a)); }
                if let Some(c) = conditions {
                    let cond_val: serde_json::Value = serde_json::from_str(c)
                        .map_err(|e| anyhow::anyhow!("Invalid JSON for --conditions: {e}"))?;
                    body.insert("conditions".to_string(), cond_val);
                }
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
