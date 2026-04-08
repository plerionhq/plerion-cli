pub mod credentials;

use crate::error::PlerionError;
use std::path::PathBuf;

pub const VALID_REGIONS: &[&str] = &["au", "sg1", "in1", "us1"];

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub region: String,
    pub profile: String,
    pub output: crate::output::OutputFormat,
    pub no_color: bool,
    pub query: Option<String>,
    /// Custom base URL (overrides region-derived URL)
    pub endpoint_url: Option<String>,
}

impl Config {
    /// Load config with precedence:
    /// 1. CLI flags (passed in via overrides)
    /// 2. Environment variables
    /// 3. Named profile from credentials/config files
    /// 4. [default] profile
    pub fn load(overrides: ConfigOverrides) -> Result<Self, PlerionError> {
        // Determine profile name
        let profile = overrides
            .profile
            .or_else(|| std::env::var("PLERION_PROFILE").ok())
            .unwrap_or_else(|| "default".to_string());

        // Load from files
        let file_config = credentials::load_profile(&profile)?;

        // Resolve api_key
        let api_key = overrides
            .api_key
            .or_else(|| std::env::var("PLERION_API_KEY").ok())
            .or(file_config.api_key)
            .ok_or(PlerionError::MissingApiKey)?;

        // Resolve region (only validated when no endpoint_url override)
        let region = overrides
            .region
            .or_else(|| std::env::var("PLERION_REGION").ok())
            .or(file_config.region)
            .unwrap_or_else(|| "au".to_string());

        // Resolve custom endpoint URL
        let endpoint_url = overrides
            .endpoint_url
            .or_else(|| std::env::var("PLERION_ENDPOINT_URL").ok())
            .or(file_config.endpoint_url);

        // Only validate region when no custom endpoint is set
        if endpoint_url.is_none() && !VALID_REGIONS.contains(&region.as_str()) {
            return Err(PlerionError::InvalidRegion(region));
        }

        // Resolve output format
        let output = overrides
            .output
            .or(file_config.output)
            .unwrap_or(crate::output::OutputFormat::Table);

        // Resolve no_color: env NO_COLOR or --no-color flag
        let no_color = overrides.no_color
            || std::env::var("NO_COLOR").is_ok()
            || std::env::var("PLERION_NO_COLOR").is_ok();

        Ok(Config {
            api_key,
            region,
            profile,
            output,
            no_color,
            query: overrides.query,
            endpoint_url,
        })
    }

    pub fn base_url(&self) -> String {
        if let Some(url) = &self.endpoint_url {
            // Trim trailing slash for consistency
            url.trim_end_matches('/').to_string()
        } else {
            format!("https://{}.api.plerion.com", self.region)
        }
    }

    pub fn config_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".plerion")
    }
}

/// CLI-level overrides (from flags)
#[derive(Debug, Default)]
pub struct ConfigOverrides {
    pub profile: Option<String>,
    pub api_key: Option<String>,
    pub region: Option<String>,
    pub endpoint_url: Option<String>,
    pub output: Option<crate::output::OutputFormat>,
    pub no_color: bool,
    pub query: Option<String>,
}
