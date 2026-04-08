use mockito::Server;
use plerion::api::client::PlerionClient;

#[test]
fn test_client_creation() {
    let client = PlerionClient::with_base_url("https://example.com", "test-key").unwrap();
    // Verify base_url is stored
    assert_eq!(client.base_url, "https://example.com");
}

#[tokio::test]
async fn test_client_sends_auth_header() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/test")
        .match_header("Authorization", "Bearer my-api-key")
        .with_status(200)
        .with_body(r#""ok""#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "my-api-key").unwrap();
    let _: String = client.execute(client.get("/test")).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_client_sends_content_type() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/test")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_body(r#""ok""#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let _: String = client.execute(client.get("/test")).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_client_sends_user_agent() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/test")
        .match_header("User-Agent", mockito::Matcher::Regex("plerion-cli/.*".to_string()))
        .with_status(200)
        .with_body(r#""ok""#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let _: String = client.execute(client.get("/test")).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_client_post() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/test")
        .match_header("Authorization", "Bearer key")
        .with_status(201)
        .with_body(r#"{"id": "new"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp: serde_json::Value = client.execute(client.post("/test")).await.unwrap();
    assert_eq!(resp["id"], "new");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_client_patch() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PATCH", "/test/123")
        .with_status(200)
        .with_body(r#"{"updated": true}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp: serde_json::Value = client.execute(client.patch("/test/123")).await.unwrap();
    assert_eq!(resp["updated"], true);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_client_delete() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", "/test/123")
        .with_status(200)
        .with_body(r#""deleted""#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let _: String = client.execute(client.delete("/test/123")).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_client_400_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/bad")
        .with_status(400)
        .with_body(r#"{"message": "Bad request"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result: Result<serde_json::Value, _> = client.execute(client.get("/bad")).await;
    let err = result.unwrap_err().to_string();
    assert!(err.contains("400"));
    assert!(err.contains("Bad request"));
}

#[tokio::test]
async fn test_client_401_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/auth")
        .with_status(401)
        .with_body(r#"{"message": "Unauthorized"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result: Result<serde_json::Value, _> = client.execute(client.get("/auth")).await;
    assert!(result.unwrap_err().to_string().contains("401"));
}

#[tokio::test]
async fn test_client_500_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/fail")
        .with_status(500)
        .with_body(r#"{"message": "Internal server error"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result: Result<serde_json::Value, _> = client.execute(client.get("/fail")).await;
    assert!(result.unwrap_err().to_string().contains("500"));
}

#[tokio::test]
async fn test_client_execute_bytes() {
    let mut server = Server::new_async().await;
    let content = b"binary-content-here";
    let _mock = server
        .mock("GET", "/download")
        .with_status(200)
        .with_body(content)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let bytes = client.execute_bytes(client.get("/download")).await.unwrap();
    assert_eq!(bytes.as_ref(), content);
}

#[tokio::test]
async fn test_client_execute_bytes_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/download")
        .with_status(403)
        .with_body("Forbidden")
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result = client.execute_bytes(client.get("/download")).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("403"));
}

#[tokio::test]
async fn test_client_parse_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/bad-json")
        .with_status(200)
        .with_body("not-valid-json")
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let result: Result<serde_json::Value, _> = client.execute(client.get("/bad-json")).await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("parse"));
}
