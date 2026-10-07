use clap::{Args, Subcommand};

use crate::api::client::PlerionClient;
use crate::api::endpoints::custom_reports::list_custom_reports;
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct CustomReportsArgs {
    #[command(subcommand)]
    pub command: CustomReportsCommands,
}

#[derive(Subcommand, Debug)]
pub enum CustomReportsCommands {
    /// List custom dashboards
    List(ListCustomReportsArgs),
}

#[derive(Args, Debug)]
pub struct ListCustomReportsArgs {
    /// Dashboards per page (1 to 100)
    #[arg(long, default_value = "50", value_parser = clap::value_parser!(u32).range(1..=100))]
    pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)]
    pub all: bool,
}

pub async fn run(args: &CustomReportsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        CustomReportsCommands::List(a) => {
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let resp = list_custom_reports(&client, Some(100), cursor.as_deref()).await?;
                    let empty = resp.data.is_empty();
                    all_items.extend(resp.data);
                    // No hasNextPage here: meta.cursor goes null once the list is
                    // exhausted. A page ending on the last dashboard still carries
                    // a cursor, so an empty page also ends the loop.
                    cursor = resp.meta.cursor;
                    if cursor.is_none() || empty {
                        break;
                    }
                }
                output::render_list(
                    &all_items,
                    config.output,
                    config.query.as_deref(),
                    config.no_color,
                )?;
            } else {
                let resp = list_custom_reports(&client, Some(a.per_page), None).await?;
                output::render_list(
                    &resp.data,
                    config.output,
                    config.query.as_deref(),
                    config.no_color,
                )?;
            }
        }
    }
    Ok(())
}
