use crate::api::client::PlerionClient;
use crate::api::models::audit_logs::AuditLogsResponse;
use crate::error::PlerionError;

pub async fn list_audit_logs(
    client: &PlerionClient,
    start: Option<&str>,
    end: Option<&str>,
    user_id: Option<&str>,
    operation: Option<&str>,
    per_page: Option<u32>,
    cursor: Option<&str>,
) -> Result<AuditLogsResponse, PlerionError> {
    let mut req = client.get("/v1/tenant/audit-logs");
    if let Some(v) = start { req = req.query(&[("operationTimeStart", v)]); }
    if let Some(v) = end { req = req.query(&[("operationTimeEnd", v)]); }
    if let Some(v) = user_id { req = req.query(&[("userId", v)]); }
    if let Some(v) = operation { req = req.query(&[("operation", v)]); }
    if let Some(v) = per_page { req = req.query(&[("perPage", v)]); }
    if let Some(v) = cursor { req = req.query(&[("cursor", v)]); }
    client.execute(req).await
}
