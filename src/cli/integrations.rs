use clap::{Args, Subcommand};
use crate::api::{
    client::PlerionClient,
    endpoints::integrations::{list_integrations, replace_user_defined_tags},
    models::integrations::{ReplaceUserDefinedTagsRequest, UserDefinedTag},
};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct IntegrationsArgs {
    #[command(subcommand)]
    pub command: IntegrationsCommands,
}

#[derive(Subcommand, Debug)]
pub enum IntegrationsCommands {
    List(ListIntegrationsArgs),
    /// Replace all user-defined tags on an integration.
    ///
    /// The tags you pass become the complete set: any user-defined tag you leave
    /// out is removed. Tags read from the cloud account are not changed, but a
    /// user-defined tag overrides a cloud tag with the same key.
    SetTags(SetTagsArgs),
}

#[derive(Args, Debug)]
pub struct SetTagsArgs {
    /// Integration ID
    pub id: String,
    /// A tag as KEY=VALUE; repeat for each tag (up to 50). Splits on the first `=`
    #[arg(long = "tag", value_name = "KEY=VALUE", value_parser = parse_tag)]
    pub tags: Vec<UserDefinedTag>,
    /// Remove every user-defined tag
    #[arg(long, conflicts_with = "tags")]
    pub clear: bool,
}

/// Parses KEY=VALUE, splitting on the first `=`. Key and value must be non-empty.
pub fn parse_tag(s: &str) -> Result<UserDefinedTag, String> {
    match s.split_once('=') {
        Some((k, v)) if !k.is_empty() && !v.is_empty() => Ok(UserDefinedTag {
            key: k.to_string(),
            value: v.to_string(),
        }),
        _ => Err(format!(
            "'{s}' is not KEY=VALUE with a non-empty key and value"
        )),
    }
}

/// Builds the replacement tag list, rejecting an empty or ambiguous request.
pub fn tags_to_send(tags: &[UserDefinedTag], clear: bool) -> anyhow::Result<Vec<UserDefinedTag>> {
    if clear {
        return Ok(Vec::new());
    }
    if tags.is_empty() {
        anyhow::bail!("No tags given. Pass one or more --tag KEY=VALUE, or --clear to remove every user-defined tag.");
    }
    for (i, t) in tags.iter().enumerate() {
        if tags[..i].iter().any(|p| p.key == t.key) {
            anyhow::bail!(
                "Tag key '{}' is given more than once. Keys must be unique.",
                t.key
            );
        }
    }
    Ok(tags.to_vec())
}

#[derive(Args, Debug)]
pub struct ListIntegrationsArgs {
    #[arg(long, default_value = "50")] pub per_page: u32,
    #[arg(long)] pub include_total: bool,
    /// Fetch all pages automatically
    #[arg(long)] pub all: bool,
}

pub async fn run(args: &IntegrationsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        IntegrationsCommands::List(a) => {
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let resp = list_integrations(&client, Some(1000), cursor.as_deref(), a.include_total).await?;
                    let has_next = resp.meta.has_next_page.unwrap_or(false);
                    cursor = resp.meta.cursor.clone();
                    all_items.extend(resp.data);
                    if !has_next { break; }
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_integrations(&client, Some(a.per_page), None, a.include_total).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
        IntegrationsCommands::SetTags(a) => {
            let body = ReplaceUserDefinedTagsRequest {
                tags: tags_to_send(&a.tags, a.clear)?,
            };
            let resp = replace_user_defined_tags(&client, &a.id, &body).await?;
            output::render_list(
                &resp.data.tags,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
    }
    Ok(())
}
