use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::aws};

#[tokio::test]
async fn test_get_external_id() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "externalId": "ext-abc-123" } });
    let _mock = server
        .mock("GET", "/v1/tenant/external-id")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = aws::get_external_id(&client).await.unwrap();
    assert_eq!(resp["data"]["externalId"], "ext-abc-123");
}

#[tokio::test]
async fn test_get_cloudformation_template() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "templateURL": "https://example.com/template.yaml" } });
    let mock = server
        .mock("GET", "/v1/tenant/cloudformation-templates")
        .match_query(mockito::Matcher::UrlEncoded("type".to_string(), "AWSAccount".to_string()))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = aws::get_cloudformation_template(&client, "AWSAccount").await.unwrap();
    assert_eq!(resp["data"]["templateURL"], "https://example.com/template.yaml");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_generate_token() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({ "data": { "token": "tmp-token-xyz" } });
    let mock = server
        .mock("POST", "/v1/tenant/integrations/token")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = aws::generate_token(&client, "int-123").await.unwrap();
    assert_eq!(resp["data"]["token"], "tmp-token-xyz");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_external_id_forbidden() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/external-id")
        .with_status(403)
        .with_body(r#"{"message": "Forbidden"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result = aws::get_external_id(&client).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("403"));
}
