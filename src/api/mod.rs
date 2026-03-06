use anyhow::{Context, Result};
use reqwest::blocking::Client;

pub mod space;
pub mod user;

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

        let api_key = crate::secret::get(&auth.space_key)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
