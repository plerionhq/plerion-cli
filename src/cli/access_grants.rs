use clap::{Args, Subcommand};
use crate::api::{
    client::PlerionClient,
    endpoints::access_grants::{
        get_access_grant, get_access_grant_stats, list_access_grant_external_principals,
        list_access_grants, update_access_grant, ListAccessGrantsParams,
    },
    models::access_grants::UpdateAccessGrantRequest,
};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct AccessGrantsArgs {
    #[command(subcommand)]
    pub command: AccessGrantsCommands,
}

#[derive(Subcommand, Debug)]
pub enum AccessGrantsCommands {
    /// List access grants
    List(ListAccessGrantsArgs),
    /// Get a single access grant by ID
    Get { id: String },
    /// Access grant counts for the tenant
    Stats,
    /// External principals holding access, with their grant counts
    ExternalPrincipals,
    /// Record a review decision on an access grant
    Update(UpdateAccessGrantArgs),
}

#[derive(Args, Debug)]
pub struct ListAccessGrantsArgs {
    /// Grant IDs (comma-separated UUIDs)
    #[arg(long)] pub ids: Option<String>,
    /// Whether the principal is outside the organization and not blocked by an RCP
    #[arg(long, value_parser = ["external", "internal"])]
    pub grant_origin: Option<String>,
    /// Trust statuses (comma-separated: trusted, untrusted)
    #[arg(long)] pub trust_statuses: Option<String>,
    /// Trust boundaries (comma-separated: cross-org, federated, public, same-account, same-org, aws-service)
    #[arg(long)] pub grant_scopes: Option<String>,
    /// Sharing mechanisms (comma-separated: resource_policy, trust_policy, ram_share, attribute_share)
    #[arg(long)] pub mechanisms: Option<String>,
    /// Principal types (comma-separated)
    #[arg(long)] pub principal_types: Option<String>,
    /// Resource types (comma-separated, e.g. AWS::S3::Bucket)
    #[arg(long)] pub resource_types: Option<String>,
    /// Integration IDs (comma-separated UUIDs)
    #[arg(long)] pub integration_ids: Option<String>,
    /// AWS accounts owning the granting resource (comma-separated 12-digit IDs)
    #[arg(long)] pub aws_account_ids: Option<String>,
    /// AWS accounts of the principal (comma-separated 12-digit IDs)
    #[arg(long)] pub principal_account_ids: Option<String>,
    /// Substring match on the resource name or asset ID
    #[arg(long)] pub asset_name: Option<String>,
    /// Substring match on the principal
    #[arg(long)] pub principal: Option<String>,
    /// Substring match across principal, resource name and asset ID
    #[arg(long)] pub search: Option<String>,
    /// Recorded review decisions (comma-separated: keep, remove, review_later)
    #[arg(long)] pub review_decisions: Option<String>,
    /// Substring match on the recorded owner
    #[arg(long)] pub grant_owner: Option<String>,
    /// Only grants due for review by this ISO 8601 date-time
    #[arg(long)] pub next_review_at_end: Option<String>,
    #[arg(long, default_value = "50")] pub per_page: u32,
    /// Fetch all pages automatically
    #[arg(long)] pub all: bool,
}

#[derive(Args, Debug)]
pub struct UpdateAccessGrantArgs {
    pub id: String,
    /// Review decision to record
    #[arg(long, value_parser = ["keep", "remove", "review_later"])]
    pub review_decision: Option<String>,
    /// Free-text review comment
    #[arg(long)] pub review_comment: Option<String>,
    /// Owner accountable for the grant
    #[arg(long)] pub grant_owner: Option<String>,
    /// When the grant is next due for review (ISO 8601 date-time)
    #[arg(long)] pub next_review_at: Option<String>,
    /// Clear the recorded review decision
    #[arg(long, conflicts_with = "review_decision")] pub clear_review_decision: bool,
    /// Clear the review comment
    #[arg(long, conflicts_with = "review_comment")] pub clear_review_comment: bool,
    /// Clear the grant owner
    #[arg(long, conflicts_with = "grant_owner")] pub clear_grant_owner: bool,
    /// Clear the next review date
    #[arg(long, conflicts_with = "next_review_at")] pub clear_next_review_at: bool,
}

