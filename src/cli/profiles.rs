use clap::{Args, Subcommand};

use crate::api::client::PlerionClient;
use crate::api::endpoints::profiles::{
    get_detection_exemptions, list_profiles, replace_detection_exemptions,
};
use crate::api::models::profiles::{DetectionExemptionSet, ReplaceDetectionExemptionsRequest};
use crate::config::Config;
use crate::output::{self, OutputFormat};

#[derive(Args, Debug)]
pub struct ProfilesArgs {
    #[command(subcommand)]
    pub command: ProfilesCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProfilesCommands {
    /// List the tenant's profiles and the integrations each applies to
    List,
    /// Read or replace the exemptions on one detection in a profile
    DetectionExemptions(DetectionExemptionsArgs),
}

#[derive(Args, Debug)]
pub struct DetectionExemptionsArgs {
    #[command(subcommand)]
    pub command: DetectionExemptionsCommands,
}

#[derive(Subcommand, Debug)]
pub enum DetectionExemptionsCommands {
    /// Get a detection's exemptions, the types it accepts and its current version
    Get(DetectionRef),
    /// Replace a detection's whole exemption set.
    ///
    /// The set you send becomes the complete set: entries you leave out are
    /// removed. Read the set with `get --output json`, edit it, and send all of
    /// it back. Pass the `version` from that read as --if-match so the write is
    /// refused (HTTP 412) if the detection changed in the meantime; without it
    /// the write overwrites any concurrent change.
    Replace(ReplaceDetectionExemptionsArgs),
}

#[derive(Args, Debug)]
pub struct DetectionRef {
    /// Profile ID, or `default` for the tenant's default profile
    #[arg(long)]
    pub profile_id: String,
    /// Detection ID, e.g. PLERION-AWS-16
    #[arg(long)]
    pub detection_id: String,
}

#[derive(Args, Debug)]
pub struct ReplaceDetectionExemptionsArgs {
    #[command(flatten)]
    pub detection: DetectionRef,
    /// The complete exemption set as JSON: an array, or an object with an
    /// `exemptions` array (such as the output of `get --output json`)
    #[arg(long, value_name = "JSON", conflicts_with = "file")]
    pub exemptions: Option<String>,
    /// Read the exemption set JSON from a file, or `-` for stdin
    #[arg(long, value_name = "PATH")]
    pub file: Option<String>,
    /// Remove every exemption on the detection
    #[arg(long, conflicts_with_all = ["exemptions", "file"])]
    pub clear: bool,
    /// The `version` from the last read; omit only when it was null
    #[arg(long, value_name = "VERSION")]
    pub if_match: Option<String>,
}

/// Accepts a JSON array of exemptions, or an object holding one under `exemptions`.
pub fn parse_exemptions(text: &str) -> anyhow::Result<Vec<serde_json::Value>> {
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| anyhow::anyhow!("Invalid JSON for the exemption set: {e}"))?;
    let list = match value {
        serde_json::Value::Array(items) => items,
        serde_json::Value::Object(mut obj) => match obj.remove("exemptions") {
            Some(serde_json::Value::Array(items)) => items,
            _ => anyhow::bail!("The exemption set JSON object has no `exemptions` array."),
        },
        _ => anyhow::bail!(
            "The exemption set must be a JSON array, or an object with an `exemptions` array."
        ),
    };
    if let Some(i) = list.iter().position(|e| !e.is_object()) {
        anyhow::bail!("Exemption {} is not a JSON object.", i + 1);
    }
    Ok(list)
}

fn read_input(a: &ReplaceDetectionExemptionsArgs) -> anyhow::Result<Vec<serde_json::Value>> {
    if a.clear {
        return Ok(Vec::new());
    }
    let text = match (&a.exemptions, &a.file) {
        (Some(json), _) => json.clone(),
        (None, Some(path)) if path == "-" => std::io::read_to_string(std::io::stdin())?,
        (None, Some(path)) => std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Could not read {path}: {e}"))?,
        (None, None) => anyhow::bail!(
            "Nothing to replace. Pass the exemption set with --exemptions or --file, \
             or --clear to remove every exemption."
        ),
    };
    let list = parse_exemptions(&text)?;
    if list.is_empty() {
        anyhow::bail!(
            "The exemption set is empty, which would remove every exemption. \
             Pass --clear if that is what you want."
        );
    }
    Ok(list)
}

fn render_set(set: &DetectionExemptionSet, config: &Config) -> anyhow::Result<()> {
    let query = config.query.as_deref();
    output::render(set, config.output, query, config.no_color)?;
    let tabular = matches!(config.output, OutputFormat::Table | OutputFormat::Text);
    if query.is_none() && tabular && !set.exemptions.is_empty() {
        println!();
        output::render_list(&set.exemptions, config.output, None, config.no_color)?;
    }
    Ok(())
}

pub async fn run(args: &ProfilesArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        ProfilesCommands::List => {
            let resp = list_profiles(&client).await?;
            output::render_list(
                &resp.data,
                config.output,
                config.query.as_deref(),
                config.no_color,
            )?;
        }
        ProfilesCommands::DetectionExemptions(d) => match &d.command {
            DetectionExemptionsCommands::Get(r) => {
                let resp =
                    get_detection_exemptions(&client, &r.profile_id, &r.detection_id).await?;
                render_set(&resp.data, config)?;
            }
            DetectionExemptionsCommands::Replace(a) => {
                let body = ReplaceDetectionExemptionsRequest {
                    exemptions: read_input(a)?,
                };
                let resp = replace_detection_exemptions(
                    &client,
                    &a.detection.profile_id,
                    &a.detection.detection_id,
                    a.if_match.as_deref(),
                    &body,
                )
                .await?;
                render_set(&resp.data, config)?;
            }
        },
    }
    Ok(())
}
