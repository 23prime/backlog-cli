use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::BacklogClient;
use super::deserialize;
use crate::api::activity::Activity;
use crate::api::issue::Issue;
use crate::api::project::Project;
use crate::api::wiki::WikiListItem;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyViewedProject {
    pub project: Project,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyViewedWiki {
    pub page: WikiListItem,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Star {
    pub id: u64,
    pub comment: Option<String>,
    pub url: String,
    pub title: String,
    pub presenter: User,
    pub created: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarCount {
    pub count: u64,
}

impl BacklogClient {
    pub fn get_myself(&self) -> Result<User> {
        let value = self.get("/users/myself")?;
        deserialize(value)
    }

    pub fn get_users(&self) -> Result<Vec<User>> {
        let value = self.get("/users")?;
        deserialize(value)
    }

    pub fn get_user(&self, user_id: u64) -> Result<User> {
        let value = self.get(&format!("/users/{user_id}"))?;
        deserialize(value)
    }

    pub fn get_user_activities(
        &self,
        user_id: u64,
        params: &[(String, String)],
    ) -> Result<Vec<Activity>> {
        let value = self.get_with_query(&format!("/users/{user_id}/activities"), params)?;
        deserialize(value)
    }

    pub fn get_recently_viewed_issues(
        &self,
        params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedIssue>> {
        let value = self.get_with_query("/users/myself/recentlyViewedIssues", params)?;
        deserialize(value)
    }

    pub fn add_user(&self, params: &[(String, String)]) -> Result<User> {
        let value = self.post_form("/users", params)?;
        deserialize(value)
    }

    pub fn update_user(&self, user_id: u64, params: &[(String, String)]) -> Result<User> {
        let value = self.patch_form(&format!("/users/{user_id}"), params)?;
        deserialize(value)
    }

    pub fn delete_user(&self, user_id: u64) -> Result<User> {
        let value = self.delete_req(&format!("/users/{user_id}"))?;
        deserialize(value)
    }

    pub fn get_recently_viewed_projects(
        &self,
        params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedProject>> {
        let value = self.get_with_query("/users/myself/recentlyViewedProjects", params)?;
        deserialize(value)
    }

    pub fn get_recently_viewed_wikis(
        &self,
        params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedWiki>> {
        let value = self.get_with_query("/users/myself/recentlyViewedWikis", params)?;
        deserialize(value)
    }

    pub fn get_user_stars(&self, user_id: u64, params: &[(String, String)]) -> Result<Vec<Star>> {
        let value = self.get_with_query(&format!("/users/{user_id}/stars"), params)?;
        deserialize(value)
    }

    pub fn count_user_stars(&self, user_id: u64, params: &[(String, String)]) -> Result<StarCount> {
        let value = self.get_with_query(&format!("/users/{user_id}/stars/count"), params)?;
        deserialize(value)
    }

    pub fn add_star(&self, params: &[(String, String)]) -> Result<()> {
        self.post_form("/stars", params)?;
        Ok(())
    }

    pub fn delete_star(&self, star_id: u64) -> Result<()> {
        self.delete_req(&format!("/stars/{star_id}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-key";

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
        let activities = client.get_user_activities(123, &[]).unwrap();
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
        let err = client.get_user_activities(123, &[]).unwrap_err();
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
        let items = client.get_recently_viewed_issues(&[]).unwrap();
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
        let err = client.get_recently_viewed_issues(&[]).unwrap_err();
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

    #[test]
    fn add_user_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/users");
            then.status(201).json_body(user_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let user = client
            .add_user(&[
                ("userId".to_string(), "john".to_string()),
                ("password".to_string(), "secret".to_string()),
                ("name".to_string(), "John Doe".to_string()),
                ("mailAddress".to_string(), "john@example.com".to_string()),
                ("roleType".to_string(), "1".to_string()),
            ])
            .unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.name, "John Doe");
    }

    #[test]
    fn add_user_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/users");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.add_user(&[]).unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn update_user_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH).path("/users/123");
            then.status(200).json_body(user_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let user = client
            .update_user(123, &[("name".to_string(), "New Name".to_string())])
            .unwrap();
        assert_eq!(user.id, 123);
    }

    #[test]
    fn update_user_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH).path("/users/999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No user"}]}));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.update_user(999, &[]).unwrap_err();
        assert!(err.to_string().contains("No user"));
    }

    #[test]
    fn delete_user_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/users/123");
            then.status(200).json_body(user_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let user = client.delete_user(123).unwrap();
        assert_eq!(user.id, 123);
    }

    #[test]
    fn delete_user_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/users/999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No user"}]}));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.delete_user(999).unwrap_err();
        assert!(err.to_string().contains("No user"));
    }

    #[test]
    fn get_recently_viewed_projects_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/users/myself/recentlyViewedProjects");
            then.status(200).json_body(json!([{
                "project": {
                    "id": 1, "projectKey": "TEST", "name": "Test Project",
                    "chartEnabled": false, "subtaskingEnabled": false,
                    "projectLeaderCanEditProjectLeader": false,
                    "textFormattingRule": "markdown", "archived": false
                },
                "updated": "2024-06-01T00:00:00Z"
            }]));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let items = client.get_recently_viewed_projects(&[]).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].project.project_key, "TEST");
    }

    #[test]
    fn get_recently_viewed_wikis_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/myself/recentlyViewedWikis");
            then.status(200).json_body(json!([{
                "page": {
                    "id": 1, "projectId": 1, "name": "Home",
                    "tags": [],
                    "createdUser": {"id": 1, "userId": "admin", "name": "Admin", "roleType": 1},
                    "created": "2024-01-01T00:00:00Z",
                    "updatedUser": {"id": 1, "userId": "admin", "name": "Admin", "roleType": 1},
                    "updated": "2024-06-01T00:00:00Z"
                },
                "updated": "2024-06-01T00:00:00Z"
            }]));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let items = client.get_recently_viewed_wikis(&[]).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].page.name, "Home");
    }

    #[test]
    fn get_user_stars_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/123/stars");
            then.status(200).json_body(json!([{
                "id": 1,
                "comment": null,
                "url": "https://example.com/issue/1",
                "title": "Issue title",
                "presenter": {"id": 2, "userId": "alice", "name": "Alice", "roleType": 1},
                "created": "2024-01-01T00:00:00Z"
            }]));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let stars = client.get_user_stars(123, &[]).unwrap();
        assert_eq!(stars.len(), 1);
        assert_eq!(stars[0].title, "Issue title");
        assert_eq!(stars[0].comment, None);
    }

    #[test]
    fn count_user_stars_returns_count() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/123/stars/count");
            then.status(200).json_body(json!({"count": 42}));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let result = client.count_user_stars(123, &[]).unwrap();
        assert_eq!(result.count, 42);
    }
}
