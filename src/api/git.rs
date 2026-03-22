use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitUser {
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
pub struct GitRepository {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub description: String,
    pub hook_url: Option<String>,
    pub http_url: String,
    pub ssh_url: String,
    pub display_order: u64,
    pub pushed_at: Option<String>,
    pub created_user: GitUser,
    pub created: String,
    pub updated_user: GitUser,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BacklogClient {
    pub fn get_git_repositories(&self, project_id_or_key: &str) -> Result<Vec<GitRepository>> {
        let value = self.get(&format!("/projects/{}/git/repositories", project_id_or_key))?;
        deserialize(value)
    }

    pub fn get_git_repository(
        &self,
        project_id_or_key: &str,
        repo_id_or_name: &str,
    ) -> Result<GitRepository> {
        let value = self.get(&format!(
            "/projects/{}/git/repositories/{}",
            project_id_or_key, repo_id_or_name
        ))?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-api-key";

    fn git_user_json() -> serde_json::Value {
        json!({
            "id": 1,
            "userId": "john",
            "name": "John Doe",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "john@example.com"
        })
    }

    fn git_repo_json() -> serde_json::Value {
        json!({
            "id": 1,
            "projectId": 10,
            "name": "main",
            "description": "My repository",
            "hookUrl": null,
            "httpUrl": "https://example.backlog.com/git/TEST/main.git",
            "sshUrl": "git@example.backlog.com:/TEST/main.git",
            "displayOrder": 0,
            "pushedAt": null,
            "createdUser": git_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": git_user_json(),
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    #[test]
    fn get_git_repositories_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([git_repo_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let repos = client.get_git_repositories("TEST").unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "main");
        assert_eq!(repos[0].hook_url, None);
        assert_eq!(repos[0].pushed_at, None);
    }

    #[test]
    fn get_git_repository_returns_single() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/projects/TEST/git/repositories/main")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(git_repo_json());
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let repo = client.get_git_repository("TEST", "main").unwrap();
        assert_eq!(repo.id, 1);
        assert_eq!(repo.name, "main");
    }
}
