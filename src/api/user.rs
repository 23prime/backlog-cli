use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    use httpmock::prelude::*;
    use serde_json::json;

    fn user_json() -> serde_json::Value {
        json!({
            "id": 123,
            "userId": "john",
            "name": "John Doe",
            "mailAddress": "john@example.com",
            "roleType": 1
        })
    }

    #[test]
    fn get_myself_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/myself");
            then.status(200).json_body(user_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let user = client.get_myself().unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.user_id, "john");
        assert_eq!(user.name, "John Doe");
    }

    #[test]
    fn get_myself_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/myself");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_myself().unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

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
