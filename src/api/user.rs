use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_user() {
        let v = json!({
            "id": 123,
            "userId": "john",
            "name": "John Doe",
            "mailAddress": "john@example.com",
            "roleType": 1
        });
        let user: User = serde_json::from_value(v).unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.user_id, "john");
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.mail_address, "john@example.com");
        assert_eq!(user.role_type, 1);
    }

    #[test]
    fn deserialize_user_fails_on_missing_required_field() {
        let v = json!({"id": 123, "userId": "john"});
        assert!(serde_json::from_value::<User>(v).is_err());
    }
}
