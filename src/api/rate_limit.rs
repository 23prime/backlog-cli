use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitCategory {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitInfo {
    pub read: RateLimitCategory,
    pub update: RateLimitCategory,
    pub search: RateLimitCategory,
    pub icon: Option<RateLimitCategory>,
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
                "read":   {"limit": 600, "remaining": 591, "reset": 1774268714},
                "update": {"limit": 150, "remaining": 150, "reset": 1774268655},
                "search": {"limit": 150, "remaining": 150, "reset": 1774268655},
                "icon":   {"limit":  60, "remaining":  60, "reset": 1774268655}
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
        assert_eq!(rl.rate_limit.read.limit, 600);
        assert_eq!(rl.rate_limit.read.remaining, 591);
        assert_eq!(rl.rate_limit.update.limit, 150);
        assert_eq!(rl.rate_limit.search.limit, 150);
        assert_eq!(rl.rate_limit.icon.unwrap().limit, 60);
    }

    #[test]
    fn get_rate_limit_without_icon() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/rateLimit");
            then.status(200).json_body(json!({
                "rateLimit": {
                    "read":   {"limit": 600, "remaining": 600, "reset": 1774268714},
                    "update": {"limit": 150, "remaining": 150, "reset": 1774268655},
                    "search": {"limit": 150, "remaining": 150, "reset": 1774268655}
                }
            }));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let rl = client.get_rate_limit().unwrap();
        assert!(rl.rate_limit.icon.is_none());
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
