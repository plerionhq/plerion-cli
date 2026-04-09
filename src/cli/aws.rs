use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::aws::*};
use crate::config::Config;
use crate::output;

#[derive(Args, Debug)]
pub struct AwsArgs {
    #[command(subcommand)]
    pub command: AwsCommands,
}

#[derive(Subcommand, Debug)]
pub enum AwsCommands {
    /// Get the tenant external ID for cross-account role trust
    GetExternalId,
    /// Get the CloudFormation template for AWS integration
    GetCloudformationTemplate {
        /// Template type (e.g. AWSAccount)
        #[arg(long, name = "type")]
        template_type: Option<String>,
    },
    /// Generate a temporary integration token
    GenerateToken { #[arg(long)] integration_id: String },
}

pub async fn run(args: &AwsArgs, config: &Config) -> anyhow::Result<()> {
    let client = PlerionClient::new(config)?;
    match &args.command {
        AwsCommands::GetExternalId => {
            let resp = get_external_id(&client).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
        AwsCommands::GetCloudformationTemplate { template_type } => {
            let resp = get_cloudformation_template(&client, template_type.as_deref()).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
        AwsCommands::GenerateToken { integration_id } => {
            let resp = generate_token(&client, integration_id).await?;
            output::render_json_value(&resp, config.output, config.query.as_deref())?;
        }
    }
    Ok(())
}
