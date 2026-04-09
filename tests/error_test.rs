use plerion::error::PlerionError;

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
