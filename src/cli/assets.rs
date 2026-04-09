use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::assets::{get_asset, get_asset_sbom, list_assets, ListAssetsParams}};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct AssetsArgs {
    #[command(subcommand)]
    pub command: AssetsCommands,
}

#[derive(Subcommand, Debug)]
pub enum AssetsCommands {
    /// List assets
    List(ListAssetsArgs),
    /// Get a specific asset by ID
    Get {
        /// Asset ID (e.g. prn:assets:...)
        #[arg(long)]
        asset_id: String,
    },
    /// Get SBOM for a specific asset
    GetSbom {
        /// Asset ID (e.g. prn:assets:...)
        #[arg(long)]
        asset_id: String,
    },
}

#[derive(Args, Debug)]
pub struct ListAssetsArgs {
    #[arg(long)] pub provider: Option<String>,
    #[arg(long)] pub region: Option<String>,
    #[arg(long)] pub resource_type: Option<String>,
    #[arg(long)] pub service: Option<String>,
    #[arg(long)] pub integration_id: Option<String>,
    #[arg(long)] pub asset_group_id: Option<String>,
    #[arg(long)] pub environment_id: Option<String>,
    #[arg(long)] pub severity: Option<String>,
    #[arg(long)] pub is_publicly_exposed: bool,
    #[arg(long)] pub is_vulnerable: bool,
    #[arg(long)] pub has_kev: bool,
    #[arg(long)] pub has_exploit: bool,
    #[arg(long)] pub has_admin_privileges: bool,
    #[arg(long)] pub risk_score_gte: Option<f64>,
    #[arg(long)] pub query_string: Option<String>,
    #[arg(long)] pub operational_state: Option<String>,
    #[arg(long)] pub sort_by: Option<String>,
    #[arg(long)] pub sort_order: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
    #[arg(long)] pub all: bool,
}

pub async fn run(args: &AssetsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        AssetsCommands::Get { asset_id } => {
            let resp = get_asset(&client, asset_id).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
        AssetsCommands::GetSbom { asset_id } => {
            let resp = get_asset_sbom(&client, asset_id).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
        AssetsCommands::List(a) => {
            let params = ListAssetsParams {
                providers: a.provider.clone(),
                regions: a.region.clone(),
                resource_types: a.resource_type.clone(),
                services: a.service.clone(),
                integration_ids: a.integration_id.clone(),
                asset_group_ids: a.asset_group_id.clone(),
                environment_ids: a.environment_id.clone(),
                severity_levels: a.severity.clone(),
                is_publicly_exposed: if a.is_publicly_exposed { Some(true) } else { None },
                is_vulnerable: if a.is_vulnerable { Some(true) } else { None },
                has_kev: if a.has_kev { Some(true) } else { None },
                has_exploit: if a.has_exploit { Some(true) } else { None },
                has_admin_privileges: if a.has_admin_privileges { Some(true) } else { None },
                risk_score_gte: a.risk_score_gte,
                query: a.query_string.clone(),
                operational_states: a.operational_state.clone(),
                sort_by: a.sort_by.clone(),
                sort_order: a.sort_order.clone(),
                per_page: Some(a.per_page),
                ..Default::default()
            };
            if a.all {
                let mut all_assets = Vec::new();
                let mut page = 1u32;
                loop {
                    let p = ListAssetsParams { page: Some(page), per_page: Some(1000), ..ListAssetsParams {
                        providers: params.providers.clone(),
                        regions: params.regions.clone(),
                        resource_types: params.resource_types.clone(),
                        services: params.services.clone(),
                        integration_ids: params.integration_ids.clone(),
                        asset_group_ids: params.asset_group_ids.clone(),
                        environment_ids: params.environment_ids.clone(),
                        severity_levels: params.severity_levels.clone(),
                        is_publicly_exposed: params.is_publicly_exposed,
                        is_vulnerable: params.is_vulnerable,
                        has_kev: params.has_kev,
                        has_exploit: params.has_exploit,
                        has_admin_privileges: params.has_admin_privileges,
                        risk_score_gte: params.risk_score_gte,
                        query: params.query.clone(),
                        operational_states: params.operational_states.clone(),
                        sort_by: params.sort_by.clone(),
                        sort_order: params.sort_order.clone(),
                        ..Default::default()
                    }};
                    let resp = list_assets(&client, &p).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    all_assets.extend(resp.data);
                    if !has_next { break; }
                    page += 1;
                }
                output::render_list(&all_assets, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_assets(&client, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
    }
    Ok(())
}
