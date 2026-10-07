use serde::{Deserialize, Serialize};

use crate::api::models::findings::PaginationMeta;
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfileIntegration {
    pub integration_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub profile_id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_default: bool,
    #[serde(default)]
    pub integrations: Vec<ProfileIntegration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfilesResponse {
    pub data: Vec<Profile>,
    #[serde(default)]
    pub meta: Option<PaginationMeta>,
}

impl TableRenderable for Profile {
    fn headers() -> Vec<&'static str> {
        vec![
            "PROFILE ID",
            "NAME",
            "DEFAULT",
            "INTEGRATIONS",
            "DESCRIPTION",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.profile_id.clone(),
            self.name.clone(),
            self.is_default.to_string(),
            self.integrations.len().to_string(),
            self.description.clone().unwrap_or_default(),
        ]
    }
}

/// One stored exemption, kept as raw JSON so it can be sent back unchanged.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct StoredDetectionExemption(pub serde_json::Value);

impl StoredDetectionExemption {
    fn str_field(&self, key: &str) -> String {
        self.0
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    }

    fn tag_part(&self, key: &str) -> (String, String) {
        let part = self.0.get(key);
        let get = |k: &str| {
            part.and_then(|p| p.get(k))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        };
        (get("option"), get("value"))
    }

    /// What the exemption matches, summarised as one line.
    pub fn match_summary(&self) -> String {
        match self.str_field("type").as_str() {
            "NAME_EXEMPTION" | "PRINCIPAL_EXEMPTION" => {
                format!(
                    "{} {:?}",
                    self.str_field("condition"),
                    self.str_field("value")
                )
            }
            "TAG_EXEMPTION" => {
                let (_, key) = self.tag_part("tagKey");
                let (option, value) = self.tag_part("tagValue");
                if option == "any" {
                    format!("tag {key:?} with any value")
                } else {
                    format!("tag {key:?} value {option} {value:?}")
                }
            }
            "REGION_EXEMPTION" => self
                .0
                .get("regions")
                .and_then(|v| v.as_array())
                .map(|r| {
                    r.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default(),
            "ROUTE_EXEMPTION" => {
                format!("{} {}", self.str_field("apiId"), self.str_field("routeKey"))
            }
            _ => String::new(),
        }
    }
}

impl TableRenderable for StoredDetectionExemption {
    fn headers() -> Vec<&'static str> {
        vec!["TYPE", "REASON", "MATCH", "CREATED BY", "CREATED AT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.str_field("type"),
            self.str_field("reason"),
            self.match_summary(),
            self.str_field("createdBy"),
            self.str_field("createdAt"),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DetectionExemptionSet {
    pub profile_id: String,
    pub detection_id: String,
    pub provider: Option<String>,
    pub supports_exemptions: bool,
    #[serde(default)]
    pub supported_exemption_types: Vec<String>,
    #[serde(default)]
    pub exemptions: Vec<StoredDetectionExemption>,
    /// Opaque; send back unchanged as `If-Match`. Null when never configured.
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionExemptionSetResponse {
    pub data: DetectionExemptionSet,
}

impl TableRenderable for DetectionExemptionSet {
    fn headers() -> Vec<&'static str> {
        vec![
            "PROFILE ID",
            "DETECTION ID",
            "PROVIDER",
            "SUPPORTS EXEMPTIONS",
            "SUPPORTED TYPES",
            "EXEMPTIONS",
            "VERSION",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.profile_id.clone(),
            self.detection_id.clone(),
            self.provider.clone().unwrap_or_default(),
            self.supports_exemptions.to_string(),
            self.supported_exemption_types.join(", "),
            self.exemptions.len().to_string(),
            self.version.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplaceDetectionExemptionsRequest {
    pub exemptions: Vec<serde_json::Value>,
}
