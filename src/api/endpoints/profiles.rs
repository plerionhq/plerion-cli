use crate::api::client::PlerionClient;
use crate::api::models::profiles::{
    DetectionExemptionSetResponse, ProfilesResponse, ReplaceDetectionExemptionsRequest,
};
use crate::error::PlerionError;

use super::path::segment;

fn exemptions_path(profile_id: &str, detection_id: &str) -> Result<String, PlerionError> {
    Ok(format!(
        "/v1/tenant/profiles/{}/detection-settings/{}/exemptions",
        segment(profile_id, "profile ID")?,
        segment(detection_id, "detection ID")?
    ))
}

pub async fn list_profiles(client: &PlerionClient) -> Result<ProfilesResponse, PlerionError> {
    client.execute(client.get("/v1/tenant/profiles")).await
}

pub async fn get_detection_exemptions(
    client: &PlerionClient,
    profile_id: &str,
    detection_id: &str,
) -> Result<DetectionExemptionSetResponse, PlerionError> {
    let path = exemptions_path(profile_id, detection_id)?;
    client.execute(client.get(&path)).await
}

/// Replaces the whole set. `if_match` is the `version` from the last read.
pub async fn replace_detection_exemptions(
    client: &PlerionClient,
    profile_id: &str,
    detection_id: &str,
    if_match: Option<&str>,
    body: &ReplaceDetectionExemptionsRequest,
) -> Result<DetectionExemptionSetResponse, PlerionError> {
    let path = exemptions_path(profile_id, detection_id)?;
    let mut req = client.put(&path).json(body);
    if let Some(v) = if_match {
        req = req.header("If-Match", v);
    }
    client.execute(req).await
}
