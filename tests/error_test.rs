use plerion::error::PlerionError;
use std::process::Command;

#[test]
fn test_missing_api_key_display() {
    let err = PlerionError::MissingApiKey;
    let s = err.to_string();
    assert!(s.contains("API key"));
    assert!(s.contains("plerion configure"));
}

#[test]
fn test_missing_region_display() {
    let err = PlerionError::MissingRegion;
    let s = err.to_string();
    assert!(s.contains("region"));
}

#[test]
fn test_invalid_region_display() {
    let err = PlerionError::InvalidRegion("xyz".to_string());
    let s = err.to_string();
    assert!(s.contains("Invalid region"));
    assert!(s.contains("xyz"));
    assert!(s.contains("au"));
}

#[test]
fn test_profile_not_found_display() {
    let err = PlerionError::ProfileNotFound("staging".to_string());
    let s = err.to_string();
    assert!(s.contains("staging"));
    assert!(s.contains("not found"));
}

#[test]
fn test_api_error_display() {
    let err = PlerionError::ApiError {
        status: 403,
        message: "Forbidden".to_string(),
    };
    let s = err.to_string();
    assert!(s.contains("403"));
    assert!(s.contains("Forbidden"));
}

#[test]
fn test_parse_error_display() {
    let err = PlerionError::ParseError("invalid json".to_string());
    let s = err.to_string();
    assert!(s.contains("parse"));
    assert!(s.contains("invalid json"));
}

#[test]
fn test_config_error_display() {
    let err = PlerionError::ConfigError("file not found".to_string());
    let s = err.to_string();
    assert!(s.contains("Config"));
    assert!(s.contains("file not found"));
}

#[test]
fn test_io_error_from() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
    let err: PlerionError = io_err.into();
    let s = err.to_string();
    assert!(s.contains("no such file"));
}

#[test]
fn test_error_is_debug() {
    let err = PlerionError::MissingApiKey;
    let debug = format!("{:?}", err);
    assert!(debug.contains("MissingApiKey"));
}

#[test]
fn test_profile_not_found_includes_configure_hint() {
    let err = PlerionError::ProfileNotFound("nonexistent".to_string());
    let s = err.to_string();
    assert!(s.contains("nonexistent"));
    assert!(s.contains("not found"));
    assert!(s.contains("plerion configure"));
}

#[test]
fn test_cli_connection_refused_error() {
    let binary = env!("CARGO_BIN_EXE_plerion");
    let output = Command::new(binary)
        .args(["tenant", "get", "--output", "json"])
        .env("PLERION_API_KEY", "test-key")
        .env("PLERION_ENDPOINT_URL", "http://127.0.0.1:1")
        .output()
        .expect("failed to execute plerion binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Connection failed"),
        "Error should say 'Connection failed', got: {stderr}"
    );
}

#[test]
fn test_cli_nonexistent_profile_mentions_profile_name() {
    let binary = env!("CARGO_BIN_EXE_plerion");
    let output = Command::new(binary)
        .args(["--profile", "nonexistent", "tenant", "get"])
        .env_remove("PLERION_API_KEY")
        .env_remove("PLERION_PROFILE")
        .env_remove("PLERION_ENDPOINT_URL")
        .output()
        .expect("failed to execute plerion binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("nonexistent"),
        "Error should mention the profile name, got: {stderr}"
    );
}
