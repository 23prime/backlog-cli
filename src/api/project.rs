use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::BacklogClient;

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
}
