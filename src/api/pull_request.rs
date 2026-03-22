use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    let pretty = serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string());
    serde_json::from_value(value).map_err(|e| {
        anyhow::anyhow!(
            "Failed to deserialize response: {}\nRaw JSON:\n{}",
            e,
            pretty
        )
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrUser {
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
pub struct PullRequestStatus {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrLinkedIssue {
    pub id: u64,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestAttachment {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub created_user: PrUser,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestStar {
    pub id: u64,
    pub comment: Option<String>,
    pub url: String,
    pub title: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrChangeLog {
    pub field: String,
    pub new_value: Option<String>,
    pub original_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestComment {
    pub id: u64,
    pub content: Option<String>,
    pub change_log: Vec<PrChangeLog>,
    pub created_user: PrUser,
    pub created: String,
    pub updated: String,
    pub stars: Vec<PullRequestStar>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestCount {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestCommentCount {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub id: u64,
    pub project_id: u64,
    pub repository_id: u64,
    pub number: u64,
    pub summary: String,
    pub description: String,
    pub base: String,
    pub branch: String,
    pub status: PullRequestStatus,
    pub assignee: Option<PrUser>,
    pub issue: Option<PrLinkedIssue>,
    pub base_commit: Option<String>,
    pub branch_commit: Option<String>,
    pub merge_commit: Option<String>,
    pub close_at: Option<String>,
    pub merge_at: Option<String>,
    pub created_user: PrUser,
    pub created: String,
    pub updated_user: PrUser,
    pub updated: String,
    pub attachments: Vec<PullRequestAttachment>,
    pub stars: Vec<PullRequestStar>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BacklogClient {
    pub fn get_pull_requests(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        params: &[(String, String)],
    ) -> Result<Vec<PullRequest>> {
        let value = self.get_with_query(
            &format!(
                "/projects/{}/git/repositories/{}/pullRequests",
                project_id_or_key, repo_id_or_name
            ),
            params,
        )?;
        deserialize(value)
    }

    pub fn count_pull_requests(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        params: &[(String, String)],
    ) -> Result<PullRequestCount> {
        let value = self.get_with_query(
            &format!(
                "/projects/{}/git/repositories/{}/pullRequests/count",
                project_id_or_key, repo_id_or_name
            ),
            params,
        )?;
        deserialize(value)
    }

    pub fn get_pull_request(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
    ) -> Result<PullRequest> {
        let value = self.get(&format!(
            "/projects/{}/git/repositories/{}/pullRequests/{}",
            project_id_or_key, repo_id_or_name, number
        ))?;
        deserialize(value)
    }

    pub fn create_pull_request(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        params: &[(String, String)],
    ) -> Result<PullRequest> {
        let value = self.post_form(
            &format!(
                "/projects/{}/git/repositories/{}/pullRequests",
                project_id_or_key, repo_id_or_name
            ),
            params,
        )?;
        deserialize(value)
    }

    pub fn update_pull_request(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
        params: &[(String, String)],
    ) -> Result<PullRequest> {
        let value = self.patch_form(
            &format!(
                "/projects/{}/git/repositories/{}/pullRequests/{}",
                project_id_or_key, repo_id_or_name, number
            ),
            params,
        )?;
        deserialize(value)
    }

    pub fn get_pull_request_comments(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
        params: &[(String, String)],
    ) -> Result<Vec<PullRequestComment>> {
        let value = self.get_with_query(
            &format!(
                "/projects/{}/git/repositories/{}/pullRequests/{}/comments",
                project_id_or_key, repo_id_or_name, number
            ),
            params,
        )?;
        deserialize(value)
    }

    pub fn count_pull_request_comments(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
    ) -> Result<PullRequestCommentCount> {
        let value = self.get(&format!(
            "/projects/{}/git/repositories/{}/pullRequests/{}/comments/count",
            project_id_or_key, repo_id_or_name, number
        ))?;
        deserialize(value)
    }

    pub fn add_pull_request_comment(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
        params: &[(String, String)],
    ) -> Result<PullRequestComment> {
        let value = self.post_form(
            &format!(
                "/projects/{}/git/repositories/{}/pullRequests/{}/comments",
                project_id_or_key, repo_id_or_name, number
            ),
            params,
        )?;
        deserialize(value)
    }

    pub fn update_pull_request_comment(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
        comment_id: u64,
        params: &[(String, String)],
    ) -> Result<PullRequestComment> {
        let value = self.patch_form(
            &format!(
                "/projects/{}/git/repositories/{}/pullRequests/{}/comments/{}",
                project_id_or_key, repo_id_or_name, number, comment_id
            ),
            params,
        )?;
        deserialize(value)
    }

    pub fn get_pull_request_attachments(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
    ) -> Result<Vec<PullRequestAttachment>> {
        let value = self.get(&format!(
            "/projects/{}/git/repositories/{}/pullRequests/{}/attachments",
            project_id_or_key, repo_id_or_name, number
        ))?;
        deserialize(value)
    }

    pub fn download_pull_request_attachment(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
        attachment_id: u64,
    ) -> Result<(Vec<u8>, String)> {
        self.download(&format!(
            "/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
            project_id_or_key, repo_id_or_name, number, attachment_id
        ))
    }

    pub fn delete_pull_request_attachment(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
        number: u64,
        attachment_id: u64,
    ) -> Result<PullRequestAttachment> {
        let value = self.delete_req(&format!(
            "/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
            project_id_or_key, repo_id_or_name, number, attachment_id
        ))?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-api-key";

    fn pr_user_json() -> serde_json::Value {
        json!({
            "id": 1,
            "userId": "john",
            "name": "John Doe",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "john@example.com"
        })
    }

    fn pr_json() -> serde_json::Value {
        json!({
            "id": 1,
            "projectId": 10,
            "repositoryId": 2,
            "number": 1,
            "summary": "Fix bug",
            "description": "Fixes the bug",
            "base": "main",
            "branch": "feature/fix",
            "status": {"id": 1, "name": "Open"},
            "assignee": null,
            "issue": null,
            "baseCommit": null,
            "branchCommit": null,
            "mergeCommit": null,
            "closeAt": null,
            "mergeAt": null,
            "createdUser": pr_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": pr_user_json(),
            "updated": "2024-01-01T00:00:00Z",
            "attachments": [],
            "stars": []
        })
    }

    fn pr_comment_json() -> serde_json::Value {
        json!({
            "id": 1,
            "content": "LGTM",
            "changeLog": [],
            "createdUser": pr_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z",
            "stars": [],
            "notifications": []
        })
    }

    fn pr_attachment_json() -> serde_json::Value {
        json!({
            "id": 1,
            "name": "screenshot.png",
            "size": 1024,
            "createdUser": pr_user_json(),
            "created": "2024-01-01T00:00:00Z"
        })
    }

    fn client(server: &MockServer) -> super::super::BacklogClient {
        super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap()
    }

    #[test]
    fn get_pull_requests_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main/pullRequests")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([pr_json()]));
        });
        let prs = client(&server)
            .get_pull_requests("TEST", "main", &[])
            .unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].summary, "Fix bug");
        assert!(prs[0].assignee.is_none());
    }

    #[test]
    fn count_pull_requests_returns_count() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main/pullRequests/count")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!({"count": 5}));
        });
        let count = client(&server)
            .count_pull_requests("TEST", "main", &[])
            .unwrap();
        assert_eq!(count.count, 5);
    }

    #[test]
    fn get_pull_request_returns_single() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main/pullRequests/1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(pr_json());
        });
        let pr = client(&server).get_pull_request("TEST", "main", 1).unwrap();
        assert_eq!(pr.number, 1);
        assert_eq!(pr.base, "main");
    }

    #[test]
    fn get_pull_request_comments_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main/pullRequests/1/comments")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([pr_comment_json()]));
        });
        let comments = client(&server)
            .get_pull_request_comments("TEST", "main", 1, &[])
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].content.as_deref(), Some("LGTM"));
    }

    #[test]
    fn count_pull_request_comments_returns_count() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main/pullRequests/1/comments/count")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!({"count": 3}));
        });
        let count = client(&server)
            .count_pull_request_comments("TEST", "main", 1)
            .unwrap();
        assert_eq!(count.count, 3);
    }

    #[test]
    fn get_pull_request_attachments_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main/pullRequests/1/attachments")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([pr_attachment_json()]));
        });
        let attachments = client(&server)
            .get_pull_request_attachments("TEST", "main", 1)
            .unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].name, "screenshot.png");
    }

    #[test]
    fn create_pull_request_returns_pr() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST)
                .path("/projects/TEST/git/repositories/main/pullRequests")
                .query_param("apiKey", TEST_KEY);
            then.status(201).json_body(pr_json());
        });
        let pr = client(&server)
            .create_pull_request("TEST", "main", &[])
            .unwrap();
        assert_eq!(pr.number, 1);
        assert_eq!(pr.summary, "Fix bug");
    }

    #[test]
    fn update_pull_request_returns_pr() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(PATCH)
                .path("/projects/TEST/git/repositories/main/pullRequests/1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(pr_json());
        });
        let pr = client(&server)
            .update_pull_request("TEST", "main", 1, &[])
            .unwrap();
        assert_eq!(pr.number, 1);
    }

    #[test]
    fn add_pull_request_comment_returns_comment() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST)
                .path("/projects/TEST/git/repositories/main/pullRequests/1/comments")
                .query_param("apiKey", TEST_KEY);
            then.status(201).json_body(pr_comment_json());
        });
        let comment = client(&server)
            .add_pull_request_comment("TEST", "main", 1, &[])
            .unwrap();
        assert_eq!(comment.content.as_deref(), Some("LGTM"));
    }

    #[test]
    fn update_pull_request_comment_returns_comment() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(PATCH)
                .path("/projects/TEST/git/repositories/main/pullRequests/1/comments/1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(pr_comment_json());
        });
        let comment = client(&server)
            .update_pull_request_comment("TEST", "main", 1, 1, &[])
            .unwrap();
        assert_eq!(comment.id, 1);
    }

    #[test]
    fn download_pull_request_attachment_returns_bytes() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main/pullRequests/1/attachments/1");
            then.status(200)
                .header(
                    "Content-Disposition",
                    "attachment; filename=\"screenshot.png\"",
                )
                .body(b"hello");
        });
        let (bytes, filename) = client(&server)
            .download_pull_request_attachment("TEST", "main", 1, 1)
            .unwrap();
        assert_eq!(bytes, b"hello");
        assert_eq!(filename, "screenshot.png");
    }

    #[test]
    fn delete_pull_request_attachment_returns_attachment() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE)
                .path("/projects/TEST/git/repositories/main/pullRequests/1/attachments/1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(pr_attachment_json());
        });
        let attachment = client(&server)
            .delete_pull_request_attachment("TEST", "main", 1, 1)
            .unwrap();
        assert_eq!(attachment.name, "screenshot.png");
    }
}
