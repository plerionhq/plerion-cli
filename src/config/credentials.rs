use crate::error::PlerionError;
use crate::output::OutputFormat;
use configparser::ini::Ini;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct ProfileConfig {
    pub api_key: Option<String>,
    pub region: Option<String>,
    pub output: Option<OutputFormat>,
    pub endpoint_url: Option<String>,
}

pub fn credentials_path() -> PathBuf {
    super::Config::config_dir().join("credentials")
}

pub fn config_path() -> PathBuf {
    super::Config::config_dir().join("config")
}

/// Load a named profile from ~/.plerion/credentials and ~/.plerion/config.
pub fn load_profile(profile: &str) -> Result<ProfileConfig, PlerionError> {
    let mut result = ProfileConfig::default();

    // Load api_key from credentials file
    if let Ok(creds) = load_ini(&credentials_path()) {
        if let Some(key) = creds.get(profile, "api_key") {
            result.api_key = Some(key);
        }
    }

    // Load region/output from config file
    // Config uses [default] for default, [profile <name>] for named profiles
    if let Ok(cfg) = load_ini(&config_path()) {
        let section = if profile == "default" {
            profile.to_string()
        } else {
            format!("profile {profile}")
        };

        if let Some(r) = cfg.get(&section, "region") {
            result.region = Some(r);
        }
        if let Some(o) = cfg.get(&section, "output") {
            result.output = OutputFormat::from_str(&o).ok();
        }
        if let Some(u) = cfg.get(&section, "endpoint_url") {
            result.endpoint_url = Some(u);
        }
    }

    Ok(result)
}

fn load_ini(path: &std::path::Path) -> Result<Ini, PlerionError> {
    if !path.exists() {
        return Err(PlerionError::ConfigError(format!(
            "{} not found",
            path.display()
        )));
    }
    let path_str = path
        .to_str()
        .ok_or_else(|| PlerionError::ConfigError("Non-UTF8 path".to_string()))?;
    let mut ini = Ini::new();
    ini.load(path_str)
        .map_err(|e| PlerionError::ConfigError(e.to_string()))?;
    Ok(ini)
}

/// List all profile names from both files.
pub fn list_profiles() -> Vec<String> {
    let mut names = std::collections::HashSet::new();

    if let Ok(creds) = load_ini(&credentials_path()) {
        for section in creds.sections() {
            names.insert(section);
        }
    }
    if let Ok(cfg) = load_ini(&config_path()) {
        for section in cfg.sections() {
            let name = section
                .strip_prefix("profile ")
                .unwrap_or(&section)
                .to_string();
            names.insert(name);
        }
    }

    let mut list: Vec<String> = names.into_iter().collect();
    list.sort();
    list
}

/// Write a profile to credentials and config files.
pub fn write_profile(
    profile: &str,
    api_key: &str,
    region: &str,
    output: &str,
) -> Result<(), PlerionError> {
    let dir = super::Config::config_dir();
    std::fs::create_dir_all(&dir)?;

    // Write credentials
    let creds_path = credentials_path();
    let mut creds = Ini::new();
    if creds_path.exists() {
        let p = creds_path.to_str().unwrap();
        creds.load(p).map_err(|e| PlerionError::ConfigError(e.to_string()))?;
    }
    creds.set(profile, "api_key", Some(api_key.to_string()));
    let creds_str = creds_path.to_str().unwrap().to_string();
    creds
        .write(&creds_str)
        .map_err(|e| PlerionError::ConfigError(e.to_string()))?;

    // Write config
    let cfg_path = config_path();
    let mut cfg = Ini::new();
    if cfg_path.exists() {
        let p = cfg_path.to_str().unwrap();
        cfg.load(p).map_err(|e| PlerionError::ConfigError(e.to_string()))?;
    }
    let section = if profile == "default" {
        "default".to_string()
    } else {
        format!("profile {profile}")
    };
    cfg.set(&section, "region", Some(region.to_string()));
    cfg.set(&section, "output", Some(output.to_string()));
    let cfg_str = cfg_path.to_str().unwrap().to_string();
    cfg.write(&cfg_str)
        .map_err(|e| PlerionError::ConfigError(e.to_string()))?;

    Ok(())
}
