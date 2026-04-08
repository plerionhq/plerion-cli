use clap::{Args, Subcommand};
use crate::config::credentials;
use crate::config::VALID_REGIONS;

#[derive(Args, Debug)]
pub struct ConfigureArgs {
    #[command(subcommand)]
    pub command: Option<ConfigureCommands>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigureCommands {
    /// List all configured profiles
    List,
}

pub async fn run(args: &ConfigureArgs) -> anyhow::Result<()> {
    match &args.command {
        Some(ConfigureCommands::List) => list_profiles(),
        None => interactive_configure(),
    }
}

fn list_profiles() -> anyhow::Result<()> {
    let profiles = credentials::list_profiles();
    if profiles.is_empty() {
        println!("No profiles configured. Run `plerion configure` to set one up.");
    } else {
        println!("Configured profiles:");
        for p in profiles {
            println!("  {p}");
        }
    }
    Ok(())
}

fn interactive_configure() -> anyhow::Result<()> {

    println!("Plerion CLI Configuration");
    println!("─────────────────────────");

    let profile = prompt("Profile name [default]: ")?.trim().to_string();
    let profile = if profile.is_empty() { "default".to_string() } else { profile };

    let api_key = prompt("Plerion API Key: ")?.trim().to_string();
    if api_key.is_empty() {
        anyhow::bail!("API key cannot be empty.");
    }

    let region_prompt = format!(
        "Region ({}) [au]: ",
        VALID_REGIONS.join(", ")
    );
    let region = prompt(&region_prompt)?.trim().to_string();
    let region = if region.is_empty() { "au".to_string() } else { region };
    if !VALID_REGIONS.contains(&region.as_str()) {
        anyhow::bail!("Invalid region '{region}'. Valid: {}", VALID_REGIONS.join(", "));
    }

    let output = prompt("Default output format (table/json/yaml/text) [table]: ")?
        .trim()
        .to_string();
    let output = if output.is_empty() { "table".to_string() } else { output };

    credentials::write_profile(&profile, &api_key, &region, &output)?;

    println!("\n✓ Profile '{profile}' saved.");
    println!("  Credentials: {}", credentials::credentials_path().display());
    println!("  Config:      {}", credentials::config_path().display());

    Ok(())
}

fn prompt(msg: &str) -> anyhow::Result<String> {
    use std::io::{self, Write};
    print!("{msg}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim_end_matches('\n').to_string())
}
