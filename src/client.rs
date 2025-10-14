use crate::model::CreateOrderRequest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub const API_URL_PROD: &str = "https://api.hyros.com/v1/api/v1.0";

pub struct HyrosApiClient {
    api_url: String,
    api_key: HeaderValue,
    client: reqwest::Client,
}

impl HyrosApiClient {
    pub fn new(api_key: &str, api_url: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into(),
            client: reqwest::Client::new(),
            api_key: HeaderValue::from_str(api_key).unwrap(),
        }
    }

    pub async fn create_order<'a>(&self, request: &CreateOrderRequest<'a>) -> Result<(), String> {
        let url = format!("{}/orders", self.api_url);
        let request_json = serde_json::to_string(request).unwrap();
        let res = self
            .client
            .post(&url)
            .headers(self.build_headers())
            .body(request_json)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status = res.status();
        let body = res.text().await.unwrap_or_default();

        if status.is_success() && body.contains("OK") {
            Ok(())
        } else {
            Err(format!("HTTP {}: {}", status, body))
        }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(HeaderName::from_static("API-Key"), self.api_key.clone());

        headers
    }
}
