use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use crate::api::issue::Issue;

fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    let pretty = serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string());
    serde_json::from_value(value)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize response: {e}\nRaw JSON:\n{pretty}"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watching {
    pub id: u64,
    pub resource_already_read: bool,
    pub note: Option<String>,
    #[serde(rename = "type")]
    pub watching_type: String,
    pub issue: Issue,
    pub last_content_updated: Option<String>,
    pub created: String,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchingCount {
    pub count: u64,
}

impl BacklogClient {
    pub fn get_watchings(
        &self,
        user_id: u64,
        params: &[(String, String)],
    ) -> Result<Vec<Watching>> {
        let value = self.get_with_query(&format!("/users/{user_id}/watchings"), params)?;
        deserialize(value)
    }

    pub fn count_watchings(
        &self,
        user_id: u64,
        params: &[(String, String)],
    ) -> Result<WatchingCount> {
        let value = self.get_with_query(&format!("/users/{user_id}/watchings/count"), params)?;
        deserialize(value)
    }

    pub fn get_watching(&self, watching_id: u64) -> Result<Watching> {
        let value = self.get_with_query(&format!("/watchings/{watching_id}"), &[])?;
        deserialize(value)
    }

    pub fn add_watching(&self, params: &[(String, String)]) -> Result<Watching> {
        let value = self.post_form("/watchings", params)?;
        deserialize(value)
    }

    pub fn update_watching(
        &self,
        watching_id: u64,
        params: &[(String, String)],
    ) -> Result<Watching> {
        let value = self.patch_form(&format!("/watchings/{watching_id}"), params)?;
        deserialize(value)
    }

    pub fn delete_watching(&self, watching_id: u64) -> Result<Watching> {
        let value = self.delete_req(&format!("/watchings/{watching_id}"))?;
        deserialize(value)
    }

    pub fn read_watching(&self, watching_id: u64) -> Result<()> {
        self.post_form(&format!("/watchings/{watching_id}/markAsRead"), &[])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    const TEST_KEY: &str = "test-key";

    fn sample_issue_json() -> serde_json::Value {
        serde_json::json!({
            "id": 10,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": { "id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0 },
            "summary": "Test issue",
            "description": null,
            "resolution": null,
            "priority": { "id": 3, "name": "Normal" },
            "status": { "id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000 },
            "assignee": null,
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "createdUser": { "id": 1, "userId": "alice", "name": "Alice", "roleType": 1, "lang": null, "mailAddress": null, "lastLoginTime": null },
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": { "id": 1, "userId": "alice", "name": "Alice", "roleType": 1, "lang": null, "mailAddress": null, "lastLoginTime": null },
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    fn sample_watching_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "resourceAlreadyRead": false,
            "note": "watching this",
            "type": "issue",
            "issue": sample_issue_json(),
            "lastContentUpdated": "2024-01-01T00:00:00Z",
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    #[test]
    fn get_watchings_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/1/watchings");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!([sample_watching_json()]));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let result = client.get_watchings(1, &[]);
        assert!(result.is_ok());
        let watchings = result.unwrap();
        assert_eq!(watchings.len(), 1);
        assert_eq!(watchings[0].id, 1);
    }

    #[test]
    fn count_watchings_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/users/1/watchings/count");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({ "count": 5 }));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let result = client.count_watchings(1, &[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().count, 5);
    }

    #[test]
    fn get_watching_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/watchings/1");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(sample_watching_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let result = client.get_watching(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }

    #[test]
    fn add_watching_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/watchings");
            then.status(201)
                .header("content-type", "application/json")
                .json_body(sample_watching_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let params = vec![("issueIdOrKey".to_string(), "TEST-1".to_string())];
        let result = client.add_watching(&params);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }

    #[test]
    fn update_watching_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::PATCH).path("/watchings/1");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(sample_watching_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let params = vec![("note".to_string(), "updated note".to_string())];
        let result = client.update_watching(1, &params);
        assert!(result.is_ok());
    }

    #[test]
    fn delete_watching_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/watchings/1");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(sample_watching_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let result = client.delete_watching(1);
        assert!(result.is_ok());
    }

    #[test]
    fn read_watching_sends_post() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/watchings/1/markAsRead");
            then.status(204);
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        assert!(client.read_watching(1).is_ok());
    }
}
