use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::compliance};

#[tokio::test]
async fn test_list_compliance_frameworks() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": {
            "frameworks": [
                {
                    "id": "CIS-AWSFB-v140",
                    "name": "CIS AWS Foundations Benchmark",
                    "version": "v1.4.0",
                    "posture": 82.13,
                    "passedFindings": 832,
                    "totalFindings": 1013,
                    "isCustom": false,
                    "providers": ["AWS"],
                    "description": "CIS benchmark"
                }
            ],
            "totalPosture": 82.13
        }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/compliance-frameworks")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = compliance::list_compliance_frameworks(&client, None).await.unwrap();
    assert_eq!(resp.data.frameworks.len(), 1);
    assert_eq!(resp.data.frameworks[0].id.as_deref(), Some("CIS-AWSFB-v140"));
    assert_eq!(resp.data.frameworks[0].posture, Some(82.13));
    assert_eq!(resp.data.total_posture, Some(82.13));
}

#[tokio::test]
async fn test_list_compliance_frameworks_custom_filter() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "frameworks": [], "totalPosture": 0.0 } });
    let mock = server
        .mock("GET", "/v1/tenant/compliance-frameworks")
        .match_query(mockito::Matcher::UrlEncoded("custom".to_string(), "true".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    compliance::list_compliance_frameworks(&client, Some(true)).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_request_compliance_report() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "status": "generating" });
    let mock = server
        .mock("POST", "/v1/tenant/integrations/int-1/frameworks/CIS/reports")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = compliance::request_compliance_report(&client, "int-1", "CIS").await.unwrap();
    assert_eq!(resp["status"], "generating");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_download_compliance_report() {
    let mut server = Server::new_async().await;
    let pdf_bytes = b"fake-pdf-content";
    let mock = server
        .mock("GET", "/v1/tenant/integrations/int-1/compliance-frameworks/CIS/download")
        .with_status(200)
        .with_body(pdf_bytes)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let bytes = compliance::download_compliance_report(&client, "int-1", "CIS").await.unwrap();
    assert_eq!(bytes.as_ref(), pdf_bytes);
    mock.assert_async().await;
}
