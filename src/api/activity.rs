use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: u64,
    pub project: Option<ActivityProject>,
    #[serde(rename = "type")]
    pub activity_type: u32,
    pub content: serde_json::Value,
    pub created_user: ActivityUser,
    pub created: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityProject {
    pub id: u64,
    pub project_key: String,
    pub name: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityUser {
    pub id: u64,
    pub user_id: Option<String>,
    pub name: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

pub(crate) fn format_activity_row(a: &Activity) -> String {
    let project = a
        .project
        .as_ref()
        .map(|p| p.project_key.as_str())
        .unwrap_or("-");
    format!(
        "[{}] type={} project={} user={} created={}",
        a.id, a.activity_type, project, a.created_user.name, a.created,
    )
}

impl BacklogClient {
    pub fn get_space_activities(&self, params: &[(String, String)]) -> Result<Vec<Activity>> {
        let value = self.get_with_query("/space/activities", params)?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    fn activity_json() -> serde_json::Value {
        json!([{
            "id": 1,
            "project": {
                "id": 10,
                "projectKey": "TEST",
                "name": "Test Project"
            },
            "type": 1,
            "content": {"summary": "Fix bug"},
            "createdUser": {
                "id": 100,
                "userId": "john",
                "name": "John Doe"
            },
            "created": "2024-01-01T00:00:00Z"
        }])
    }

    #[test]
    fn get_space_activities_returns_parsed_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/activities");
            then.status(200).json_body(activity_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let activities = client.get_space_activities(&[]).unwrap();
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].id, 1);
        assert_eq!(activities[0].activity_type, 1);
        assert_eq!(activities[0].created_user.user_id.as_deref(), Some("john"));
    }

    #[test]
    fn get_space_activities_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/activities");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_space_activities(&[]).unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }

    #[test]
    fn deserialize_activity_with_null_user_id() {
        let v = json!({
            "id": 3,
            "type": 2,
            "content": {},
            "createdUser": {"id": 0, "userId": null, "name": "Bot"},
            "created": "2024-01-01T00:00:00Z"
        });
        let activity: Activity = serde_json::from_value(v).unwrap();
        assert!(activity.created_user.user_id.is_none());
        assert_eq!(activity.created_user.name, "Bot");
    }

    #[test]
    fn deserialize_activity_without_project() {
        let v = json!({
            "id": 2,
            "type": 5,
            "content": {},
            "createdUser": {"id": 1, "userId": "admin", "name": "Admin"},
            "created": "2024-01-01T00:00:00Z"
        });
        let activity: Activity = serde_json::from_value(v).unwrap();
        assert_eq!(activity.id, 2);
        assert!(activity.project.is_none());
    }
}
