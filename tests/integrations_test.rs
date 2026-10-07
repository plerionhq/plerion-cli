use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::integrations};

#[tokio::test]
async fn test_list_integrations() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [
            {
                "integrationId": "int-123",
                "name": "AWS Prod",
                "provider": "AWS",
                "type": "AWSAccount",
                "status": "Active",
                "riskScore": 8.19,
                "awsAccountId": "123456789012",
                "createdAt": "2023-02-04T06:07:09.092Z"
            }
        ],
        "meta": { "perPage": 10, "cursor": "abc", "total": 1 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::UrlEncoded("perPage".to_string(), "10".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = integrations::list_integrations(&client, Some(10), None, false).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].integration_id.as_deref(), Some("int-123"));
    assert_eq!(resp.data[0].provider.as_deref(), Some("AWS"));
    assert_eq!(resp.data[0].aws_account_id.as_deref(), Some("123456789012"));
}

#[tokio::test]
async fn test_list_integrations_with_include_total() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "perPage": 10, "total": 42 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("includeTotal".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = integrations::list_integrations(&client, Some(10), None, true).await.unwrap();
    assert_eq!(resp.data.len(), 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_integrations_with_cursor() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [],
        "meta": { "perPage": 10 }
    });
    let mock = server
        .mock("GET", "/v1/tenant/integrations")
        .match_query(mockito::Matcher::UrlEncoded("cursor".to_string(), "page2".to_string()))
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let _resp = integrations::list_integrations(&client, Some(10), Some("page2"), false).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_replace_user_defined_tags() {
    use plerion::api::models::integrations::{ReplaceUserDefinedTagsRequest, UserDefinedTag};
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": { "tags": [
            { "key": "Division", "value": "payments", "source": "user-defined" },
            { "key": "Environment", "value": "prod", "source": "cloud-provider" }
        ] }
    });
    let mock = server
        .mock("PUT", "/v1/tenant/integrations/int-1/user-defined-tags")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "tags": [ { "key": "Division", "value": "payments" } ]
        })))
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let req = ReplaceUserDefinedTagsRequest {
        tags: vec![UserDefinedTag {
            key: "Division".into(),
            value: "payments".into(),
        }],
    };
    let resp = integrations::replace_user_defined_tags(&client, "int-1", &req)
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.tags.len(), 2);
    assert_eq!(resp.data.tags[1].source.as_deref(), Some("cloud-provider"));
}

#[tokio::test]
async fn test_replace_user_defined_tags_empty_list_clears() {
    use plerion::api::models::integrations::ReplaceUserDefinedTagsRequest;
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", "/v1/tenant/integrations/int-1/user-defined-tags")
        .match_body(mockito::Matcher::Json(serde_json::json!({ "tags": [] })))
        .with_status(200)
        .with_body(r#"{"data":{"tags":[]}}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let req = ReplaceUserDefinedTagsRequest { tags: vec![] };
    let resp = integrations::replace_user_defined_tags(&client, "int-1", &req)
        .await
        .unwrap();
    mock.assert_async().await;
    assert!(resp.data.tags.is_empty());
}

#[tokio::test]
async fn test_replace_user_defined_tags_not_found() {
    use plerion::api::models::integrations::ReplaceUserDefinedTagsRequest;
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("PUT", "/v1/tenant/integrations/missing/user-defined-tags")
        .with_status(404)
        .with_body(r#"{"message":"Not found"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let req = ReplaceUserDefinedTagsRequest { tags: vec![] };
    let err = integrations::replace_user_defined_tags(&client, "missing", &req)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("404"));
}

#[test]
fn test_parse_tag_splits_on_first_equals() {
    use plerion::cli::integrations::parse_tag;
    let t = parse_tag("Owner=team=a").unwrap();
    assert_eq!((t.key.as_str(), t.value.as_str()), ("Owner", "team=a"));
    assert!(parse_tag("Owner").is_err());
    assert!(parse_tag("=x").is_err());
    assert!(parse_tag("x=").is_err());
}

#[test]
fn test_tags_to_send_rules() {
    use plerion::cli::integrations::{parse_tag, tags_to_send};
    assert!(tags_to_send(&[], true).unwrap().is_empty());
    assert!(tags_to_send(&[], false)
        .unwrap_err()
        .to_string()
        .contains("--clear"));
    let dup = [parse_tag("a=1").unwrap(), parse_tag("a=2").unwrap()];
    assert!(tags_to_send(&dup, false)
        .unwrap_err()
        .to_string()
        .contains("more than once"));
    // Keys are case-sensitive, so these are distinct.
    let ok = [parse_tag("a=1").unwrap(), parse_tag("A=2").unwrap()];
    assert_eq!(tags_to_send(&ok, false).unwrap().len(), 2);
}
