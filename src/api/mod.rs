use anyhow::{Context, Result};
use reqwest::blocking::Client;

pub mod activity;
pub mod disk_usage;
pub mod issue;
pub mod project;
pub mod space;
pub mod space_notification;
pub mod user;
pub mod wiki;

use activity::Activity;
use disk_usage::DiskUsage;
use issue::{Issue, IssueAttachment, IssueComment, IssueCount};
use project::{
    Project, ProjectCategory, ProjectDiskUsage, ProjectIssueType, ProjectStatus, ProjectUser,
    ProjectVersion,
};
use space::Space;
use space_notification::SpaceNotification;
use user::User;
use wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem};

pub trait BacklogApi {
    fn get_space(&self) -> Result<Space>;
    fn get_myself(&self) -> Result<User>;
    fn get_space_activities(&self) -> Result<Vec<Activity>>;
    fn get_space_disk_usage(&self) -> Result<DiskUsage>;
    fn get_space_notification(&self) -> Result<SpaceNotification>;
    fn get_projects(&self) -> Result<Vec<Project>>;
    fn get_project(&self, key: &str) -> Result<Project>;
    fn get_project_activities(&self, key: &str) -> Result<Vec<Activity>>;
    fn get_project_disk_usage(&self, key: &str) -> Result<ProjectDiskUsage>;
    fn get_project_users(&self, key: &str) -> Result<Vec<ProjectUser>>;
    fn get_project_statuses(&self, key: &str) -> Result<Vec<ProjectStatus>>;
    fn get_project_issue_types(&self, key: &str) -> Result<Vec<ProjectIssueType>>;
    fn get_project_categories(&self, key: &str) -> Result<Vec<ProjectCategory>>;
    fn get_project_versions(&self, key: &str) -> Result<Vec<ProjectVersion>>;
    fn get_issues(&self, params: &[(String, String)]) -> Result<Vec<Issue>>;
    fn count_issues(&self, params: &[(String, String)]) -> Result<IssueCount>;
    fn get_issue(&self, key: &str) -> Result<Issue>;
    fn create_issue(&self, params: &[(String, String)]) -> Result<Issue>;
    fn update_issue(&self, key: &str, params: &[(String, String)]) -> Result<Issue>;
    fn delete_issue(&self, key: &str) -> Result<Issue>;
    fn get_issue_comments(&self, key: &str) -> Result<Vec<IssueComment>>;
    fn add_issue_comment(&self, key: &str, params: &[(String, String)]) -> Result<IssueComment>;
    fn update_issue_comment(
        &self,
        key: &str,
        comment_id: u64,
        params: &[(String, String)],
    ) -> Result<IssueComment>;
    fn delete_issue_comment(&self, key: &str, comment_id: u64) -> Result<IssueComment>;
    fn get_issue_attachments(&self, key: &str) -> Result<Vec<IssueAttachment>>;
    fn get_wikis(&self, params: &[(String, String)]) -> Result<Vec<WikiListItem>>;
    fn get_wiki(&self, wiki_id: u64) -> Result<Wiki>;
    fn create_wiki(&self, params: &[(String, String)]) -> Result<Wiki>;
    fn update_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki>;
    fn delete_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki>;
    fn get_wiki_history(&self, wiki_id: u64) -> Result<Vec<WikiHistory>>;
    fn get_wiki_attachments(&self, wiki_id: u64) -> Result<Vec<WikiAttachment>>;
}

impl BacklogApi for BacklogClient {
    fn get_space(&self) -> Result<Space> {
        self.get_space()
    }

    fn get_myself(&self) -> Result<User> {
        self.get_myself()
    }

    fn get_space_activities(&self) -> Result<Vec<Activity>> {
        self.get_space_activities()
    }

    fn get_space_disk_usage(&self) -> Result<DiskUsage> {
        self.get_space_disk_usage()
    }

    fn get_space_notification(&self) -> Result<SpaceNotification> {
        self.get_space_notification()
    }

    fn get_projects(&self) -> Result<Vec<Project>> {
        self.get_projects()
    }

    fn get_project(&self, key: &str) -> Result<Project> {
        self.get_project(key)
    }

    fn get_project_activities(&self, key: &str) -> Result<Vec<Activity>> {
        self.get_project_activities(key)
    }

    fn get_project_disk_usage(&self, key: &str) -> Result<ProjectDiskUsage> {
        self.get_project_disk_usage(key)
    }

    fn get_project_users(&self, key: &str) -> Result<Vec<ProjectUser>> {
        self.get_project_users(key)
    }

    fn get_project_statuses(&self, key: &str) -> Result<Vec<ProjectStatus>> {
        self.get_project_statuses(key)
    }

    fn get_project_issue_types(&self, key: &str) -> Result<Vec<ProjectIssueType>> {
        self.get_project_issue_types(key)
    }

    fn get_project_categories(&self, key: &str) -> Result<Vec<ProjectCategory>> {
        self.get_project_categories(key)
    }

    fn get_project_versions(&self, key: &str) -> Result<Vec<ProjectVersion>> {
        self.get_project_versions(key)
    }

    fn get_issues(&self, params: &[(String, String)]) -> Result<Vec<Issue>> {
        self.get_issues(params)
    }

    fn count_issues(&self, params: &[(String, String)]) -> Result<IssueCount> {
        self.count_issues(params)
    }

    fn get_issue(&self, key: &str) -> Result<Issue> {
        self.get_issue(key)
    }

    fn create_issue(&self, params: &[(String, String)]) -> Result<Issue> {
        self.create_issue(params)
    }

