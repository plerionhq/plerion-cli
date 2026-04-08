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
