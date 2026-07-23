use crate::onboard::aws_api::AwsApi;
use crate::onboard::{OnboardError, OnboardOptions};
use std::time::Duration;

/// Poll the stack until CREATE_COMPLETE, a terminal failure, or timeout.
/// Returns the final status string on success.
pub async fn wait_for_stack(
    aws: &dyn AwsApi,
    stack_id: &str,
    opts: &OnboardOptions,
) -> Result<String, OnboardError> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(opts.wait_timeout_secs);
    let mut last_status = String::new();
    loop {
        // Transient DescribeStacks failures (throttling, credential refresh)
        // shouldn't kill a deploy that's still running — keep polling.
        let status = match aws.find_stack(stack_id).await {
            Ok(Some(s)) => s,
            Ok(None) => {
                return Err(OnboardError::DeployFailed(
                    "stack disappeared while waiting".to_string(),
                ))
            }
            Err(_) => {
                if tokio::time::Instant::now() >= deadline {
                    return Err(OnboardError::DeployFailed(
                        "timed out polling stack status".to_string(),
                    ));
                }
                tokio::time::sleep(Duration::from_secs(opts.poll_interval_secs)).await;
                continue;
            }
        };

        if status != last_status {
            eprintln!("      {status}");
            last_status = status.clone();
        }

        match status.as_str() {
            "CREATE_COMPLETE" => return Ok(status),
            "CREATE_IN_PROGRESS" => {}
            _ => {
                // Terminal failure: surface the actual resource failures.
                let events = aws.stack_failure_events(stack_id).await.unwrap_or_default();
                let mut lines = Vec::new();
                let mut token_hint = false;
                for e in &events {
                    if e.logical_id == "PlerionAPICall" {
                        token_hint = true;
                    }
                    lines.push(format!(
                        "      {}: {} ({})",
                        e.logical_id,
                        e.status,
                        e.reason.as_deref().unwrap_or("no reason")
                    ));
                }
                let mut msg = format!("stack entered {status}");
                if !lines.is_empty() {
                    msg = format!("{msg}\n{}", lines.join("\n"));
                }
                if token_hint {
                    msg = format!(
                        "{msg}\n      hint: a PlerionAPICall failure usually means the \
                         registration token expired or was rejected — delete the rolled-back \
                         stack and re-run (a fresh token is generated every run)"
                    );
                }
                return Err(OnboardError::DeployFailed(msg));
            }
        }

        if tokio::time::Instant::now() >= deadline {
            return Err(OnboardError::DeployFailed(format!(
                "timed out after {}s waiting for stack completion (status: {status})",
                opts.wait_timeout_secs
            )));
        }
        tokio::time::sleep(Duration::from_secs(opts.poll_interval_secs)).await;
    }
}
