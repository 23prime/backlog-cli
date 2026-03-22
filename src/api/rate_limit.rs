use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitInfo {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit: RateLimitInfo,
}

impl BacklogClient {
    pub fn get_rate_limit(&self) -> Result<RateLimit> {
        let value = self.get("/rateLimit")?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    fn rate_limit_json() -> serde_json::Value {
        json!({
            "rateLimit": {
                "limit": 600,
                "remaining": 599,
                "reset": 1698230400
            }
        })
    }

    #[test]
    fn get_rate_limit_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/rateLimit");
            then.status(200).json_body(rate_limit_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let rl = client.get_rate_limit().unwrap();
        assert_eq!(rl.rate_limit.limit, 600);
        assert_eq!(rl.rate_limit.remaining, 599);
        assert_eq!(rl.rate_limit.reset, 1698230400);
    }

    #[test]
    fn get_rate_limit_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/rateLimit");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_rate_limit().unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }
}
