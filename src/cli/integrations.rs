use clap::{Args, Subcommand};
use crate::api::{client::PlerionClient, endpoints::integrations::list_integrations};
use crate::config::Config;
use crate::onboard;
use crate::output;

#[derive(Args, Debug)]
pub struct IntegrationsArgs {
    #[command(subcommand)]
    pub command: IntegrationsCommands,
}

#[derive(Subcommand, Debug)]
pub enum IntegrationsCommands {
    List(ListIntegrationsArgs),
    /// Onboard a cloud account as a new integration
    Add(AddArgs),
}

#[derive(Args, Debug)]
pub struct ListIntegrationsArgs {
    #[arg(long, default_value = "50")] pub per_page: u32,
    #[arg(long)] pub include_total: bool,
    /// Fetch all pages automatically
    #[arg(long)] pub all: bool,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[command(subcommand)]
    pub provider: AddProvider,
}

#[derive(Subcommand, Debug)]
pub enum AddProvider {
    /// Onboard an AWS account: preflight checks, CloudFormation deploy,
    /// automatic registration with Plerion
    Aws(AddAwsArgs),
}

#[derive(Args, Debug)]
pub struct AddAwsArgs {
    /// AWS CLI profile for the target account (default: AWS default credential chain)
    #[arg(long)]
    pub aws_profile: Option<String>,

    /// AWS region to deploy the stack in (IAM is global; any enabled region works)
    #[arg(long)]
    pub aws_region: Option<String>,

    /// Abort unless the resolved AWS account matches (recommended with --yes)
    #[arg(long)]
    pub expect_account_id: Option<String>,

    /// Skip interactive confirmation
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// CloudFormation stack name (must start with "Plerion-" unless --no-auto-update)
    #[arg(long, default_value = onboard::DEFAULT_STACK_NAME)]
    pub stack_name: String,

    /// Do not create the Plerion auto-update role
    #[arg(long)]
    pub no_auto_update: bool,

    /// KMS key access mode for workload scanning
    #[arg(long, default_value = "ALL_KEYS", value_parser = ["ALL_KEYS", "SELECTED_KEYS"])]
    pub kms_key_access_mode: String,

    /// Plerion-managed CWPP service account ID (only needed when it cannot be
    /// resolved automatically)
    #[arg(long)]
    pub service_account_id: Option<String>,

    /// Proceed even if the account already has a Plerion integration or roles
    #[arg(long)]
    pub allow_existing: bool,

    /// Treat IAM simulation denials as fatal (default: advisory — the
    /// simulator has known false negatives with AWS Identity Center roles)
    #[arg(long)]
    pub strict_preflight: bool,

    /// Run every preflight check, deploy nothing
    #[arg(long)]
    pub validate_only: bool,

    /// Print the execution plan without any network calls
    #[arg(long)]
    pub dry_run: bool,

    /// Seconds to wait for stack completion
    #[arg(long, default_value = "1800")]
    pub wait_timeout: u64,

    #[arg(long, default_value = onboard::PLERION_ACCOUNT_ID_DEFAULT, hide = true)]
    pub plerion_account_id: String,
}

pub async fn run(args: &IntegrationsArgs, config: &Config) -> anyhow::Result<()> {
    match &args.command {
        IntegrationsCommands::List(a) => {
            let client = PlerionClient::new(config)?;
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
            Ok(())
        }
        IntegrationsCommands::Add(add) => match &add.provider {
            AddProvider::Aws(a) => run_add_aws(a, config).await,
        },
    }
}

/// The host the deployed stack reports back to. Derived from the tenant
/// region, or from --endpoint-url when overridden (dev tenants).
fn plerion_url(config: &Config) -> String {
    match &config.endpoint_url {
        Some(url) => url
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/')
            .to_string(),
        None => format!("{}.api.plerion.com", config.region),
    }
}

async fn run_add_aws(args: &AddAwsArgs, config: &Config) -> anyhow::Result<()> {
    let opts = onboard::OnboardOptions {
        stack_name: args.stack_name.clone(),
        auto_update: !args.no_auto_update,
        kms_key_access_mode: args.kms_key_access_mode.clone(),
        service_account_id: args.service_account_id.clone(),
        expect_account_id: args.expect_account_id.clone(),
        allow_existing: args.allow_existing,
        strict_preflight: args.strict_preflight,
        validate_only: args.validate_only,
        yes: args.yes,
        wait_timeout_secs: args.wait_timeout,
        poll_interval_secs: 15,
        plerion_account_id: args.plerion_account_id.clone(),
        plerion_url: plerion_url(config),
        tenant_label: config
            .endpoint_url
            .clone()
            .unwrap_or_else(|| config.region.clone()),
    };

    if args.dry_run {
        print!("{}", onboard::plan::render(&opts));
        return Ok(());
    }

    let client = PlerionClient::new(config)?;
    let aws = onboard::sdk::SdkAws::new(args.aws_profile.as_deref(), args.aws_region.as_deref()).await;
    let mut ui = onboard::ui::TtyUi;

    match onboard::run(&client, &aws, &mut ui, &opts).await? {
        onboard::OnboardOutcome::Completed(result) | onboard::OnboardOutcome::Validated(result) => {
            output::render_json_value(&result, config.output, config.query.as_deref())?;
        }
    }
    Ok(())
}
