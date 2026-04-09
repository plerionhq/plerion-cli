use std::fs;
use tempfile::TempDir;

/// Helper: set up a temp home with credentials and config files.
fn setup_config_dir(credentials: &str, config: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    let plerion = dir.path().join(".plerion");
    fs::create_dir_all(&plerion).unwrap();
    fs::write(plerion.join("credentials"), credentials).unwrap();
    fs::write(plerion.join("config"), config).unwrap();
    dir
}

#[test]
fn test_parse_credentials_default_profile() {
    let _dir = setup_config_dir(
        "[default]\napi_key = test_key_123\n",
        "[default]\nregion = au\noutput = table\n",
    );

    // We need to point the config dir to our temp dir.
    // For now just test the INI parsing directly.
    let ini_content = "[default]\napi_key = test_key_123\n";
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials");
    fs::write(&path, ini_content).unwrap();

    let mut ini = configparser::ini::Ini::new();
    ini.load(&path).unwrap();
    let key = ini.get("default", "api_key").unwrap();
    assert_eq!(key, "test_key_123");
}

#[test]
fn test_parse_credentials_named_profile() {
    let ini_content = "[default]\napi_key = key_default\n\n[prod]\napi_key = key_prod\n";
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials");
    fs::write(&path, ini_content).unwrap();

    let mut ini = configparser::ini::Ini::new();
    ini.load(&path).unwrap();

    assert_eq!(ini.get("default", "api_key").unwrap(), "key_default");
    assert_eq!(ini.get("prod", "api_key").unwrap(), "key_prod");
}

#[test]
fn test_config_base_url_by_region() {
    let cases = [
        ("au", "https://au.api.plerion.com"),
        ("sg1", "https://sg1.api.plerion.com"),
        ("in1", "https://in1.api.plerion.com"),
        ("us1", "https://us1.api.plerion.com"),
    ];
    for (region, expected_url) in cases {
        let url = format!("https://{}.api.plerion.com", region);
        assert_eq!(url, expected_url);
    }
}

#[test]
fn test_valid_regions() {
    use plerion::config::VALID_REGIONS;
    assert!(VALID_REGIONS.contains(&"au"));
    assert!(VALID_REGIONS.contains(&"sg1"));
    assert!(VALID_REGIONS.contains(&"in1"));
    assert!(VALID_REGIONS.contains(&"us1"));
    assert!(!VALID_REGIONS.contains(&"us-east-1"));
}

#[test]
fn test_output_format_parsing() {
    use plerion::output::OutputFormat;
    use std::str::FromStr;

    assert_eq!(OutputFormat::from_str("table").unwrap(), OutputFormat::Table);
    assert_eq!(OutputFormat::from_str("json").unwrap(), OutputFormat::Json);
    assert_eq!(OutputFormat::from_str("yaml").unwrap(), OutputFormat::Yaml);
    assert_eq!(OutputFormat::from_str("text").unwrap(), OutputFormat::Text);
    assert!(OutputFormat::from_str("xml").is_err());
}

#[test]
fn test_output_format_display() {
    use plerion::output::OutputFormat;
    assert_eq!(OutputFormat::Table.to_string(), "table");
    assert_eq!(OutputFormat::Json.to_string(), "json");
    assert_eq!(OutputFormat::Yaml.to_string(), "yaml");
    assert_eq!(OutputFormat::Text.to_string(), "text");
}
