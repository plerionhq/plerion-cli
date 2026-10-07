use mockito::{Matcher, Server};
use plerion::api::{
    client::PlerionClient,
    endpoints::metrics::query_metrics,
    models::metrics::{MetricPeriod, MetricSelector, QueryMetricsRequest},
};

fn request() -> QueryMetricsRequest {
    QueryMetricsRequest {
        metric: MetricSelector {
            namespace: "finding".into(),
            metric_names: vec!["failed_count".into()],
        },
        period: MetricPeriod {
            interval: 86400,
            start: "2026-08-01T00:00:00Z".into(),
            end: "2026-08-03T00:00:00Z".into(),
            stat: "latest".into(),
        },
        integration_ids: None,
        integration_group_ids: None,
        asset_group_ids: None,
        environment_ids: None,
    }
}

fn response() -> String {
    serde_json::json!({
        "data": [
            { "timestamp": "2026-08-01T00:00:00.000Z", "metrics": { "failed_count": 128 } },
            { "timestamp": "2026-08-02T00:00:00.000Z", "metrics": { "failed_count": 0 } }
        ],
        "meta": {
            "period": { "start": "2026-08-01T00:00:00Z", "end": "2026-08-03T00:00:00Z", "interval": 86400, "stat": "latest" },
            "namespace": "finding",
            "metricNames": ["failed_count"]
        }
    })
    .to_string()
}

/// Unset filters are left out: the API rejects unknown or unexpected fields.
#[tokio::test]
async fn test_query_metrics_sends_only_set_fields() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/tenant/metrics")
        .match_body(Matcher::Json(serde_json::json!({
            "metric": { "namespace": "finding", "metricNames": ["failed_count"] },
            "period": { "interval": 86400, "start": "2026-08-01T00:00:00Z", "end": "2026-08-03T00:00:00Z", "stat": "latest" }
        })))
        .with_status(200)
        .with_body(response())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let resp = query_metrics(&client, &request()).await.unwrap();
    assert_eq!(resp.data.len(), 2);
    let rows: Vec<_> = resp.data[0].rows().collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].metric, "failed_count");
    assert_eq!(rows[0].value, "128");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_query_metrics_sends_filters_as_camel_case_arrays() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/tenant/metrics")
        .match_body(Matcher::PartialJson(serde_json::json!({
            "integrationIds": ["i-1", "i-2"],
            "integrationGroupIds": ["g-1"],
            "assetGroupIds": ["ag-1"],
            "environmentIds": ["e-1"]
        })))
        .with_status(200)
        .with_body(response())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let mut body = request();
    body.integration_ids = Some(vec!["i-1".into(), "i-2".into()]);
    body.integration_group_ids = Some(vec!["g-1".into()]);
    body.asset_group_ids = Some(vec!["ag-1".into()]);
    body.environment_ids = Some(vec!["e-1".into()]);
    query_metrics(&client, &body).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_query_metrics_surfaces_403() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("POST", "/v1/tenant/metrics")
        .with_status(403)
        .with_body(r#"{"errors":[{"code":"IntegrationAccessDenied","message":"integration access denied"}]}"#)
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "k").unwrap();
    let err = query_metrics(&client, &request())
        .await
        .unwrap_err()
        .to_string();
    assert!(err.contains("403"), "{err}");
}
