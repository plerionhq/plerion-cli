use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::vulnerabilities::{list_vulnerabilities, ListVulnerabilitiesParams}};

#[tokio::test]
async fn test_list_vulnerabilities_deserializes_correctly() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "vulnerabilityId": "CVE-2022-22965",
                "title": "Spring4Shell",
                "severityLevel": "HIGH",
                "provider": "AWS",
                "assetType": "AWS::EC2::Instance",
                "hasKev": true,
                "hasExploit": false,
                "hasVendorFix": true,
                "firstObservedAt": "2023-10-27T04:54:37.830Z"
            }
        ],
        "meta": { "page": 1, "perPage": 50, "total": 1, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_vulnerabilities(&client, &ListVulnerabilitiesParams::default()).await.unwrap();

    assert_eq!(resp.data.len(), 1);
    let v = &resp.data[0];
    assert_eq!(v.vulnerability_id.as_deref(), Some("CVE-2022-22965"));
    assert_eq!(v.has_kev, Some(true));
    assert_eq!(v.severity_level.as_deref(), Some("HIGH"));
}

#[tokio::test]
async fn test_list_vulnerabilities_with_new_filters() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("executionIds".to_string(), "exec-1".to_string()),
            mockito::Matcher::UrlEncoded("targetName".to_string(), "openssl".to_string()),
            mockito::Matcher::UrlEncoded("targetType".to_string(), "library".to_string()),
            mockito::Matcher::UrlEncoded("targetClass".to_string(), "lang-pkgs".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListVulnerabilitiesParams {
        execution_ids: Some("exec-1".to_string()),
        target_name: Some("openssl".to_string()),
        target_type: Some("library".to_string()),
        target_class: Some("lang-pkgs".to_string()),
        ..Default::default()
    };
    let resp = list_vulnerabilities(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_vulnerabilities_with_all_cli_filters() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("vulnerabilityIds".to_string(), "CVE-2024-1".to_string()),
            mockito::Matcher::UrlEncoded("assetGroupIds".to_string(), "ag-1".to_string()),
            mockito::Matcher::UrlEncoded("environmentIds".to_string(), "production".to_string()),
            mockito::Matcher::UrlEncoded("packageName".to_string(), "openssl".to_string()),
            mockito::Matcher::UrlEncoded("isExempted".to_string(), "false".to_string()),
            mockito::Matcher::UrlEncoded("isExploitable".to_string(), "true".to_string()),
            mockito::Matcher::UrlEncoded("firstObservedAtStart".to_string(), "2025-01-01T00:00:00Z".to_string()),
            mockito::Matcher::UrlEncoded("firstObservedAtEnd".to_string(), "2025-06-01T00:00:00Z".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListVulnerabilitiesParams {
        vulnerability_ids: Some("CVE-2024-1".to_string()),
        asset_group_ids: Some("ag-1".to_string()),
        environment_ids: Some("production".to_string()),
        package_name: Some("openssl".to_string()),
        is_exempted: Some(false),
        is_exploitable: Some(true),
        first_observed_at_start: Some("2025-01-01T00:00:00Z".to_string()),
        first_observed_at_end: Some("2025-06-01T00:00:00Z".to_string()),
        ..Default::default()
    };
    let resp = list_vulnerabilities(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_vulnerabilities_deserializes_epss() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            { "vulnerabilityId": "CVE-2021-44228", "epssScore": 0.943, "epssScoreDate": "2026-10-04" },
            { "vulnerabilityId": "CVE-2099-0001", "epssScore": null, "epssScoreDate": null },
            { "vulnerabilityId": "CVE-2099-0002" }
        ],
        "meta": { "page": 1, "perPage": 50, "total": 3, "hasNextPage": false, "hasPreviousPage": false }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_vulnerabilities(&client, &ListVulnerabilitiesParams::default()).await.unwrap();

    assert_eq!(resp.data[0].epss_score, Some(0.943));
    assert_eq!(resp.data[0].epss_score_date.as_deref(), Some("2026-10-04"));
    assert_eq!(resp.data[1].epss_score, None);
    assert_eq!(resp.data[1].epss_score_date, None);
    assert_eq!(resp.data[2].epss_score, None);
}

#[tokio::test]
async fn test_list_vulnerabilities_sends_epss_filters() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "page": 1, "perPage": 50, "total": 0, "hasNextPage": false, "hasPreviousPage": false }
    });
    let mock = server
        .mock("GET", "/v1/tenant/vulnerabilities")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("epssScoreGte".to_string(), "0.1".to_string()),
            mockito::Matcher::UrlEncoded("epssScoreLte".to_string(), "0.5".to_string()),
            mockito::Matcher::UrlEncoded("hasEpssScore".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListVulnerabilitiesParams {
        epss_score_gte: Some(0.1),
        epss_score_lte: Some(0.5),
        has_epss_score: Some(true),
        ..Default::default()
    };
    list_vulnerabilities(&client, &params).await.unwrap();
    mock.assert_async().await;
}

