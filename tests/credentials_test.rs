use std::fs;
use tempfile::TempDir;

/// Test the INI parsing and profile reading/writing logic
/// using the configparser crate directly (since credentials.rs
/// uses hardcoded paths, we test the underlying logic).

#[test]
fn test_write_and_read_credentials() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials");

    let mut ini = configparser::ini::Ini::new();
    ini.set("default", "api_key", Some("test-key-123".to_string()));
    ini.write(path.to_str().unwrap()).unwrap();

    let mut ini2 = configparser::ini::Ini::new();
    ini2.load(path.to_str().unwrap()).unwrap();
    assert_eq!(ini2.get("default", "api_key").unwrap(), "test-key-123");
}

#[test]
fn test_write_multiple_profiles() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials");

    let mut ini = configparser::ini::Ini::new();
    ini.set("default", "api_key", Some("key-default".to_string()));
    ini.set("prod", "api_key", Some("key-prod".to_string()));
    ini.set("staging", "api_key", Some("key-staging".to_string()));
    ini.write(path.to_str().unwrap()).unwrap();

    let mut ini2 = configparser::ini::Ini::new();
    ini2.load(path.to_str().unwrap()).unwrap();

    assert_eq!(ini2.get("default", "api_key").unwrap(), "key-default");
    assert_eq!(ini2.get("prod", "api_key").unwrap(), "key-prod");
    assert_eq!(ini2.get("staging", "api_key").unwrap(), "key-staging");

    let sections = ini2.sections();
    assert!(sections.contains(&"default".to_string()));
    assert!(sections.contains(&"prod".to_string()));
    assert!(sections.contains(&"staging".to_string()));
}

#[test]
fn test_config_file_with_named_profiles() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config");

    let content = "[default]\nregion = au\noutput = table\n\n[prod]\nregion = us1\noutput = json\n";
    fs::write(&path, content).unwrap();

    let mut ini = configparser::ini::Ini::new();
    ini.load(path.to_str().unwrap()).unwrap();

    assert_eq!(ini.get("default", "region").unwrap(), "au");
    assert_eq!(ini.get("default", "output").unwrap(), "table");
    assert_eq!(ini.get("prod", "region").unwrap(), "us1");
    assert_eq!(ini.get("prod", "output").unwrap(), "json");
}

#[test]
fn test_config_with_endpoint_url() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config");

    let content = "[default]\nregion = au\nendpoint_url = https://custom.api.example.com\n";
    fs::write(&path, content).unwrap();

    let mut ini = configparser::ini::Ini::new();
    ini.load(path.to_str().unwrap()).unwrap();

    assert_eq!(
        ini.get("default", "endpoint_url").unwrap(),
        "https://custom.api.example.com"
    );
}

#[test]
fn test_update_existing_profile() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials");

    let mut ini = configparser::ini::Ini::new();
    ini.set("default", "api_key", Some("old-key".to_string()));
    ini.write(path.to_str().unwrap()).unwrap();

    // Load, update, save
    let mut ini2 = configparser::ini::Ini::new();
    ini2.load(path.to_str().unwrap()).unwrap();
    ini2.set("default", "api_key", Some("new-key".to_string()));
    ini2.write(path.to_str().unwrap()).unwrap();

    // Verify
    let mut ini3 = configparser::ini::Ini::new();
    ini3.load(path.to_str().unwrap()).unwrap();
    assert_eq!(ini3.get("default", "api_key").unwrap(), "new-key");
}

#[test]
fn test_missing_key_returns_none() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials");

    let mut ini = configparser::ini::Ini::new();
    ini.set("default", "api_key", Some("key".to_string()));
    ini.write(path.to_str().unwrap()).unwrap();

    let mut ini2 = configparser::ini::Ini::new();
    ini2.load(path.to_str().unwrap()).unwrap();

    assert!(ini2.get("default", "nonexistent").is_none());
    assert!(ini2.get("nonexistent_section", "api_key").is_none());
}

#[test]
fn test_empty_credentials_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials");
    fs::write(&path, "").unwrap();

    let mut ini = configparser::ini::Ini::new();
    ini.load(path.to_str().unwrap()).unwrap();

    assert!(ini.sections().is_empty());
    assert!(ini.get("default", "api_key").is_none());
}

#[test]
fn test_profile_config_struct() {
    use plerion::config::credentials::ProfileConfig;

    let pc = ProfileConfig::default();
    assert!(pc.api_key.is_none());
    assert!(pc.region.is_none());
    assert!(pc.output.is_none());
    assert!(pc.endpoint_url.is_none());
}
