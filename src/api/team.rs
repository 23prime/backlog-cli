use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    pub id: u64,
    /// `null` for bot accounts.
    pub user_id: Option<String>,
    pub name: String,
    pub role_type: u8,
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default)]
    pub mail_address: Option<String>,
    #[serde(default)]
    pub last_login_time: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: u64,
    pub name: String,
    pub members: Vec<TeamMember>,
    pub display_order: Option<u64>,
    pub created: String,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BacklogClient {
    pub fn get_teams(&self, params: &[(String, String)]) -> Result<Vec<Team>> {
        let value = self.get_with_query("/teams", params)?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize teams response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn get_team(&self, team_id: u64) -> Result<Team> {
        let value = self.get(&format!("/teams/{team_id}"))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize team response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn add_team(&self, params: &[(String, String)]) -> Result<Team> {
        let value = self.post_form("/teams", params)?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize team response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn update_team(&self, team_id: u64, params: &[(String, String)]) -> Result<Team> {
        let value = self.patch_form(&format!("/teams/{team_id}"), params)?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize team response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn delete_team(&self, team_id: u64) -> Result<Team> {
        let value = self.delete_req(&format!("/teams/{team_id}"))?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize team response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::Method::PATCH;
    use httpmock::prelude::*;
    use serde_json::json;

    fn team_json() -> serde_json::Value {
        json!({
            "id": 1,
            "name": "dev-team",
            "members": [
                {
                    "id": 2,
                    "userId": "developer",
                    "name": "Developer",
                    "roleType": 2,
                    "lang": null,
                    "mailAddress": "dev@example.com",
                    "lastLoginTime": "2024-06-01T00:00:00Z"
                }
            ],
            "displayOrder": null,
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-06-01T00:00:00Z"
        })
    }

    #[test]
    fn get_teams_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/teams");
            then.status(200).json_body(json!([team_json()]));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let teams = client.get_teams(&[]).unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].id, 1);
        assert_eq!(teams[0].name, "dev-team");
    }

    #[test]
    fn get_teams_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/teams");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_teams(&[]).unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn get_team_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/teams/1");
            then.status(200).json_body(team_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let team = client.get_team(1).unwrap();
        assert_eq!(team.id, 1);
        assert_eq!(team.name, "dev-team");
        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].name, "Developer");
    }

    #[test]
    fn get_team_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/teams/999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No team"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_team(999).unwrap_err();
        assert!(err.to_string().contains("No team"));
    }

    #[test]
    fn deserialize_team_with_null_display_order() {
        let v = team_json();
        let team: Team = serde_json::from_value(v).unwrap();
        assert_eq!(team.display_order, None);
        assert_eq!(team.members[0].lang, None);
    }

    #[test]
    fn add_team_returns_created_team() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/teams");
            then.status(201).json_body(team_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let params = vec![("name".to_string(), "dev-team".to_string())];
        let team = client.add_team(&params).unwrap();
        assert_eq!(team.id, 1);
        assert_eq!(team.name, "dev-team");
    }

    #[test]
    fn add_team_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/teams");
            then.status(403)
                .json_body(json!({"errors": [{"message": "Forbidden"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let params = vec![("name".to_string(), "dev-team".to_string())];
        let err = client.add_team(&params).unwrap_err();
        assert!(err.to_string().contains("Forbidden"));
    }

    #[test]
    fn update_team_returns_updated_team() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(PATCH).path("/teams/1");
            then.status(200).json_body(team_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let params = vec![("name".to_string(), "new-name".to_string())];
        let team = client.update_team(1, &params).unwrap();
        assert_eq!(team.id, 1);
    }

    #[test]
    fn update_team_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(PATCH).path("/teams/999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No team"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let params = vec![("name".to_string(), "new-name".to_string())];
        let err = client.update_team(999, &params).unwrap_err();
        assert!(err.to_string().contains("No team"));
    }

    #[test]
    fn delete_team_returns_deleted_team() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/teams/1");
            then.status(200).json_body(team_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let team = client.delete_team(1).unwrap();
        assert_eq!(team.id, 1);
        assert_eq!(team.name, "dev-team");
    }

    #[test]
    fn delete_team_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/teams/999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No team"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.delete_team(999).unwrap_err();
        assert!(err.to_string().contains("No team"));
    }
}
