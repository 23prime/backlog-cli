use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiUser {
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
pub struct WikiTag {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiListItem {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub tags: Vec<WikiTag>,
    pub created_user: WikiUser,
    pub created: String,
    pub updated_user: WikiUser,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wiki {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub content: String,
    pub tags: Vec<WikiTag>,
    pub created_user: WikiUser,
    pub created: String,
    pub updated_user: WikiUser,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiHistory {
    pub page_id: u64,
    pub version: u64,
    pub name: String,
    pub content: String,
    pub created_user: WikiUser,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiAttachment {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub created_user: WikiUser,
    pub created: String,
}

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

impl BacklogClient {
    pub fn get_wikis(&self, params: &[(String, String)]) -> Result<Vec<WikiListItem>> {
        let value = self.get_with_query("/wikis", params)?;
        deserialize(value)
    }

    pub fn get_wiki(&self, wiki_id: u64) -> Result<Wiki> {
        let value = self.get(&format!("/wikis/{}", wiki_id))?;
        deserialize(value)
    }

    pub fn create_wiki(&self, params: &[(String, String)]) -> Result<Wiki> {
        let value = self.post_form("/wikis", params)?;
        deserialize(value)
    }

    pub fn update_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki> {
        let value = self.patch_form(&format!("/wikis/{}", wiki_id), params)?;
        deserialize(value)
    }

    pub fn delete_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki> {
        let value = self.delete_form(&format!("/wikis/{}", wiki_id), params)?;
        deserialize(value)
    }

    pub fn get_wiki_history(&self, wiki_id: u64) -> Result<Vec<WikiHistory>> {
        let value = self.get(&format!("/wikis/{}/history", wiki_id))?;
        deserialize(value)
    }

    pub fn get_wiki_attachments(&self, wiki_id: u64) -> Result<Vec<WikiAttachment>> {
        let value = self.get(&format!("/wikis/{}/attachments", wiki_id))?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-api-key";

    fn wiki_user_json() -> serde_json::Value {
        json!({
            "id": 1,
            "userId": "john",
            "name": "John Doe",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "john@example.com"
        })
    }

    fn wiki_list_item_json() -> serde_json::Value {
        json!({
            "id": 1,
            "projectId": 1,
            "name": "Home",
            "tags": [],
            "createdUser": wiki_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": wiki_user_json(),
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    fn wiki_json() -> serde_json::Value {
        json!({
            "id": 1,
            "projectId": 1,
            "name": "Home",
            "content": "# Home\nWelcome!",
            "tags": [],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": wiki_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": wiki_user_json(),
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    fn history_json() -> serde_json::Value {
        json!({
            "pageId": 1,
            "version": 1,
            "name": "Home",
            "content": "# Home",
            "createdUser": wiki_user_json(),
            "created": "2024-01-01T00:00:00Z"
        })
    }

    fn attachment_json() -> serde_json::Value {
        json!({
            "id": 1,
            "name": "image.png",
            "size": 2048,
            "createdUser": wiki_user_json(),
            "created": "2024-01-01T00:00:00Z"
        })
    }

    #[test]
    fn get_wikis_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([wiki_list_item_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let wikis = client.get_wikis(&[]).unwrap();
        assert_eq!(wikis.len(), 1);
        assert_eq!(wikis[0].name, "Home");
    }

    #[test]
    fn get_wiki_returns_single() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis/1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(wiki_json());
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let wiki = client.get_wiki(1).unwrap();
        assert_eq!(wiki.name, "Home");
        assert_eq!(wiki.content, "# Home\nWelcome!");
    }

    #[test]
    fn get_wiki_returns_error_on_not_found() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/wikis/999");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No wiki page"}]}));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get_wiki(999).unwrap_err();
        assert!(err.to_string().contains("No wiki page"));
    }

    #[test]
    fn get_wiki_history_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis/1/history")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([history_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let history = client.get_wiki_history(1).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].version, 1);
    }

    #[test]
    fn get_wiki_attachments_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis/1/attachments")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([attachment_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let attachments = client.get_wiki_attachments(1).unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].name, "image.png");
    }

    #[test]
    fn wiki_with_null_user_id_deserializes() {
        let v = json!({
            "id": 1,
            "projectId": 1,
            "name": "Home",
            "content": "# Home",
            "tags": [],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
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
        let wiki: Wiki = serde_json::from_value(v).unwrap();
        assert!(wiki.created_user.user_id.is_none());
    }
}
