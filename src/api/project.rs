use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::deserialize;
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
pub struct CustomFieldItem {
    pub id: u64,
    pub name: String,
    pub display_order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCustomField {
    pub id: u64,
    pub type_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub applicable_issue_types: Vec<serde_json::Value>,
    pub allow_add_item: Option<bool>,
    pub items: Option<Vec<CustomFieldItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookUser {
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
pub struct ProjectWebhook {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub hook_url: String,
    pub all_event: bool,
    pub activity_type_ids: Vec<u64>,
    pub created_user: WebhookUser,
    pub created: String,
    pub updated_user: WebhookUser,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCategory {
    pub id: u64,
    pub project_id: u64,
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

pub struct UpdateProjectWebhookParams<'a> {
    pub name: Option<&'a str>,
    pub hook_url: Option<&'a str>,
    pub description: Option<&'a str>,
    pub all_event: Option<bool>,
    pub activity_type_ids: Option<&'a [u64]>,
}

pub struct UpdateProjectVersionParams<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub start_date: Option<&'a str>,
    pub release_due_date: Option<&'a str>,
    pub archived: Option<bool>,
}

impl BacklogClient {
    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let value = self.get("/projects")?;
        deserialize(value)
    }

    pub fn get_project(&self, key: &str) -> Result<Project> {
        let value = self.get(&format!("/projects/{}", key))?;
        deserialize(value)
    }

    pub fn get_project_activities(
        &self,
        key: &str,
        params: &[(String, String)],
    ) -> Result<Vec<Activity>> {
        let value = self.get_with_query(&format!("/projects/{}/activities", key), params)?;
        deserialize(value)
    }

    pub fn get_project_disk_usage(&self, key: &str) -> Result<ProjectDiskUsage> {
        let value = self.get(&format!("/projects/{}/diskUsage", key))?;
        deserialize(value)
    }

    pub fn get_project_users(&self, key: &str) -> Result<Vec<ProjectUser>> {
        let value = self.get(&format!("/projects/{}/users", key))?;
        deserialize(value)
    }

    pub fn get_project_statuses(&self, key: &str) -> Result<Vec<ProjectStatus>> {
        let value = self.get(&format!("/projects/{}/statuses", key))?;
        deserialize(value)
    }

    pub fn get_project_issue_types(&self, key: &str) -> Result<Vec<ProjectIssueType>> {
        let value = self.get(&format!("/projects/{}/issueTypes", key))?;
        deserialize(value)
    }

    pub fn add_project_issue_type(
        &self,
        key: &str,
        name: &str,
        color: &str,
    ) -> Result<ProjectIssueType> {
        let params = vec![
            ("name".to_string(), name.to_string()),
            ("color".to_string(), color.to_string()),
        ];
        let value = self.post_form(&format!("/projects/{}/issueTypes", key), &params)?;
        deserialize(value)
    }

    pub fn update_project_issue_type(
        &self,
        key: &str,
        issue_type_id: u64,
        params: &[(String, String)],
    ) -> Result<ProjectIssueType> {
        let value = self.patch_form(
            &format!("/projects/{}/issueTypes/{}", key, issue_type_id),
            params,
        )?;
        deserialize(value)
    }

    pub fn delete_project_issue_type(
        &self,
        key: &str,
        issue_type_id: u64,
        substitute_issue_type_id: u64,
    ) -> Result<ProjectIssueType> {
        let params = vec![(
            "substituteIssueTypeId".to_string(),
            substitute_issue_type_id.to_string(),
        )];
        let value = self.delete_form(
            &format!("/projects/{}/issueTypes/{}", key, issue_type_id),
            &params,
        )?;
        deserialize(value)
    }

    pub fn get_project_categories(&self, key: &str) -> Result<Vec<ProjectCategory>> {
        let value = self.get(&format!("/projects/{}/categories", key))?;
        deserialize(value)
    }

    pub fn add_project_category(&self, key: &str, name: &str) -> Result<ProjectCategory> {
        let params = vec![("name".to_string(), name.to_string())];
        let value = self.post_form(&format!("/projects/{}/categories", key), &params)?;
        deserialize(value)
    }

    pub fn update_project_category(
        &self,
        key: &str,
        category_id: u64,
        name: &str,
    ) -> Result<ProjectCategory> {
        let params = vec![("name".to_string(), name.to_string())];
        let value = self.patch_form(
            &format!("/projects/{}/categories/{}", key, category_id),
            &params,
        )?;
        deserialize(value)
    }

    pub fn delete_project_category(&self, key: &str, category_id: u64) -> Result<ProjectCategory> {
        let value = self.delete_req(&format!("/projects/{}/categories/{}", key, category_id))?;
        deserialize(value)
    }

    pub fn add_project_version(
        &self,
        key: &str,
        name: &str,
        description: Option<&str>,
        start_date: Option<&str>,
        release_due_date: Option<&str>,
    ) -> Result<ProjectVersion> {
        let mut params = vec![("name".to_string(), name.to_string())];
        if let Some(d) = description {
            params.push(("description".to_string(), d.to_string()));
        }
        if let Some(s) = start_date {
            params.push(("startDate".to_string(), s.to_string()));
        }
        if let Some(r) = release_due_date {
            params.push(("releaseDueDate".to_string(), r.to_string()));
        }
        let value = self.post_form(&format!("/projects/{}/versions", key), &params)?;
        deserialize(value)
    }

    pub fn update_project_version(
        &self,
        key: &str,
        version_id: u64,
        params: &UpdateProjectVersionParams<'_>,
    ) -> Result<ProjectVersion> {
        let mut form = vec![("name".to_string(), params.name.to_string())];
        if let Some(d) = params.description {
            form.push(("description".to_string(), d.to_string()));
        }
        if let Some(s) = params.start_date {
            form.push(("startDate".to_string(), s.to_string()));
        }
        if let Some(r) = params.release_due_date {
            form.push(("releaseDueDate".to_string(), r.to_string()));
        }
        if let Some(a) = params.archived {
            form.push(("archived".to_string(), a.to_string()));
        }
        let value =
            self.patch_form(&format!("/projects/{}/versions/{}", key, version_id), &form)?;
        deserialize(value)
    }

    pub fn delete_project_version(&self, key: &str, version_id: u64) -> Result<ProjectVersion> {
        let value = self.delete_req(&format!("/projects/{}/versions/{}", key, version_id))?;
        deserialize(value)
    }

    pub fn get_project_versions(&self, key: &str) -> Result<Vec<ProjectVersion>> {
        let value = self.get(&format!("/projects/{}/versions", key))?;
        deserialize(value)
    }

    pub fn create_project(&self, params: &[(String, String)]) -> Result<Project> {
        let value = self.post_form("/projects", params)?;
        deserialize(value)
    }

    pub fn update_project(&self, key: &str, params: &[(String, String)]) -> Result<Project> {
        let value = self.patch_form(&format!("/projects/{}", key), params)?;
        deserialize(value)
    }

    pub fn delete_project(&self, key: &str) -> Result<Project> {
        let value = self.delete_req(&format!("/projects/{}", key))?;
        deserialize(value)
    }

    pub fn add_project_status(&self, key: &str, name: &str, color: &str) -> Result<ProjectStatus> {
        let params = vec![
            ("name".to_string(), name.to_string()),
            ("color".to_string(), color.to_string()),
        ];
        let value = self.post_form(&format!("/projects/{}/statuses", key), &params)?;
        deserialize(value)
    }

    pub fn update_project_status(
        &self,
        key: &str,
        status_id: u64,
        params: &[(String, String)],
    ) -> Result<ProjectStatus> {
        let value =
            self.patch_form(&format!("/projects/{}/statuses/{}", key, status_id), params)?;
        deserialize(value)
    }

    pub fn delete_project_status(
        &self,
        key: &str,
        status_id: u64,
        substitute_status_id: u64,
    ) -> Result<ProjectStatus> {
        let params = vec![(
            "substituteStatusId".to_string(),
            substitute_status_id.to_string(),
        )];
        let value = self.delete_form(
            &format!("/projects/{}/statuses/{}", key, status_id),
            &params,
        )?;
        deserialize(value)
    }

    pub fn reorder_project_statuses(
        &self,
        key: &str,
        status_ids: &[u64],
    ) -> Result<Vec<ProjectStatus>> {
        let params: Vec<(String, String)> = status_ids
            .iter()
            .map(|id| ("statusId[]".to_string(), id.to_string()))
            .collect();
        let value = self.patch_form(
            &format!("/projects/{}/statuses/updateDisplayOrder", key),
            &params,
        )?;
        deserialize(value)
    }

    pub fn add_project_user(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        let params = vec![("userId".to_string(), user_id.to_string())];
        let value = self.post_form(&format!("/projects/{}/users", key), &params)?;
        deserialize(value)
    }

    pub fn delete_project_user(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        let params = vec![("userId".to_string(), user_id.to_string())];
        let value = self.delete_form(&format!("/projects/{}/users", key), &params)?;
        deserialize(value)
    }

    pub fn get_project_administrators(&self, key: &str) -> Result<Vec<ProjectUser>> {
        let value = self.get(&format!("/projects/{}/administrators", key))?;
        deserialize(value)
    }

    pub fn add_project_administrator(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        let params = vec![("userId".to_string(), user_id.to_string())];
        let value = self.post_form(&format!("/projects/{}/administrators", key), &params)?;
        deserialize(value)
    }

    pub fn delete_project_administrator(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        let params = vec![("userId".to_string(), user_id.to_string())];
        let value = self.delete_form(&format!("/projects/{}/administrators", key), &params)?;
        deserialize(value)
    }

    pub fn get_project_custom_fields(&self, key: &str) -> Result<Vec<ProjectCustomField>> {
        let value = self.get(&format!("/projects/{}/customFields", key))?;
        deserialize(value)
    }

    pub fn add_project_custom_field(
        &self,
        key: &str,
        type_id: u64,
        name: &str,
        description: Option<&str>,
        required: Option<bool>,
    ) -> Result<ProjectCustomField> {
        let mut params = vec![
            ("typeId".to_string(), type_id.to_string()),
            ("name".to_string(), name.to_string()),
        ];
        if let Some(d) = description {
            params.push(("description".to_string(), d.to_string()));
        }
        if let Some(r) = required {
            params.push(("required".to_string(), r.to_string()));
        }
        let value = self.post_form(&format!("/projects/{}/customFields", key), &params)?;
        deserialize(value)
    }

    pub fn update_project_custom_field(
        &self,
        key: &str,
        custom_field_id: u64,
        name: Option<&str>,
        description: Option<&str>,
        required: Option<bool>,
    ) -> Result<ProjectCustomField> {
        let mut params = vec![];
        if let Some(n) = name {
            params.push(("name".to_string(), n.to_string()));
        }
        if let Some(d) = description {
            params.push(("description".to_string(), d.to_string()));
        }
        if let Some(r) = required {
            params.push(("required".to_string(), r.to_string()));
        }
        let value = self.patch_form(
            &format!("/projects/{}/customFields/{}", key, custom_field_id),
            &params,
        )?;
        deserialize(value)
    }

    pub fn delete_project_custom_field(
        &self,
        key: &str,
        custom_field_id: u64,
    ) -> Result<ProjectCustomField> {
        let value = self.delete_req(&format!(
            "/projects/{}/customFields/{}",
            key, custom_field_id
        ))?;
        deserialize(value)
    }

    pub fn add_project_custom_field_item(
        &self,
        key: &str,
        custom_field_id: u64,
        name: &str,
    ) -> Result<ProjectCustomField> {
        let params = vec![("name".to_string(), name.to_string())];
        let value = self.post_form(
            &format!("/projects/{}/customFields/{}/items", key, custom_field_id),
            &params,
        )?;
        deserialize(value)
    }

    pub fn update_project_custom_field_item(
        &self,
        key: &str,
        custom_field_id: u64,
        item_id: u64,
        name: &str,
    ) -> Result<ProjectCustomField> {
        let params = vec![("name".to_string(), name.to_string())];
        let value = self.patch_form(
            &format!(
                "/projects/{}/customFields/{}/items/{}",
                key, custom_field_id, item_id
            ),
            &params,
        )?;
        deserialize(value)
    }

    pub fn delete_project_custom_field_item(
        &self,
        key: &str,
        custom_field_id: u64,
        item_id: u64,
    ) -> Result<ProjectCustomField> {
        let value = self.delete_req(&format!(
            "/projects/{}/customFields/{}/items/{}",
            key, custom_field_id, item_id
        ))?;
        deserialize(value)
    }

    pub fn get_project_webhooks(&self, key: &str) -> Result<Vec<ProjectWebhook>> {
        let value = self.get(&format!("/projects/{key}/webhooks"))?;
        deserialize(value)
    }

    pub fn get_project_webhook(&self, key: &str, webhook_id: u64) -> Result<ProjectWebhook> {
        let value = self.get(&format!("/projects/{key}/webhooks/{webhook_id}"))?;
        deserialize(value)
    }

    pub fn add_project_webhook(
        &self,
        key: &str,
        name: &str,
        hook_url: &str,
        description: Option<&str>,
        all_event: Option<bool>,
        activity_type_ids: &[u64],
    ) -> Result<ProjectWebhook> {
        let mut params = vec![
            ("name".to_string(), name.to_string()),
            ("hookUrl".to_string(), hook_url.to_string()),
        ];
        if let Some(d) = description {
            params.push(("description".to_string(), d.to_string()));
        }
        if let Some(a) = all_event {
            params.push(("allEvent".to_string(), a.to_string()));
        }
        for id in activity_type_ids {
            params.push(("activityTypeIds[]".to_string(), id.to_string()));
        }
        let value = self.post_form(&format!("/projects/{key}/webhooks"), &params)?;
        deserialize(value)
    }

    pub fn update_project_webhook(
        &self,
        key: &str,
        webhook_id: u64,
        params: &UpdateProjectWebhookParams<'_>,
    ) -> Result<ProjectWebhook> {
        let mut form = vec![];
        if let Some(n) = params.name {
            form.push(("name".to_string(), n.to_string()));
        }
        if let Some(u) = params.hook_url {
            form.push(("hookUrl".to_string(), u.to_string()));
        }
        if let Some(d) = params.description {
            form.push(("description".to_string(), d.to_string()));
        }
        if let Some(a) = params.all_event {
            form.push(("allEvent".to_string(), a.to_string()));
        }
        if let Some(ids) = params.activity_type_ids {
            for id in ids {
                form.push(("activityTypeIds[]".to_string(), id.to_string()));
            }
        }
        let value = self.patch_form(&format!("/projects/{key}/webhooks/{webhook_id}"), &form)?;
        deserialize(value)
    }

    pub fn delete_project_webhook(&self, key: &str, webhook_id: u64) -> Result<ProjectWebhook> {
        let value = self.delete_req(&format!("/projects/{key}/webhooks/{webhook_id}"))?;
        deserialize(value)
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
        let acts = client.get_project_activities("TEST", &[]).unwrap();
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
        let err = client.get_project_activities("TEST", &[]).unwrap_err();
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
                "projectId": 1,
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

    #[test]
    fn get_project_versions_handles_null_description() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/versions");
            then.status(200).json_body(json!([{
                "id": 3,
                "projectId": 1,
                "name": "Version 0.1",
                "description": null,
                "startDate": null,
                "releaseDueDate": null,
                "archived": false,
                "displayOrder": 0
            }]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let versions = client.get_project_versions("TEST").unwrap();
        assert!(versions[0].description.is_none());
    }

    #[test]
    fn create_project_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects");
            then.status(201).json_body(project_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let params = vec![
            ("name".to_string(), "Test Project".to_string()),
            ("key".to_string(), "TEST".to_string()),
            ("chartEnabled".to_string(), "false".to_string()),
            ("subtaskingEnabled".to_string(), "false".to_string()),
            ("textFormattingRule".to_string(), "markdown".to_string()),
        ];
        let project = client.create_project(&params).unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.project_key, "TEST");
    }

    #[test]
    fn create_project_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.create_project(&[]).unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn update_project_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH).path("/projects/TEST");
            then.status(200).json_body(project_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let params = vec![("name".to_string(), "New Name".to_string())];
        let project = client.update_project("TEST", &params).unwrap();
        assert_eq!(project.id, 1);
    }

    #[test]
    fn update_project_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path("/projects/UNKNOWN");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No project"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.update_project("UNKNOWN", &[]).unwrap_err();
        assert!(err.to_string().contains("No project"));
    }

    #[test]
    fn delete_project_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/TEST");
            then.status(200).json_body(project_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let project = client.delete_project("TEST").unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.project_key, "TEST");
    }

