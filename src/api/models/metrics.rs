use crate::output::TableRenderable;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Body of `POST /v1/tenant/metrics`. The API rejects unknown fields, so the
/// optional filters are omitted when unset.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryMetricsRequest {
    pub metric: MetricSelector,
    pub period: MetricPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_group_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_group_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricSelector {
    pub namespace: String,
    pub metric_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricPeriod {
    pub interval: u32,
    pub start: String,
    pub end: String,
    pub stat: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMetricsResponse {
    pub data: Vec<MetricDataPoint>,
    pub meta: Option<serde_json::Value>,
}

/// One interval: every requested metric's value at `timestamp`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricDataPoint {
    pub timestamp: Option<String>,
    #[serde(default)]
    pub metrics: BTreeMap<String, serde_json::Value>,
}

/// One metric value in one interval. Metric names are only known at runtime,
/// so the table shows one row per value instead of one column per metric.
#[derive(Debug, Serialize, Clone)]
pub struct MetricValueRow {
    pub timestamp: String,
    pub metric: String,
    pub value: String,
}

impl MetricDataPoint {
    pub fn rows(&self) -> impl Iterator<Item = MetricValueRow> + '_ {
        self.metrics.iter().map(|(name, value)| MetricValueRow {
            timestamp: self.timestamp.clone().unwrap_or_default(),
            metric: name.clone(),
            value: match value {
                serde_json::Value::Null => String::new(),
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            },
        })
    }
}

impl TableRenderable for MetricValueRow {
    fn headers() -> Vec<&'static str> {
        vec!["TIMESTAMP", "METRIC", "VALUE"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.timestamp.clone(),
            self.metric.clone(),
            self.value.clone(),
        ]
    }
}
