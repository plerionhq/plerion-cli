use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::well_architected};

#[tokio::test]
async fn test_list_well_architected_frameworks() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "frameworks": [
                {
                    "id": "WAF-2024",
                    "name": "AWS Well-Architected",
                    "version": "2024",
                    "posture": 75.0,
                    "passedFindings": 150,
                    "totalFindings": 200,
                    "isCustom": false,
                    "providers": ["AWS"]
                }
            ],
            "totalPosture": 75.0
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/well-architected-frameworks")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = well_architected::list_well_architected_frameworks(&client).await.unwrap();
    assert_eq!(resp.data.frameworks.len(), 1);
    assert_eq!(resp.data.frameworks[0].id.as_deref(), Some("WAF-2024"));
    assert_eq!(resp.data.frameworks[0].posture, Some(75.0));
}

#[tokio::test]
async fn test_request_well_architected_report() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "status": "generating" });
    let mock = server
        .mock("POST", "/v1/tenant/integrations/int-1/frameworks/WAF/reports")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = well_architected::request_well_architected_report(&client, "int-1", "WAF").await.unwrap();
    assert_eq!(resp["status"], "generating");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_download_well_architected_report() {
    let mut server = Server::new_async().await;
    let content = b"report-bytes";
    let mock = server
        .mock("GET", "/v1/tenant/integrations/int-1/well-architected-frameworks/WAF/download")
        .with_status(200)
        .with_body(content)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let bytes = well_architected::download_well_architected_report(&client, "int-1", "WAF").await.unwrap();
    assert_eq!(bytes.as_ref(), content);
    mock.assert_async().await;
}
