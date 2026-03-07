use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueUser {
    pub id: u64,
    pub user_id: Option<String>,
    pub name: String,
    pub role_type: u8,
    pub lang: Option<String>,
    pub mail_address: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub color: String,
    pub display_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuePriority {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueStatus {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub color: String,
    pub display_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueResolution {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: u64,
    pub project_id: u64,
    pub issue_key: String,
    pub key_id: u64,
    pub issue_type: IssueType,
    pub summary: String,
    pub description: Option<String>,
    pub resolutions: Option<IssueResolution>,
    pub priority: IssuePriority,
    pub status: IssueStatus,
    pub assignee: Option<IssueUser>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub parent_issue_id: Option<u64>,
    pub created_user: IssueUser,
    pub created: String,
    pub updated_user: IssueUser,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCount {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueComment {
    pub id: u64,
    pub content: Option<String>,
    pub created_user: IssueUser,
    pub created: String,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueAttachment {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub created_user: IssueUser,
    pub created: String,
}

fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to deserialize response: {}\nRaw JSON:\n{}",
            e,
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
        )
    })
}

impl BacklogClient {
    pub fn get_issues(&self, params: &[(String, String)]) -> Result<Vec<Issue>> {
        let value = self.get_with_query("/issues", params)?;
        deserialize(value)
    }

    pub fn count_issues(&self, params: &[(String, String)]) -> Result<IssueCount> {
        let value = self.get_with_query("/issues/count", params)?;
        deserialize(value)
    }

    pub fn get_issue(&self, key: &str) -> Result<Issue> {
        let value = self.get(&format!("/issues/{}", key))?;
        deserialize(value)
    }

    pub fn create_issue(&self, params: &[(String, String)]) -> Result<Issue> {
        let value = self.post_form("/issues", params)?;
        deserialize(value)
    }

    pub fn update_issue(&self, key: &str, params: &[(String, String)]) -> Result<Issue> {
        let value = self.patch_form(&format!("/issues/{}", key), params)?;
        deserialize(value)
    }

    pub fn delete_issue(&self, key: &str) -> Result<Issue> {
        let value = self.delete_req(&format!("/issues/{}", key))?;
        deserialize(value)
    }

    pub fn get_issue_comments(&self, key: &str) -> Result<Vec<IssueComment>> {
        let value = self.get(&format!("/issues/{}/comments", key))?;
        deserialize(value)
    }

    pub fn add_issue_comment(
        &self,
        key: &str,
        params: &[(String, String)],
    ) -> Result<IssueComment> {
        let value = self.post_form(&format!("/issues/{}/comments", key), params)?;
        deserialize(value)
    }

    pub fn update_issue_comment(
        &self,
        key: &str,
        comment_id: u64,
        params: &[(String, String)],
    ) -> Result<IssueComment> {
        let value = self.patch_form(&format!("/issues/{}/comments/{}", key, comment_id), params)?;
        deserialize(value)
    }

    pub fn delete_issue_comment(&self, key: &str, comment_id: u64) -> Result<IssueComment> {
        let value = self.delete_req(&format!("/issues/{}/comments/{}", key, comment_id))?;
        deserialize(value)
    }

    pub fn get_issue_attachments(&self, key: &str) -> Result<Vec<IssueAttachment>> {
        let value = self.get(&format!("/issues/{}/attachments", key))?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-api-key";

    fn issue_user_json() -> serde_json::Value {
        json!({
            "id": 1,
            "userId": "john",
            "name": "John Doe",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "john@example.com"
        })
    }

    fn issue_json() -> serde_json::Value {
        json!({
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {
                "id": 1,
                "projectId": 1,
                "name": "Bug",
                "color": "#e30000",
                "displayOrder": 0
            },
            "summary": "Test issue",
            "description": null,
            "resolutions": null,
            "priority": { "id": 2, "name": "Normal" },
            "status": {
                "id": 1,
                "projectId": 1,
                "name": "Open",
                "color": "#ed8077",
                "displayOrder": 1000
            },
            "assignee": null,
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "createdUser": issue_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": issue_user_json(),
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    fn comment_json() -> serde_json::Value {
        json!({
            "id": 1,
            "content": "A comment",
            "createdUser": issue_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    fn attachment_json() -> serde_json::Value {
        json!({
            "id": 1,
            "name": "file.txt",
            "size": 1024,
            "createdUser": issue_user_json(),
            "created": "2024-01-01T00:00:00Z"
        })
    }

    #[test]
    fn get_issues_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/issues")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([issue_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let issues = client.get_issues(&[]).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].issue_key, "TEST-1");
    }

    #[test]
    fn get_issues_returns_error_on_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/issues");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get_issues(&[]).unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }

    #[test]
    fn count_issues_returns_count() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/issues/count")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!({"count": 42}));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let count = client.count_issues(&[]).unwrap();
        assert_eq!(count.count, 42);
    }

    #[test]
    fn get_issue_returns_single() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/issues/TEST-1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(issue_json());
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let issue = client.get_issue("TEST-1").unwrap();
        assert_eq!(issue.summary, "Test issue");
    }

    #[test]
    fn get_issue_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/issues/TEST-999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No issue"}]}));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get_issue("TEST-999").unwrap_err();
        assert!(err.to_string().contains("No issue"));
    }

    #[test]
    fn get_issue_comments_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/issues/TEST-1/comments")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([comment_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let comments = client.get_issue_comments("TEST-1").unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].content.as_deref(), Some("A comment"));
    }

    #[test]
    fn get_issue_attachments_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/issues/TEST-1/attachments")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([attachment_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let attachments = client.get_issue_attachments("TEST-1").unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].name, "file.txt");
    }

    #[test]
    fn issue_with_null_user_id_deserializes() {
        let v = json!({
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {
                "id": 1, "projectId": 1, "name": "Bug",
                "color": "#e30000", "displayOrder": 0
            },
            "summary": "Bot issue",
            "description": null,
            "resolutions": null,
            "priority": { "id": 2, "name": "Normal" },
            "status": {
                "id": 1, "projectId": 1, "name": "Open",
                "color": "#ed8077", "displayOrder": 1000
            },
            "assignee": null,
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "createdUser": {
                "id": 99, "userId": null, "name": "Bot",
                "roleType": 2, "lang": null, "mailAddress": null
            },
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": {
                "id": 99, "userId": null, "name": "Bot",
                "roleType": 2, "lang": null, "mailAddress": null
            },
            "updated": "2024-01-01T00:00:00Z"
        });
        let issue: Issue = serde_json::from_value(v).unwrap();
        assert!(issue.created_user.user_id.is_none());
    }
}
