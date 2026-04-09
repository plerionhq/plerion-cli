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
