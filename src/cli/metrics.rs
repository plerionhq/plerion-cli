use crate::api::{
    client::PlerionClient,
    endpoints::metrics::query_metrics,
    models::metrics::{MetricPeriod, MetricSelector, MetricValueRow, QueryMetricsRequest},
};
use crate::config::Config;
use crate::output::{self, OutputFormat};
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct MetricsArgs {
    #[command(subcommand)]
    pub command: MetricsCommands,
}

#[derive(Subcommand, Debug)]
pub enum MetricsCommands {
    /// Query the time series behind the Plerion dashboards
    Query(QueryMetricsArgs),
}

#[derive(Args, Debug)]
pub struct QueryMetricsArgs {
    /// Metric namespace (e.g. finding, asset, vulnerabilities, risk, alert, compliance)
    #[arg(long)]
    pub namespace: String,
    /// Metric names in the namespace (comma-separated, e.g. failed_count)
    #[arg(long)]
    pub metric_names: String,
    /// Length of each interval in seconds, from 900 (15 minutes) to 2592000 (30 days)
    #[arg(long, value_parser = clap::value_parser!(u32).range(900..=2_592_000))]
    pub interval: u32,
    /// Start of the period (ISO 8601 date-time, e.g. 2026-08-01T00:00:00Z)
    #[arg(long)]
    pub start: String,
    /// End of the period (ISO 8601 date-time)
    #[arg(long)]
    pub end: String,
    /// How data points inside an interval are combined
    #[arg(long, value_parser = ["sum", "avg", "max", "min", "latest"])]
    pub stat: String,
    /// Restrict to these integrations (comma-separated UUIDs)
    #[arg(long)]
    pub integration_ids: Option<String>,
    /// Restrict to the integrations in these integration groups (comma-separated UUIDs, max 100)
    #[arg(long)]
    pub integration_group_ids: Option<String>,
    /// Restrict to these asset groups (comma-separated UUIDs)
    #[arg(long)]
    pub asset_group_ids: Option<String>,
    /// Restrict to the integrations in these environments (comma-separated UUIDs)
    #[arg(long)]
    pub environment_ids: Option<String>,
}

fn split_list(v: &str) -> Vec<String> {
    v.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// An absent flag is omitted; a flag that holds no IDs is an error, since
/// sending an empty filter would read as "no restriction".
fn id_list(v: &Option<String>, flag: &str) -> anyhow::Result<Option<Vec<String>>> {
    match v {
        None => Ok(None),
        Some(s) => {
            let ids = split_list(s);
            if ids.is_empty() {
                anyhow::bail!("{flag} must list at least one ID.");
            }
            Ok(Some(ids))
        }
    }
}

fn build_request(a: &QueryMetricsArgs) -> anyhow::Result<QueryMetricsRequest> {
    let namespace = a.namespace.trim().to_string();
    if namespace.is_empty() {
        anyhow::bail!("--namespace must not be empty.");
    }
    let metric_names = split_list(&a.metric_names);
    if metric_names.is_empty() {
        anyhow::bail!("--metric-names must list at least one metric name.");
    }
    let start = chrono::DateTime::parse_from_rfc3339(a.start.trim()).map_err(|_| {
        anyhow::anyhow!("--start must be an ISO 8601 date-time, e.g. 2026-08-01T00:00:00Z.")
    })?;
    let end = chrono::DateTime::parse_from_rfc3339(a.end.trim()).map_err(|_| {
        anyhow::anyhow!("--end must be an ISO 8601 date-time, e.g. 2026-09-01T00:00:00Z.")
    })?;
    if start >= end {
        anyhow::bail!("--start must be before --end.");
    }
    let integration_group_ids = id_list(&a.integration_group_ids, "--integration-group-ids")?;
    if integration_group_ids
        .as_ref()
        .is_some_and(|v| v.len() > 100)
    {
        anyhow::bail!("--integration-group-ids accepts at most 100 IDs.");
    }
    Ok(QueryMetricsRequest {
        metric: MetricSelector {
            namespace,
            metric_names,
        },
        period: MetricPeriod {
            interval: a.interval,
            start: a.start.trim().to_string(),
            end: a.end.trim().to_string(),
            stat: a.stat.clone(),
        },
        integration_ids: id_list(&a.integration_ids, "--integration-ids")?,
        integration_group_ids,
        asset_group_ids: id_list(&a.asset_group_ids, "--asset-group-ids")?,
        environment_ids: id_list(&a.environment_ids, "--environment-ids")?,
    })
}

pub async fn run(args: &MetricsArgs, config: &Config) -> anyhow::Result<()> {
    match &args.command {
        MetricsCommands::Query(a) => {
            let body = build_request(a)?;
            let client = PlerionClient::new(config)?;
            let resp = query_metrics(&client, &body).await?;
            let tabular = matches!(config.output, OutputFormat::Table | OutputFormat::Text);
            if tabular && config.query.is_none() {
                // Metric names are dynamic, so tables get one row per value.
                let rows: Vec<MetricValueRow> = resp.data.iter().flat_map(|p| p.rows()).collect();
                output::render_list(&rows, config.output, None, config.no_color)?;
            } else {
                let data = serde_json::to_value(&resp.data)?;
                output::render_json_value(&data, config.output, config.query.as_deref())?;
            }
        }
    }
    Ok(())
}
