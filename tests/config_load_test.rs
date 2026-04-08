use plerion::config::{Config, ConfigOverrides, VALID_REGIONS};
use plerion::output::OutputFormat;

#[test]
fn test_config_base_url_with_endpoint_override() {
    let config = Config {
        api_key: "key".to_string(),
        region: "au".to_string(),
        profile: "default".to_string(),
        output: OutputFormat::Table,
        no_color: false,
        query: None,
        endpoint_url: Some("https://custom.api.example.com/".to_string()),
    };
    // Should strip trailing slash
    assert_eq!(config.base_url(), "https://custom.api.example.com");
}

#[test]
fn test_config_base_url_without_endpoint() {
    let config = Config {
        api_key: "key".to_string(),
        region: "sg1".to_string(),
        profile: "default".to_string(),
        output: OutputFormat::Json,
        no_color: true,
        query: None,
        endpoint_url: None,
    };
    assert_eq!(config.base_url(), "https://sg1.api.plerion.com");
}

#[test]
fn test_config_load_with_overrides() {
    // Provide all values via overrides so no file/env lookup needed
    let overrides = ConfigOverrides {
        profile: Some("test".to_string()),
        api_key: Some("my-api-key".to_string()),
        region: Some("us1".to_string()),
        endpoint_url: None,
        output: Some(OutputFormat::Json),
        no_color: true,
        query: Some("data[0]".to_string()),
    };
    let config = Config::load(overrides).unwrap();
    assert_eq!(config.api_key, "my-api-key");
    assert_eq!(config.region, "us1");
    assert_eq!(config.profile, "test");
    assert_eq!(config.output, OutputFormat::Json);
    assert!(config.no_color);
    assert_eq!(config.query.as_deref(), Some("data[0]"));
    assert_eq!(config.base_url(), "https://us1.api.plerion.com");
}

#[test]
fn test_config_invalid_region_validation() {
    // Directly test that invalid regions are not in VALID_REGIONS
    assert!(!VALID_REGIONS.contains(&"invalid-region"));
    assert!(!VALID_REGIONS.contains(&"eu-west-1"));
    assert!(!VALID_REGIONS.contains(&""));
}

#[test]
fn test_config_load_invalid_region_skipped_with_endpoint_url() {
    // Custom endpoint_url should bypass region validation
    let overrides = ConfigOverrides {
        api_key: Some("key".to_string()),
        region: Some("custom-region".to_string()),
        endpoint_url: Some("https://custom.api.com".to_string()),
        ..Default::default()
    };
    let config = Config::load(overrides).unwrap();
    assert_eq!(config.region, "custom-region");
    assert_eq!(config.base_url(), "https://custom.api.com");
}

#[test]
fn test_config_overrides_default() {
    // ConfigOverrides::default() should have all Nones
    let overrides = ConfigOverrides::default();
    assert!(overrides.api_key.is_none());
    assert!(overrides.region.is_none());
    assert!(overrides.profile.is_none());
    assert!(overrides.endpoint_url.is_none());
    assert!(overrides.output.is_none());
    assert!(!overrides.no_color);
    assert!(overrides.query.is_none());
}

#[test]
fn test_config_load_defaults_to_au_region() {
    let overrides = ConfigOverrides {
        api_key: Some("key".to_string()),
        // No region specified
        ..Default::default()
    };
    let config = Config::load(overrides).unwrap();
    assert_eq!(config.region, "au");
}

#[test]
fn test_config_load_defaults_to_table_output() {
    let overrides = ConfigOverrides {
        api_key: Some("key".to_string()),
        ..Default::default()
    };
    let config = Config::load(overrides).unwrap();
    assert_eq!(config.output, OutputFormat::Table);
}

#[test]
fn test_config_load_defaults_profile_to_default() {
    let overrides = ConfigOverrides {
        api_key: Some("key".to_string()),
        ..Default::default()
    };
    let config = Config::load(overrides).unwrap();
    assert_eq!(config.profile, "default");
}

#[test]
fn test_config_dir_returns_plerion_path() {
    let dir = Config::config_dir();
    assert!(dir.to_string_lossy().contains(".plerion"));
}

#[test]
fn test_all_valid_regions_produce_correct_urls() {
    for region in VALID_REGIONS {
        let config = Config {
            api_key: "key".to_string(),
            region: region.to_string(),
            profile: "default".to_string(),
            output: OutputFormat::Table,
            no_color: false,
            query: None,
            endpoint_url: None,
        };
        let expected = format!("https://{}.api.plerion.com", region);
        assert_eq!(config.base_url(), expected);
    }
}
