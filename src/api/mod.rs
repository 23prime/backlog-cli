use anyhow::{Context, Result};
use reqwest::blocking::Client;

pub mod space;
pub mod user;

use space::Space;
use user::User;

pub trait BacklogApi {
    fn get_space(&self) -> Result<Space>;
    fn get_myself(&self) -> Result<User>;
}

impl BacklogApi for BacklogClient {
    fn get_space(&self) -> Result<Space> {
        self.get_space()
    }

    fn get_myself(&self) -> Result<User> {
        self.get_myself()
    }
}

pub struct BacklogClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl BacklogClient {
    pub fn from_config() -> Result<Self> {
        let cfg = crate::config::load()?;
        let auth = cfg
            .auth
            .context("Not logged in. Run `bl auth login` first.")?;

        let (api_key, _) = crate::secret::get(&auth.space_key)?;

        let client = Client::builder()
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            base_url: format!("https://{}.backlog.com/api/v2", auth.space_key),
            api_key,
        })
    }

    pub fn get(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .query(&[("apiKey", &self.api_key)])
            .send()
            .with_context(|| format!("Failed to GET {}", url))?;

        let status = response.status();
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }

        Ok(body)
    }
}

fn extract_error_message(body: &serde_json::Value) -> &str {
    body.get("errors")
        .and_then(|e| e.get(0))
        .and_then(|e| e.get("message"))
        .and_then(|m| m.as_str())
        .unwrap_or("Unknown error")
}

impl BacklogClient {
    pub(crate) fn new_with(base_url: &str, api_key: &str) -> Result<Self> {
        let client = Client::builder()
            .build()
            .context("Failed to build HTTP client")?;
        Ok(Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-api-key";

    #[test]
    fn get_returns_body_on_success() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/space")
                .query_param("apiKey", TEST_KEY);
            then.status(200)
                .json_body(json!({"spaceKey": "mycompany", "name": "My Company"}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let body = client.get("/space").unwrap();
        assert_eq!(body["spaceKey"], "mycompany");
    }

    #[test]
    fn get_returns_error_with_api_message_on_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get("/space").unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }

    #[test]
    fn get_returns_error_with_fallback_message_on_unknown_error() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(500).json_body(json!({}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get("/space").unwrap_err();
        assert!(err.to_string().contains("Unknown error"));
    }

    #[test]
    fn extract_error_message_from_errors_array() {
        let body = json!({"errors": [{"message": "Authentication failure"}]});
        assert_eq!(extract_error_message(&body), "Authentication failure");
    }

    #[test]
    fn extract_error_message_fallback_when_missing() {
        let body = json!({});
        assert_eq!(extract_error_message(&body), "Unknown error");
    }

    #[test]
    fn extract_error_message_fallback_when_empty_errors() {
        let body = json!({"errors": []});
        assert_eq!(extract_error_message(&body), "Unknown error");
    }
}
