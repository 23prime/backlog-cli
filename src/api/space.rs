use anyhow::Result;
use serde::Deserialize;

use super::BacklogClient;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub space_key: String,
    pub name: String,
    pub _owner_id: u64,
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
    use serde_json::json;

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
