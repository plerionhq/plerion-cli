use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlerionError {
    #[error("No API key found. Set PLERION_API_KEY or run `plerion configure`.")]
    MissingApiKey,

    #[error("No region configured. Set PLERION_REGION or run `plerion configure`.")]
    #[allow(dead_code)]
    MissingRegion,

    #[error("Invalid region '{0}'. Valid regions: au, sg1, in1, us1")]
    InvalidRegion(String),

    #[error("Profile '{0}' not found in credentials file.")]
    #[allow(dead_code)]
    ProfileNotFound(String),

    #[error("HTTP error {status}: {message}")]
    ApiError { status: u16, message: String },

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Config file error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
}
