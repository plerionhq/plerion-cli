use crate::config::Config;
use crate::error::PlerionError;
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct PlerionClient {
    pub(crate) inner: Client,
    pub base_url: String,
    api_key: String,
}

impl PlerionClient {
    pub fn new(config: &Config) -> Result<Self, PlerionError> {
        let inner = Client::builder()
            .user_agent(format!("plerion-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self {
            inner,
            base_url: config.base_url(),
            api_key: config.api_key.clone(),
        })
    }

    /// Create a new client pointing at a custom base URL (used in tests).
    #[allow(dead_code)]
    pub fn with_base_url(base_url: &str, api_key: &str) -> Result<Self, PlerionError> {
        let inner = Client::builder()
            .user_agent(format!("plerion-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self {
            inner,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        })
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.inner
            .get(format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.inner
            .post(format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
    }

    pub fn patch(&self, path: &str) -> RequestBuilder {
        self.inner
            .patch(format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
    }

    pub fn delete(&self, path: &str) -> RequestBuilder {
        self.inner
            .delete(format!("{}{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
    }

    /// Execute a request and deserialize the JSON response.
    pub async fn execute<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<T, PlerionError> {
        let resp = req.send().await?;
        handle_response(resp).await
    }

    /// Upload a zip file for IaC scanning (needs application/zip content-type).
    pub fn upload_iac(&self, artifact_name: &str, zip_bytes: bytes::Bytes) -> RequestBuilder {
        self.inner
            .post(format!("{}/v1/tenant/shiftleft/iac/scan", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/zip")
            .query(&[("artifactName", artifact_name)])
            .body(zip_bytes)
    }

    /// Execute a request and return raw bytes (e.g. for downloads).
    pub async fn execute_bytes(&self, req: RequestBuilder) -> Result<bytes::Bytes, PlerionError> {
        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(PlerionError::ApiError { status, message });
        }
        Ok(resp.bytes().await?)
    }
}

async fn handle_response<T: DeserializeOwned>(resp: Response) -> Result<T, PlerionError> {
    let status = resp.status();
    if status.is_success() {
        let body = resp.text().await?;
        serde_json::from_str(&body)
            .map_err(|e| PlerionError::ParseError(format!("{e}: {body}")))
    } else {
        let code = status.as_u16();
        let message = resp.text().await.unwrap_or_default();
        Err(PlerionError::ApiError { status: code, message })
    }
}