    #[test]
    fn delete_project_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/UNKNOWN");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No project"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.delete_project("UNKNOWN").unwrap_err();
        assert!(err.to_string().contains("No project"));
    }

    fn project_user_json() -> serde_json::Value {
        json!({
            "id": 1,
            "userId": "john",
            "name": "John Doe",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "john@example.com",
            "lastLoginTime": null
        })
    }

    #[test]
    fn add_project_user_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects/TEST/users");
            then.status(200).json_body(project_user_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let user = client.add_project_user("TEST", 1).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
    }

    #[test]
    fn add_project_user_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects/TEST/users");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.add_project_user("TEST", 1).unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn delete_project_user_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/TEST/users");
            then.status(200).json_body(project_user_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let user = client.delete_project_user("TEST", 1).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
    }

    #[test]
    fn delete_project_user_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/TEST/users");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No user"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.delete_project_user("TEST", 1).unwrap_err();
        assert!(err.to_string().contains("No user"));
    }

    #[test]
    fn get_project_administrators_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/administrators");
            then.status(200).json_body(json!([project_user_json()]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let admins = client.get_project_administrators("TEST").unwrap();
        assert_eq!(admins.len(), 1);
        assert_eq!(admins[0].id, 1);
        assert_eq!(admins[0].name, "John Doe");
    }

    #[test]
    fn get_project_administrators_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/administrators");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_project_administrators("TEST").unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn add_project_administrator_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects/TEST/administrators");
            then.status(200).json_body(project_user_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let user = client.add_project_administrator("TEST", 1).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
    }

    #[test]
    fn add_project_administrator_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects/TEST/administrators");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.add_project_administrator("TEST", 1).unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn delete_project_administrator_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/TEST/administrators");
            then.status(200).json_body(project_user_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let user = client.delete_project_administrator("TEST", 1).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
    }

    #[test]
    fn delete_project_administrator_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/TEST/administrators");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No user"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.delete_project_administrator("TEST", 1).unwrap_err();
        assert!(err.to_string().contains("No user"));
    }

    fn custom_field_json() -> serde_json::Value {
        json!({
            "id": 1,
            "typeId": 6,
            "name": "Priority",
            "description": null,
            "required": false,
            "applicableIssueTypes": [],
            "allowAddItem": true,
            "items": []
        })
    }

    #[test]
    fn get_project_custom_fields_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/customFields");
            then.status(200).json_body(json!([custom_field_json()]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let fields = client.get_project_custom_fields("TEST").unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "Priority");
        assert_eq!(fields[0].type_id, 6);
    }

    #[test]
    fn add_project_custom_field_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects/TEST/customFields");
            then.status(200).json_body(custom_field_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let field = client
            .add_project_custom_field("TEST", 6, "Priority", None, None)
            .unwrap();
        assert_eq!(field.id, 1);
        assert_eq!(field.name, "Priority");
    }

    #[test]
    fn add_project_custom_field_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects/TEST/customFields");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client
            .add_project_custom_field("TEST", 6, "Priority", None, None)
            .unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn update_project_custom_field_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path("/projects/TEST/customFields/1");
            then.status(200).json_body(custom_field_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let field = client
            .update_project_custom_field("TEST", 1, Some("Priority"), None, None)
            .unwrap();
        assert_eq!(field.id, 1);
        assert_eq!(field.name, "Priority");
    }

    #[test]
    fn delete_project_custom_field_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/TEST/customFields/1");
            then.status(200).json_body(custom_field_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let field = client.delete_project_custom_field("TEST", 1).unwrap();
        assert_eq!(field.id, 1);
        assert_eq!(field.name, "Priority");
    }

    #[test]
    fn add_project_custom_field_item_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST)
                .path("/projects/TEST/customFields/1/items");
            then.status(200).json_body(custom_field_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let field = client
            .add_project_custom_field_item("TEST", 1, "High")
            .unwrap();
        assert_eq!(field.id, 1);
    }

    #[test]
    fn update_project_custom_field_item_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path("/projects/TEST/customFields/1/items/10");
            then.status(200).json_body(custom_field_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let field = client
            .update_project_custom_field_item("TEST", 1, 10, "Very High")
            .unwrap();
        assert_eq!(field.id, 1);
    }

    #[test]
    fn delete_project_custom_field_item_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE)
                .path("/projects/TEST/customFields/1/items/10");
            then.status(200).json_body(custom_field_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let field = client
            .delete_project_custom_field_item("TEST", 1, 10)
            .unwrap();
        assert_eq!(field.id, 1);
    }

    fn webhook_user_json() -> serde_json::Value {
        json!({"id": 1, "userId": "admin", "name": "Admin", "roleType": 1, "lang": null, "mailAddress": "admin@example.com"})
    }

    fn webhook_json() -> serde_json::Value {
        json!({
            "id": 1,
            "name": "My Webhook",
            "description": "webhook desc",
            "hookUrl": "https://example.com/hook",
            "allEvent": false,
            "activityTypeIds": [1, 2],
            "createdUser": webhook_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": webhook_user_json(),
            "updated": "2024-06-01T00:00:00Z"
        })
    }

    #[test]
    fn get_project_webhooks_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/webhooks");
            then.status(200).json_body(json!([webhook_json()]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let hooks = client.get_project_webhooks("TEST").unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].id, 1);
        assert_eq!(hooks[0].name, "My Webhook");
    }

    #[test]
    fn get_project_webhooks_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/webhooks");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_project_webhooks("TEST").unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn get_project_webhook_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/webhooks/1");
            then.status(200).json_body(webhook_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let hook = client.get_project_webhook("TEST", 1).unwrap();
        assert_eq!(hook.id, 1);
        assert_eq!(hook.hook_url, "https://example.com/hook");
    }

    #[test]
    fn add_project_webhook_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/projects/TEST/webhooks");
            then.status(200).json_body(webhook_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let hook = client
            .add_project_webhook(
                "TEST",
                "My Webhook",
                "https://example.com/hook",
                None,
                None,
                &[],
            )
            .unwrap();
        assert_eq!(hook.id, 1);
        assert_eq!(hook.name, "My Webhook");
    }

    #[test]
    fn update_project_webhook_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path("/projects/TEST/webhooks/1");
            then.status(200).json_body(webhook_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let hook = client
            .update_project_webhook(
                "TEST",
                1,
                &crate::api::project::UpdateProjectWebhookParams {
                    name: Some("New Name"),
                    hook_url: None,
                    description: None,
                    all_event: None,
                    activity_type_ids: None,
                },
            )
            .unwrap();
        assert_eq!(hook.id, 1);
    }

    #[test]
    fn delete_project_webhook_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/projects/TEST/webhooks/1");
            then.status(200).json_body(webhook_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let hook = client.delete_project_webhook("TEST", 1).unwrap();
        assert_eq!(hook.id, 1);
    }
}
