use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::assets::{list_assets, ListAssetsParams}};

fn mock_assets_response() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {
                "id": "prn:assets:abc123",
                "name": "my-ec2-instance",
                "type": "AWS::EC2::Instance",
                "provider": "AWS",
                "region": "us-east-1",
                "riskScore": "9.36",
                "isPubliclyExposed": true,
                "isVulnerable": true,
                "operationalState": "active"
            }
        ],
        "meta": {
            "page": 1,
            "perPage": 50,
            "total": 1,
            "hasNextPage": false,
            "hasPreviousPage": false
        }
    })
}

#[tokio::test]
async fn test_list_assets_deserializes_correctly() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("GET", "/v1/tenant/assets")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(mock_assets_response().to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let resp = list_assets(&client, &ListAssetsParams::default()).await.unwrap();

    assert_eq!(resp.data.len(), 1);
    let a = &resp.data[0];
    assert_eq!(a.name.as_deref(), Some("my-ec2-instance"));
    assert_eq!(a.provider.as_deref(), Some("AWS"));
    assert_eq!(a.is_publicly_exposed, Some(true));
}

#[tokio::test]
async fn test_list_assets_with_provider_filter() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/assets")
        .match_query(mockito::Matcher::UrlEncoded("providers".to_string(), "AWS".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(mock_assets_response().to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "test_key").unwrap();
    let params = ListAssetsParams {
        providers: Some("AWS".to_string()),
        ..Default::default()
    };
    let resp = list_assets(&client, &params).await.unwrap();
    assert_eq!(resp.data.len(), 1);

    mock.assert_async().await;
}
