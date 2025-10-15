use crate::model::CreateOrderRequest;
use flurl::{body::FlUrlBody, FlUrl};

pub const API_URL_PROD: &str = "https://api.hyros.com/v1/api/v1.0";

pub struct HyrosApiClient {
    api_url: String,
    api_key: String,
}

impl HyrosApiClient {
    pub fn new(api_key: &str, api_url: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into(),
            api_key: api_key.into(),
        }
    }

    pub async fn create_order<'a>(&self, request: &CreateOrderRequest<'a>) -> Result<(), String> {
        let url = format!("{}/orders", self.api_url);
        let flurl = flurl::FlUrl::new(&url);
        let flurl = self.add_headers(flurl);
        let body = FlUrlBody::as_json(request);
        let result = flurl.post(body).await;

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {} {:?}",
                url,
                result.unwrap_err()
            )
            .into());
        };

        let result = resp.receive_body().await;

        let Ok(body_bytes) = result else {
            return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
        };

        let body_str = String::from_utf8(body_bytes).unwrap();
        if body_str.contains("OK") {
            Ok(())
        } else {
            Err(format!("HTTP {}: {}", url, body_str))
        }
    }

    fn add_headers(&self, flurl: FlUrl) -> FlUrl {
        flurl
            .with_header("Content-Type", "application/json")
            .with_header("API-Key", self.api_key.clone())
    }
}
