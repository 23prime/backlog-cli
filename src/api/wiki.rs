use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use crate::api::user::Star;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiCount {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiSharedFile {
    pub id: u64,
    pub dir: String,
    pub name: String,
    pub size: u64,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

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
    serde_json::from_value(value)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize response: {}", e))
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

    pub fn get_wiki_count(&self, params: &[(String, String)]) -> Result<WikiCount> {
        let value = self.get_with_query("/wikis/count", params)?;
        deserialize(value)
    }

    pub fn get_wiki_tags(&self, params: &[(String, String)]) -> Result<Vec<WikiTag>> {
        let value = self.get_with_query("/wikis/tags", params)?;
        deserialize(value)
    }

    pub fn get_wiki_stars(&self, wiki_id: u64) -> Result<Vec<Star>> {
        let value = self.get(&format!("/wikis/{}/stars", wiki_id))?;
        deserialize(value)
    }

    pub fn add_wiki_attachments(
        &self,
        wiki_id: u64,
        attachment_ids: &[u64],
    ) -> Result<Vec<WikiAttachment>> {
        let params: Vec<(String, String)> = attachment_ids
            .iter()
            .map(|id| ("attachmentId[]".to_string(), id.to_string()))
            .collect();
        let value = self.post_form(&format!("/wikis/{}/attachments", wiki_id), &params)?;
        deserialize(value)
    }

    pub fn download_wiki_attachment(
        &self,
        wiki_id: u64,
        attachment_id: u64,
    ) -> Result<(Vec<u8>, String)> {
        self.download(&format!("/wikis/{}/attachments/{}", wiki_id, attachment_id))
    }

    pub fn delete_wiki_attachment(
        &self,
        wiki_id: u64,
        attachment_id: u64,
    ) -> Result<WikiAttachment> {
        let value =
            self.delete_req(&format!("/wikis/{}/attachments/{}", wiki_id, attachment_id))?;
        deserialize(value)
    }

    pub fn get_wiki_shared_files(&self, wiki_id: u64) -> Result<Vec<WikiSharedFile>> {
        let value = self.get(&format!("/wikis/{}/sharedFiles", wiki_id))?;
        deserialize(value)
    }

    pub fn link_wiki_shared_files(
        &self,
        wiki_id: u64,
        shared_file_ids: &[u64],
    ) -> Result<Vec<WikiSharedFile>> {
        let params: Vec<(String, String)> = shared_file_ids
            .iter()
            .map(|id| ("fileId[]".to_string(), id.to_string()))
            .collect();
        let value = self.post_form(&format!("/wikis/{}/sharedFiles", wiki_id), &params)?;
        deserialize(value)
    }

    pub fn unlink_wiki_shared_file(
        &self,
        wiki_id: u64,
        shared_file_id: u64,
    ) -> Result<WikiSharedFile> {
        let value = self.delete_req(&format!(
            "/wikis/{}/sharedFiles/{}",
            wiki_id, shared_file_id
        ))?;
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

    fn shared_file_json() -> serde_json::Value {
        json!({
            "id": 1,
            "dir": "/docs",
            "name": "spec.pdf",
            "size": 2048_u64
        })
    }

    fn star_json() -> serde_json::Value {
        json!({
            "id": 1,
            "comment": null,
            "url": "https://example.backlog.com/wiki/TEST/Home",
            "title": "Home",
            "presenter": {
                "id": 1, "userId": "john", "name": "John Doe",
                "roleType": 1, "mailAddress": "john@example.com"
            },
            "created": "2024-01-01T00:00:00Z"
        })
    }

    #[test]
    fn get_wiki_count_returns_count() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis/count")
                .query_param("apiKey", TEST_KEY)
                .query_param("projectIdOrKey", "TEST");
            then.status(200).json_body(json!({"count": 42}));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let result = client
            .get_wiki_count(&[("projectIdOrKey".to_string(), "TEST".to_string())])
            .unwrap();
        assert_eq!(result.count, 42);
    }

    #[test]
    fn get_wiki_tags_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis/tags")
                .query_param("apiKey", TEST_KEY);
            then.status(200)
                .json_body(json!([{"id": 1, "name": "guide"}]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let tags = client.get_wiki_tags(&[]).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "guide");
    }

    #[test]
    fn get_wiki_stars_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis/1/stars")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([star_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let stars = client.get_wiki_stars(1).unwrap();
        assert_eq!(stars.len(), 1);
        assert_eq!(stars[0].title, "Home");
    }

    #[test]
    fn add_wiki_attachments_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST)
                .path("/wikis/1/attachments")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([attachment_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let attachments = client.add_wiki_attachments(1, &[1]).unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].name, "image.png");
    }

    #[test]
    fn download_wiki_attachment_returns_bytes() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/wikis/1/attachments/1");
            then.status(200)
                .header("Content-Disposition", "attachment; filename=\"image.png\"")
                .body(b"binarydata");
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let (bytes, filename) = client.download_wiki_attachment(1, 1).unwrap();
        assert_eq!(bytes, b"binarydata");
        assert_eq!(filename, "image.png");
    }

    #[test]
    fn delete_wiki_attachment_returns_attachment() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE)
                .path("/wikis/1/attachments/1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(attachment_json());
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let attachment = client.delete_wiki_attachment(1, 1).unwrap();
        assert_eq!(attachment.name, "image.png");
    }

    #[test]
    fn get_wiki_shared_files_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/wikis/1/sharedFiles")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([shared_file_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let files = client.get_wiki_shared_files(1).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "spec.pdf");
    }

    #[test]
    fn link_wiki_shared_files_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST)
                .path("/wikis/1/sharedFiles")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(json!([shared_file_json()]));
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let files = client.link_wiki_shared_files(1, &[1]).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "spec.pdf");
    }

    #[test]
    fn unlink_wiki_shared_file_returns_file() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE)
                .path("/wikis/1/sharedFiles/1")
                .query_param("apiKey", TEST_KEY);
            then.status(200).json_body(shared_file_json());
        });
        let client = super::super::BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let file = client.unlink_wiki_shared_file(1, 1).unwrap();
        assert_eq!(file.name, "spec.pdf");
    }
}
