use anyhow::Result;
use serde::Deserialize;

use super::BacklogClient;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct User {
    pub id: u64,
    pub user_id: String,
    pub name: String,
    pub mail_address: String,
    pub role_type: u8,
}

impl BacklogClient {
    pub fn get_myself(&self) -> Result<User> {
        let value = self.get("/users/myself")?;
        serde_json::from_value(value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize user response: {}", e))
    }
}
