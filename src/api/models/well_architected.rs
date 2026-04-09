use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WellArchitectedFramework {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub posture: Option<f64>,
    pub passed_findings: Option<u32>,
    pub total_findings: Option<u32>,
    pub is_custom: Option<bool>,
    pub providers: Option<Vec<String>>,
    pub description: Option<String>,
    pub release_date: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WellArchitectedData {
    pub frameworks: Vec<WellArchitectedFramework>,
    pub total_posture: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WellArchitectedResponse {
    pub data: WellArchitectedData,
}

impl TableRenderable for WellArchitectedFramework {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "NAME",
            "VERSION",
            "POSTURE %",
            "PASSED",
            "TOTAL",
            "CUSTOM",
            "PROVIDERS",
            "DESCRIPTION",
            "RELEASE DATE",
            "LINK",
        ]
    }

    fn row(&self) -> Vec<String> {
        let providers = self.providers.as_ref()
            .map(|p| p.join(", "))
            .unwrap_or_default();
        vec![
            self.id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.version.clone().unwrap_or_default(),
            self.posture.map(|p| format!("{p:.1}%")).unwrap_or_default(),
            self.passed_findings.map(|n| n.to_string()).unwrap_or_default(),
            self.total_findings.map(|n| n.to_string()).unwrap_or_default(),
            self.is_custom.map(|b| if b { "yes" } else { "no" }).unwrap_or_default().to_string(),
            providers,
            self.description.clone().unwrap_or_default(),
            self.release_date.clone().unwrap_or_default(),
            self.link.clone().unwrap_or_default(),
        ]
    }
}
