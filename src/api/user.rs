use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    /// `null` for bot accounts (e.g. automation bots have no userId in Backlog API).
    pub user_id: Option<String>,
    pub name: String,
    /// `null` for bot accounts.
    pub mail_address: Option<String>,
    pub role_type: u8,
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default)]
    pub last_login_time: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BacklogClient {
    pub fn get_myself(&self) -> Result<User> {
        let value = self.get("/users/myself")?;
        serde_json::from_value(value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize user response: {}", e))
    }

    pub fn get_users(&self) -> Result<Vec<User>> {
        let value = self.get("/users")?;
        serde_json::from_value(value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize users response: {}", e))
    }

    pub fn get_user(&self, user_id: u64) -> Result<User> {
        let value = self.get(&format!("/users/{user_id}"))?;
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
            "roleType": 1,
            "lang": "ja",
            "lastLoginTime": "2024-01-01T00:00:00Z"
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
        assert_eq!(user.user_id.as_deref(), Some("john"));
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
    fn get_users_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users");
            then.status(200).json_body(json!([user_json()]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let users = client.get_users().unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, 123);
    }

    #[test]
    fn get_users_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_users().unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn get_user_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/123");
            then.status(200).json_body(user_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let user = client.get_user(123).unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.name, "John Doe");
    }

    #[test]
    fn get_user_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No user"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_user(999).unwrap_err();
        assert!(err.to_string().contains("No user"));
    }

    #[test]
    fn deserialize_user() {
        let v = user_json();
        let user: User = serde_json::from_value(v).unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.user_id.as_deref(), Some("john"));
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.mail_address.as_deref(), Some("john@example.com"));
        assert_eq!(user.role_type, 1);
    }

    #[test]
    fn deserialize_user_with_null_user_id() {
        let v = json!({
            "id": 1,
            "userId": null,
            "name": "Bot",
            "mailAddress": null,
            "roleType": 2
        });
        let user: User = serde_json::from_value(v).unwrap();
        assert_eq!(user.user_id, None);
        assert_eq!(user.mail_address, None);
    }
}
