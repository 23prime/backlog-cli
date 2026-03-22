use std::collections::BTreeMap;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceAttachmentUser {
    pub id: u64,
    pub user_id: Option<String>,
    pub name: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceAttachment {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub created_user: SpaceAttachmentUser,
    pub created: String,
}

impl BacklogClient {
    pub fn get_space(&self) -> Result<Space> {
        let value = self.get("/space")?;
        deserialize(value)
    }

    pub fn download_space_image(&self) -> Result<(Vec<u8>, String)> {
        self.download("/space/image")
    }

    pub fn upload_space_attachment(&self, file_path: &std::path::Path) -> Result<SpaceAttachment> {
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("attachment")
            .to_string();
        let bytes = std::fs::read(file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;
        let part = reqwest::blocking::multipart::Part::bytes(bytes).file_name(filename);
        let form = reqwest::blocking::multipart::Form::new().part("file", part);
        let value = self.post_multipart("/space/attachment", form)?;
        deserialize(value)
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
