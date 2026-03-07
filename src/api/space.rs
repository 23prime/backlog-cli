use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub space_key: String,
    pub name: String,
    pub owner_id: u64,
    pub lang: String,
    pub timezone: String,
    pub text_formatting_rule: String,
    pub created: String,
    pub updated: String,
}

impl BacklogClient {
    pub fn get_space(&self) -> Result<Space> {
        let value = self.get("/space")?;
        let space: Space = serde_json::from_value(value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize space response: {}", e))?;
        Ok(space)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    fn space_json() -> serde_json::Value {
        json!({
            "spaceKey": "mycompany",
            "name": "My Company",
            "ownerId": 1,
            "lang": "ja",
            "timezone": "Asia/Tokyo",
            "textFormattingRule": "markdown",
            "created": "2020-01-01T00:00:00Z",
            "updated": "2024-06-01T00:00:00Z"
        })
    }

    #[test]
    fn get_space_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(200).json_body(space_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let space = client.get_space().unwrap();
        assert_eq!(space.space_key, "mycompany");
        assert_eq!(space.name, "My Company");
        assert_eq!(space.lang, "ja");
        assert_eq!(space.timezone, "Asia/Tokyo");
        assert_eq!(space.text_formatting_rule, "markdown");
    }

    #[test]
    fn get_space_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_space().unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }

    #[test]
    fn deserialize_space() {
        let v = json!({
            "spaceKey": "mycompany",
            "name": "My Company",
            "ownerId": 1,
            "lang": "ja",
            "timezone": "Asia/Tokyo",
            "textFormattingRule": "markdown",
            "created": "2020-01-01T00:00:00Z",
            "updated": "2024-06-01T00:00:00Z"
        });
        let space: Space = serde_json::from_value(v).unwrap();
        assert_eq!(space.space_key, "mycompany");
        assert_eq!(space.name, "My Company");
        assert_eq!(space.lang, "ja");
        assert_eq!(space.timezone, "Asia/Tokyo");
        assert_eq!(space.text_formatting_rule, "markdown");
    }

    #[test]
    fn deserialize_space_fails_on_missing_required_field() {
        let v = json!({"spaceKey": "mycompany"});
        assert!(serde_json::from_value::<Space>(v).is_err());
    }
}
