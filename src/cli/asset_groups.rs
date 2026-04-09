use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::asset_groups::*};
use crate::api::models::asset_groups::{CreateAssetGroupRequest, UpdateAssetGroupRequest};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct AssetGroupsArgs {
    #[command(subcommand)]
    pub command: AssetGroupsCommands,
}

#[derive(Subcommand, Debug)]
pub enum AssetGroupsCommands {
    /// List asset groups
    List(ListAssetGroupsArgs),
    /// Get an asset group by ID
    Get {
        /// Asset group ID
        #[arg(long)]
        id: String,
    },
    /// Create a new asset group
    Create(CreateAssetGroupArgs),
    /// Update an asset group
    Update(UpdateAssetGroupArgs),
    /// Delete an asset group
    Delete {
        /// Asset group ID
        #[arg(long)]
        id: String,
    },
}

#[derive(Args, Debug)]
pub struct ListAssetGroupsArgs {
    #[arg(long)] pub name: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
}

#[derive(Args, Debug)]
pub struct CreateAssetGroupArgs {
    #[arg(long)] pub name: String,
    #[arg(long)] pub rules: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateAssetGroupArgs {
    /// Asset group ID
    #[arg(long)]
    pub id: String,
    #[arg(long)] pub name: Option<String>,
    #[arg(long)] pub rules: Option<String>,
}

pub async fn run(args: &AssetGroupsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        AssetGroupsCommands::List(a) => {
            let resp = list_asset_groups(&client, a.name.as_deref(), Some(a.per_page), None, false).await?;
            output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        AssetGroupsCommands::Get { id } => {
            let resp = get_asset_group(&client, id).await?;
            output::render(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        AssetGroupsCommands::Create(a) => {
            let rules = a.rules.as_ref()
                .map(|r| serde_json::from_str(r))
                .transpose()
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --rules: {e}"))?;
            let resp = create_asset_group(&client, CreateAssetGroupRequest {
                name: a.name.clone(),
                rules,
            }).await?;
            output::render(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        AssetGroupsCommands::Update(a) => {
            let rules = a.rules.as_ref()
                .map(|r| serde_json::from_str(r))
                .transpose()
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --rules: {e}"))?;
            let resp = update_asset_group(&client, &a.id, UpdateAssetGroupRequest {
                name: a.name.clone(),
                rules,
            }).await?;
            output::render(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        AssetGroupsCommands::Delete { id } => {
            delete_asset_group(&client, id).await?;
            println!("Asset group '{id}' deleted.");
        }
    }
    Ok(())
}