#[test]
fn test_format_epss() {
    use plerion::api::models::vulnerabilities::format_epss;
    assert_eq!(format_epss(None), "");
    assert_eq!(format_epss(Some(0.0)), "0%");
    assert_eq!(format_epss(Some(0.00004)), "<0.1%");
    assert_eq!(format_epss(Some(0.00099)), "<0.1%");
    assert_eq!(format_epss(Some(0.001)), "0.1%");
    assert_eq!(format_epss(Some(0.943)), "94.3%");
    assert_eq!(format_epss(Some(0.5)), "50.0%");
    assert_eq!(format_epss(Some(1.0)), "100.0%");
}

#[test]
fn test_vulnerability_table_shows_percent_and_text_keeps_raw_score() {
    use plerion::api::models::vulnerabilities::Vulnerability;
    use plerion::output::TableRenderable;
    let v: Vulnerability = serde_json::from_value(serde_json::json!({
        "vulnerabilityId": "CVE-2021-44228",
        "epssScore": 0.943,
        "epssScoreDate": "2026-10-04"
    }))
    .unwrap();
    let epss = Vulnerability::headers().iter().position(|h| *h == "EPSS").unwrap();
    let date = Vulnerability::headers().iter().position(|h| *h == "EPSS DATE").unwrap();
    assert_eq!(v.row()[epss], "94.3%");
    assert_eq!(v.row()[date], "2026-10-04");
    assert_eq!(v.text_row()[epss], "0.943");

    let unscored: Vulnerability = serde_json::from_value(serde_json::json!({ "epssScore": null })).unwrap();
    assert_eq!(unscored.row()[epss], "");
    assert_eq!(unscored.text_row()[epss], "");
}

#[test]
fn test_iac_vulnerability_shows_epss() {
    use plerion::api::models::iac::IacVulnerability;
    use plerion::output::TableRenderable;
    let v: IacVulnerability = serde_json::from_value(serde_json::json!({
        "vulnerabilityId": "CVE-2021-44228",
        "epssScore": 0.0004,
        "epssScoreDate": "2026-10-04"
    }))
    .unwrap();
    let epss = IacVulnerability::headers().iter().position(|h| *h == "EPSS").unwrap();
    assert_eq!(v.row().len(), IacVulnerability::headers().len());
    assert_eq!(v.row()[epss], "<0.1%");
    assert_eq!(v.text_row()[epss], "0.0004");
    assert_eq!(v.row()[epss + 1], "2026-10-04");
}

#[test]
fn test_check_epss_range() {
    use plerion::cli::vulnerabilities::check_epss_range;
    assert!(check_epss_range(Some(0.1), Some(0.5), None).is_ok());
    assert!(check_epss_range(Some(0.3), Some(0.3), None).is_ok());
    assert!(check_epss_range(Some(0.0), Some(1.0), Some(true)).is_ok());
    assert!(check_epss_range(None, None, Some(false)).is_ok());
    assert!(check_epss_range(Some(0.6), Some(0.5), None).is_err());
    assert!(check_epss_range(Some(0.1), None, Some(false)).is_err());
    assert!(check_epss_range(Some(0.0), None, Some(false)).is_err());
}