    fn update_issue(&self, key: &str, params: &[(String, String)]) -> Result<Issue> {
        self.update_issue(key, params)
    }

    fn delete_issue(&self, key: &str) -> Result<Issue> {
        self.delete_issue(key)
    }

    fn get_issue_comments(&self, key: &str) -> Result<Vec<IssueComment>> {
        self.get_issue_comments(key)
    }

    fn add_issue_comment(&self, key: &str, params: &[(String, String)]) -> Result<IssueComment> {
        self.add_issue_comment(key, params)
    }

    fn update_issue_comment(
        &self,
        key: &str,
        comment_id: u64,
        params: &[(String, String)],
    ) -> Result<IssueComment> {
        self.update_issue_comment(key, comment_id, params)
    }

    fn delete_issue_comment(&self, key: &str, comment_id: u64) -> Result<IssueComment> {
        self.delete_issue_comment(key, comment_id)
    }

    fn get_issue_attachments(&self, key: &str) -> Result<Vec<IssueAttachment>> {
        self.get_issue_attachments(key)
    }

    fn get_wikis(&self, params: &[(String, String)]) -> Result<Vec<WikiListItem>> {
        self.get_wikis(params)
    }

    fn get_wiki(&self, wiki_id: u64) -> Result<Wiki> {
        self.get_wiki(wiki_id)
    }

    fn create_wiki(&self, params: &[(String, String)]) -> Result<Wiki> {
        self.create_wiki(params)
    }

    fn update_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki> {
        self.update_wiki(wiki_id, params)
    }

    fn delete_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki> {
        self.delete_wiki(wiki_id, params)
    }

    fn get_wiki_history(&self, wiki_id: u64) -> Result<Vec<WikiHistory>> {
        self.get_wiki_history(wiki_id)
    }

    fn get_wiki_attachments(&self, wiki_id: u64) -> Result<Vec<WikiAttachment>> {
        self.get_wiki_attachments(wiki_id)
    }
}

pub struct BacklogClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl BacklogClient {
    pub fn from_config() -> Result<Self> {
        let space_key = crate::config::current_space_key()?;
        let (api_key, _) = crate::secret::get(&space_key)?;

        let client = Client::builder()
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            base_url: format!("https://{}.backlog.com/api/v2", space_key),
            api_key,
        })
    }

    pub fn get(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .query(&[("apiKey", &self.api_key)])
            .send()
            .with_context(|| format!("Failed to GET {}", url))?;

        let status = response.status();
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }

        Ok(body)
    }

    pub fn get_with_query(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut query: Vec<(&str, &str)> = vec![("apiKey", &self.api_key)];
        let extra: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        query.extend(extra.iter().copied());
        let response = self
            .client
            .get(&url)
            .query(&query)
            .send()
            .with_context(|| format!("Failed to GET {}", url))?;

        let status = response.status();
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }

        Ok(body)
    }

    pub fn post_form(&self, path: &str, params: &[(String, String)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .post(&url)
            .query(&[("apiKey", &self.api_key)])
            .form(params)
            .send()
            .with_context(|| format!("Failed to POST {}", url))?;

        let status = response.status();
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }

        Ok(body)
    }

    pub fn patch_form(&self, path: &str, params: &[(String, String)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .patch(&url)
            .query(&[("apiKey", &self.api_key)])
            .form(params)
            .send()
            .with_context(|| format!("Failed to PATCH {}", url))?;

        let status = response.status();
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }

        Ok(body)
    }

    pub fn delete_form(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .delete(&url)
            .query(&[("apiKey", &self.api_key)])
            .form(params)
            .send()
            .with_context(|| format!("Failed to DELETE {}", url))?;

        let status = response.status();
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }

        Ok(body)
    }

    pub fn delete_req(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .delete(&url)
            .query(&[("apiKey", &self.api_key)])
            .send()
            .with_context(|| format!("Failed to DELETE {}", url))?;

        let status = response.status();
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;

        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }

        Ok(body)
    }
}

fn extract_error_message(body: &serde_json::Value) -> &str {
    body.get("errors")
        .and_then(|e| e.get(0))
        .and_then(|e| e.get("message"))
        .and_then(|m| m.as_str())
        .unwrap_or("Unknown error")
}

impl BacklogClient {
    pub(crate) fn new_with(base_url: &str, api_key: &str) -> Result<Self> {
        let client = Client::builder()
            .build()
            .context("Failed to build HTTP client")?;
        Ok(Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-api-key";

    #[test]
    fn get_returns_body_on_success() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/space")
                .query_param("apiKey", TEST_KEY);
            then.status(200)
                .json_body(json!({"spaceKey": "mycompany", "name": "My Company"}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let body = client.get("/space").unwrap();
        assert_eq!(body["spaceKey"], "mycompany");
    }

    #[test]
    fn get_returns_error_with_api_message_on_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get("/space").unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }

    #[test]
    fn get_returns_error_with_fallback_message_on_unknown_error() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(500).json_body(json!({}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get("/space").unwrap_err();
        assert!(err.to_string().contains("Unknown error"));
    }

    #[test]
    fn extract_error_message_from_errors_array() {
        let body = json!({"errors": [{"message": "Authentication failure"}]});
        assert_eq!(extract_error_message(&body), "Authentication failure");
    }

    #[test]
    fn extract_error_message_fallback_when_missing() {
        let body = json!({});
        assert_eq!(extract_error_message(&body), "Unknown error");
    }

    #[test]
    fn extract_error_message_fallback_when_empty_errors() {
        let body = json!({"errors": []});
        assert_eq!(extract_error_message(&body), "Unknown error");
    }
}
