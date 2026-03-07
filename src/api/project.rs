use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{BacklogClient, activity::Activity};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: u64,
    pub project_key: String,
    pub name: String,
    pub chart_enabled: bool,
    pub subtasking_enabled: bool,
    pub project_leader_can_edit_project_leader: bool,
    pub text_formatting_rule: String,
    pub archived: bool,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDiskUsage {
    pub project_id: u64,
    pub issue: u64,
    pub wiki: u64,
    pub document: u64,
    pub file: u64,
    pub subversion: u64,
    pub git: u64,
    #[serde(rename = "gitLFS")]
    pub git_lfs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUser {
    pub id: u64,
    pub user_id: Option<String>,
    pub name: String,
    pub role_type: u8,
    pub lang: Option<String>,
    pub mail_address: Option<String>,
    pub last_login_time: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatus {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub color: String,
    pub display_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIssueType {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub color: String,
    pub display_order: u32,
    pub template_summary: Option<String>,
    pub template_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCategory {
    pub id: u64,
    pub name: String,
    pub display_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVersion {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub release_due_date: Option<String>,
    pub archived: bool,
    pub display_order: u32,
}

impl BacklogClient {
    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let value = self.get("/projects")?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize projects response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project(&self, key: &str) -> Result<Project> {
        let value = self.get(&format!("/projects/{}", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project_activities(&self, key: &str) -> Result<Vec<Activity>> {
        let value = self.get(&format!("/projects/{}/activities", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project activities response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project_disk_usage(&self, key: &str) -> Result<ProjectDiskUsage> {
        let value = self.get(&format!("/projects/{}/diskUsage", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project disk usage response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project_users(&self, key: &str) -> Result<Vec<ProjectUser>> {
        let value = self.get(&format!("/projects/{}/users", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project users response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project_statuses(&self, key: &str) -> Result<Vec<ProjectStatus>> {
        let value = self.get(&format!("/projects/{}/statuses", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project statuses response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project_issue_types(&self, key: &str) -> Result<Vec<ProjectIssueType>> {
        let value = self.get(&format!("/projects/{}/issueTypes", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project issue types response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project_categories(&self, key: &str) -> Result<Vec<ProjectCategory>> {
        let value = self.get(&format!("/projects/{}/categories", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project categories response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_project_versions(&self, key: &str) -> Result<Vec<ProjectVersion>> {
        let value = self.get(&format!("/projects/{}/versions", key))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize project versions response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    fn project_json() -> serde_json::Value {
        json!({
            "id": 1,
            "projectKey": "TEST",
            "name": "Test Project",
            "chartEnabled": false,
            "subtaskingEnabled": false,
            "projectLeaderCanEditProjectLeader": false,
            "textFormattingRule": "markdown",
            "archived": false
        })
    }

    #[test]
    fn get_projects_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects");
            then.status(200).json_body(json!([project_json()]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let projects = client.get_projects().unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, 1);
        assert_eq!(projects[0].project_key, "TEST");
        assert_eq!(projects[0].name, "Test Project");
        assert!(!projects[0].archived);
    }

    #[test]
    fn get_project_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST");
            then.status(200).json_body(project_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let project = client.get_project("TEST").unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.project_key, "TEST");
        assert_eq!(project.name, "Test Project");
    }

    #[test]
    fn get_project_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/UNKNOWN");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No project"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_project("UNKNOWN").unwrap_err();
        assert!(err.to_string().contains("No project"));
    }

    #[test]
    fn get_projects_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_projects().unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }

    #[test]
    fn get_project_activities_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/activities");
            then.status(200).json_body(json!([{
                "id": 1,
                "type": 1,
                "content": {},
                "createdUser": {"id": 1, "userId": "john", "name": "John"},
                "created": "2024-01-01T00:00:00Z"
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let acts = client.get_project_activities("TEST").unwrap();
        assert_eq!(acts.len(), 1);
        assert_eq!(acts[0].id, 1);
    }

    #[test]
    fn get_project_activities_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/activities");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_project_activities("TEST").unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn get_project_disk_usage_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/diskUsage");
            then.status(200).json_body(json!({
                "projectId": 1,
                "issue": 2048,
                "wiki": 512,
                "document": 0,
                "file": 1024,
                "subversion": 64,
                "git": 256,
                "gitLFS": 128
            }));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let usage = client.get_project_disk_usage("TEST").unwrap();
        assert_eq!(usage.project_id, 1);
        assert_eq!(usage.issue, 2048);
        assert_eq!(usage.git_lfs, 128);
    }

    #[test]
    fn get_project_disk_usage_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/diskUsage");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_project_disk_usage("TEST").unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn get_project_users_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/users");
            then.status(200).json_body(json!([{
                "id": 1,
                "userId": "john",
                "name": "John Doe",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "john@example.com",
                "lastLoginTime": "2024-01-01T00:00:00Z"
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let users = client.get_project_users("TEST").unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, 1);
        assert_eq!(users[0].name, "John Doe");
    }

    #[test]
    fn get_project_users_handles_null_user_id() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/users");
            then.status(200).json_body(json!([{
                "id": 99,
                "userId": null,
                "name": "Bot",
                "roleType": 6,
                "lang": null,
                "mailAddress": null,
                "lastLoginTime": null
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let users = client.get_project_users("TEST").unwrap();
        assert!(users[0].user_id.is_none());
    }

    #[test]
    fn get_project_statuses_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/statuses");
            then.status(200).json_body(json!([{
                "id": 1,
                "projectId": 1,
                "name": "Open",
                "color": "#ed8077",
                "displayOrder": 1000
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let statuses = client.get_project_statuses("TEST").unwrap();
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].name, "Open");
    }

    #[test]
    fn get_project_issue_types_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/issueTypes");
            then.status(200).json_body(json!([{
                "id": 1,
                "projectId": 1,
                "name": "Bug",
                "color": "#e30000",
                "displayOrder": 0,
                "templateSummary": null,
                "templateDescription": null
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let types = client.get_project_issue_types("TEST").unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].name, "Bug");
        assert!(types[0].template_summary.is_none());
    }

    #[test]
    fn get_project_categories_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/categories");
            then.status(200).json_body(json!([{
                "id": 11,
                "name": "Development",
                "displayOrder": 0
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let cats = client.get_project_categories("TEST").unwrap();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].name, "Development");
    }

    #[test]
    fn get_project_versions_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/versions");
            then.status(200).json_body(json!([{
                "id": 3,
                "projectId": 1,
                "name": "Version 0.1",
                "description": "",
                "startDate": "2024-01-01T00:00:00Z",
                "releaseDueDate": "2024-01-31T00:00:00Z",
                "archived": false,
                "displayOrder": 0
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let versions = client.get_project_versions("TEST").unwrap();
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].name, "Version 0.1");
        assert!(!versions[0].archived);
    }
}