/// A set value, an explicit null to clear, or absent to leave untouched.
fn review_field(value: Option<&String>, clear: bool) -> Option<serde_json::Value> {
    match (value, clear) {
        (Some(v), _) => Some(serde_json::Value::String(v.clone())),
        (None, true) => Some(serde_json::Value::Null),
        (None, false) => None,
    }
}

/// Grant IDs are UUIDs. Checking here stops a word like `stats` being sent as a
/// path segment, where it resolves to a sibling route and fails on deserialization
/// instead of returning a legible error.
fn ensure_grant_id(id: &str) -> anyhow::Result<()> {
    let looks_like_uuid = id.len() == 36
        && id.as_bytes().iter().enumerate().all(|(i, b)| match i {
            8 | 13 | 18 | 23 => *b == b'-',
            _ => b.is_ascii_hexdigit(),
        });
    if looks_like_uuid {
        return Ok(());
    }
    anyhow::bail!(
        "'{id}' is not a valid grant ID. Grant IDs are UUIDs; run \
         `plerion access-grants list` to find one."
    )
}

fn list_params(a: &ListAccessGrantsArgs) -> ListAccessGrantsParams {
    ListAccessGrantsParams {
        ids: a.ids.clone(),
        grant_origin: a.grant_origin.clone(),
        trust_statuses: a.trust_statuses.clone(),
        grant_scopes: a.grant_scopes.clone(),
        mechanisms: a.mechanisms.clone(),
        principal_types: a.principal_types.clone(),
        resource_types: a.resource_types.clone(),
        integration_ids: a.integration_ids.clone(),
        aws_account_ids: a.aws_account_ids.clone(),
        principal_account_ids: a.principal_account_ids.clone(),
        asset_name: a.asset_name.clone(),
        principal: a.principal.clone(),
        search: a.search.clone(),
        review_decisions: a.review_decisions.clone(),
        grant_owner: a.grant_owner.clone(),
        next_review_at_end: a.next_review_at_end.clone(),
        cursor: None,
        per_page: Some(a.per_page),
    }
}

pub async fn run(args: &AccessGrantsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        AccessGrantsCommands::List(a) => {
            let params = list_params(a);
            if a.all {
                let mut all_items = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let p = ListAccessGrantsParams {
                        cursor: cursor.clone(),
                        per_page: Some(1000),
                        ..params.clone()
                    };
                    let resp = list_access_grants(&client, &p).await?;
                    all_items.extend(resp.data);
                    // meta.cursor is null on the last page; this endpoint has no
                    // hasNextPage flag, so the cursor is the only signal.
                    cursor = resp.meta.cursor.clone();
                    if cursor.is_none() { break; }
                }
                output::render_list(&all_items, config.output, config.query.as_deref(), config.no_color)?;
            } else {
                let resp = list_access_grants(&client, &params).await?;
                output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
            }
        }
        AccessGrantsCommands::Get { id } => {
            ensure_grant_id(id)?;
            let resp = get_access_grant(&client, id).await?;
            output::render(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        AccessGrantsCommands::Stats => {
            let resp = get_access_grant_stats(&client).await?;
            output::render(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        AccessGrantsCommands::ExternalPrincipals => {
            let resp = list_access_grant_external_principals(&client).await?;
            output::render_list(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
        AccessGrantsCommands::Update(a) => {
            ensure_grant_id(&a.id)?;
            let body = UpdateAccessGrantRequest {
                review_decision: review_field(a.review_decision.as_ref(), a.clear_review_decision),
                review_comment: review_field(a.review_comment.as_ref(), a.clear_review_comment),
                grant_owner: review_field(a.grant_owner.as_ref(), a.clear_grant_owner),
                next_review_at: review_field(a.next_review_at.as_ref(), a.clear_next_review_at),
            };
            if body.is_empty() {
                anyhow::bail!(
                    "Nothing to update. Pass at least one of --review-decision, --review-comment, \
                     --grant-owner, --next-review-at, or a --clear-* flag."
                );
            }
            let resp = update_access_grant(&client, &a.id, body).await?;
            output::render(&resp.data, config.output, config.query.as_deref(), config.no_color)?;
        }
    }
    Ok(())
}
