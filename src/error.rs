use thiserror::Error;

fn format_request_error(e: &reqwest::Error) -> String {
    let url_part = e
        .url()
        .map(|u| format!(" for {u}"))
        .unwrap_or_default();

    if e.is_timeout() {
        return format!("Request timed out{url_part}");
    }
    if e.is_connect() {
        return format!("Connection failed{url_part}");
    }
    if let Some(status) = e.status() {
        return format!("HTTP error {}{url_part}", status.as_u16());
    }
    format!("Request failed{url_part}: {e}")
}

#[derive(Error, Debug)]
pub enum PlerionError {
    #[error("No API key found. Set PLERION_API_KEY or run `plerion configure`.")]
    MissingApiKey,

    #[error("No region configured. Set PLERION_REGION or run `plerion configure`.")]
    #[allow(dead_code)]
    MissingRegion,

    #[error("Invalid region '{0}'. Valid regions: au, sg1, in1, us1")]
    InvalidRegion(String),

    #[error("Profile '{0}' not found. Check ~/.plerion/credentials or run `plerion configure`.")]
    ProfileNotFound(String),

    #[error("HTTP error {status}: {message}")]
    ApiError { status: u16, message: String },

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Config file error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{}", format_request_error(.0))]
    Request(#[from] reqwest::Error),
}
