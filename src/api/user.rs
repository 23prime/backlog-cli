use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::BacklogClient;
use crate::api::activity::Activity;
use crate::api::issue::Issue;

fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value, ctx: &str) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to deserialize {ctx}: {e}\nRaw JSON:\n{}",
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
        )
    })
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyViewedIssue {
    pub issue: Issue,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BacklogClient {
    pub fn get_myself(&self) -> Result<User> {
        let value = self.get("/users/myself")?;
        deserialize(value, "user response")
    }

    pub fn get_users(&self) -> Result<Vec<User>> {
        let value = self.get("/users")?;
        deserialize(value, "users response")
    }

    pub fn get_user(&self, user_id: u64) -> Result<User> {
        let value = self.get(&format!("/users/{user_id}"))?;
        deserialize(value, "user response")
    }

    pub fn get_user_activities(&self, user_id: u64) -> Result<Vec<Activity>> {
        let value = self.get(&format!("/users/{user_id}/activities"))?;
        deserialize(value, "user activities response")
    }

    pub fn get_recently_viewed_issues(&self) -> Result<Vec<RecentlyViewedIssue>> {
        let value = self.get("/users/myself/recentlyViewedIssues")?;
        deserialize(value, "recently viewed issues response")
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
    fn get_user_activities_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/123/activities");
            then.status(200).json_body(json!([{
                "id": 1,
                "type": 2,
                "content": {},
                "createdUser": {"id": 123, "userId": "john", "name": "John Doe"},
                "created": "2024-01-01T00:00:00Z"
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let activities = client.get_user_activities(123).unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].id, 1);
    }

    #[test]
    fn get_user_activities_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/123/activities");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No user"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_user_activities(123).unwrap_err();
        assert!(err.to_string().contains("No user"));
    }

    #[test]
    fn get_recently_viewed_issues_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/myself/recentlyViewedIssues");
            then.status(200).json_body(json!([{
                "issue": {
                    "id": 1,
                    "projectId": 1,
                    "issueKey": "BLG-1",
                    "keyId": 1,
                    "summary": "first issue",
                    "description": "",
                    "priority": {"id": 3, "name": "Normal"},
                    "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
                    "issueType": {"id": 2, "projectId": 1, "name": "Task", "color": "#7ea800", "displayOrder": 0},
                    "assignee": null,
                    "category": [],
                    "versions": [],
                    "milestone": [],
                    "created": "2024-01-01T00:00:00Z",
                    "updated": "2024-06-01T00:00:00Z",
                    "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1},
                    "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1}
                },
                "updated": "2024-06-01T00:00:00Z"
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let items = client.get_recently_viewed_issues().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].issue.issue_key, "BLG-1");
        assert_eq!(items[0].updated, "2024-06-01T00:00:00Z");
    }

    #[test]
    fn get_recently_viewed_issues_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/myself/recentlyViewedIssues");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_recently_viewed_issues().unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
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
