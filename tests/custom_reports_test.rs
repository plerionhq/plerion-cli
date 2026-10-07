use mockito::{Matcher, Server};
use plerion::api::{
    client::PlerionClient, endpoints::custom_reports, models::custom_reports::CustomReportSummary,
};
use plerion::output::TableRenderable;

#[tokio::test]
async fn test_list_custom_reports() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/tenant/custom-reports")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("perPage".into(), "20".into()),
            Matcher::UrlEncoded("cursor".into(), "c1".into()),
        ]))
        .with_status(200)
        .with_body(
            serde_json::json!({
                "data": [{
                    "id": "3f1c7d2e-5a6b-4c8d-9e0f-1a2b3c4d5e6f",
                    "name": "Production security posture",
                    "createdAt": "2026-01-07T13:37:23.388Z",
                    "updatedAt": "2026-02-07T13:37:23.388Z"
                }],
                "meta": { "perPage": 20, "total": 2, "cursor": null }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = custom_reports::list_custom_reports(&client, Some(20), Some("c1"))
        .await
        .unwrap();
    mock.assert_async().await;
    assert_eq!(resp.data.len(), 1);
    assert_eq!(
        resp.data[0].name.as_deref(),
        Some("Production security posture")
    );
    assert_eq!(resp.meta.total, Some(2));
    assert!(resp.meta.cursor.is_none());
}

#[tokio::test]
async fn test_list_custom_reports_forbidden() {
    let mut server = Server::new_async().await;
    server
        .mock("GET", "/v1/tenant/custom-reports")
        .with_status(403)
        .with_body(r#"{"message":"Forbidden"}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let err = custom_reports::list_custom_reports(&client, None, None)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("403"), "{err}");
}

#[test]
fn test_custom_report_table_row() {
    let report: CustomReportSummary = serde_json::from_value(serde_json::json!({
        "id": "r-1", "name": "Posture"
    }))
    .unwrap();
    assert_eq!(CustomReportSummary::headers().len(), report.row().len());
    assert_eq!(report.row()[1], "Posture");
}
