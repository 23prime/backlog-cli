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
