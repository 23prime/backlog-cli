use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUser {
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
pub struct DocumentAttachment {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub created_user: DocumentUser,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTag {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub project_id: u64,
    pub title: String,
    pub plain: Option<String>,
    pub status_id: u64,
    pub emoji: Option<String>,
    #[serde(default)]
    pub attachments: Vec<DocumentAttachment>,
    #[serde(default)]
    pub tags: Vec<DocumentTag>,
    pub created_user: DocumentUser,
    pub created: String,
    pub updated_user: DocumentUser,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTreeNode {
    pub id: String,
    pub name: Option<String>,
    pub emoji: Option<String>,
    #[serde(default)]
    pub children: Vec<DocumentTreeNode>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTree {
    pub project_id: u64,
    pub active_tree: DocumentTreeNode,
    pub trash_tree: DocumentTreeNode,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to deserialize document response: {}\nRaw JSON:\n{}",
            e,
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
        )
    })
}

impl BacklogClient {
    pub fn get_documents(&self, params: &[(String, String)]) -> Result<Vec<Document>> {
        let value = self.get_with_query("/documents", params)?;
        deserialize(value)
    }

    pub fn get_document_tree(&self, params: &[(String, String)]) -> Result<DocumentTree> {
        let value = self.get_with_query("/documents/tree", params)?;
        deserialize(value)
    }

    pub fn get_document(&self, document_id: &str) -> Result<Document> {
        let value = self.get(&format!("/documents/{document_id}"))?;
        deserialize(value)
    }

    pub fn create_document(&self, params: &[(String, String)]) -> Result<Document> {
        let value = self.post_form("/documents", params)?;
        deserialize(value)
    }

    pub fn delete_document(&self, document_id: &str) -> Result<Document> {
        let value = self.delete_req(&format!("/documents/{document_id}"))?;
        deserialize(value)
    }

    pub fn download_document_attachment(
        &self,
        document_id: &str,
        attachment_id: u64,
    ) -> Result<(Vec<u8>, String)> {
        self.download(&format!(
            "/documents/{document_id}/attachments/{attachment_id}"
        ))
    }
}

#[cfg(test)]
pub mod tests_helper {
    use super::*;
    use std::collections::BTreeMap;

    pub fn sample_document_user() -> DocumentUser {
        DocumentUser {
            id: 1,
            user_id: Some("taro".to_string()),
            name: "Taro".to_string(),
            role_type: 1,
            lang: Some("ja".to_string()),
            mail_address: Some("taro@example.com".to_string()),
            extra: BTreeMap::new(),
        }
    }

    pub fn sample_document() -> Document {
        Document {
            id: "abc123".to_string(),
            project_id: 1,
            title: "My Doc".to_string(),
            plain: Some("Hello world".to_string()),
            status_id: 1,
            emoji: None,
            attachments: vec![],
            tags: vec![],
            created_user: sample_document_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_document_user(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    fn document_user_json() -> serde_json::Value {
        json!({
            "id": 1,
            "userId": "taro",
            "name": "Taro",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "taro@example.com"
        })
    }

    pub fn document_json() -> serde_json::Value {
        json!({
            "id": "abc123",
            "projectId": 1,
            "title": "My Doc",
            "plain": "Hello world",
            "statusId": 1,
            "emoji": null,
            "attachments": [],
            "tags": [],
            "createdUser": document_user_json(),
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": document_user_json(),
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    fn tree_json() -> serde_json::Value {
        json!({
            "projectId": 1,
            "activeTree": {
                "id": "root",
                "name": null,
                "emoji": null,
                "children": [
                    {
                        "id": "child1",
                        "name": "Chapter 1",
                        "emoji": "📖",
                        "children": []
                    }
                ]
            },
            "trashTree": {
                "id": "trash",
                "name": null,
                "emoji": null,
                "children": []
            }
        })
    }

    #[test]
    fn get_documents_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/documents");
            then.status(200).json_body(json!([document_json()]));
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let docs = client.get_documents(&[]).unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].id, "abc123");
        assert_eq!(docs[0].title, "My Doc");
    }

    #[test]
    fn get_document_tree_returns_tree() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/documents/tree");
            then.status(200).json_body(tree_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let tree = client
            .get_document_tree(&[("projectIdOrKey".to_string(), "TEST".to_string())])
            .unwrap();
        assert_eq!(tree.project_id, 1);
        assert_eq!(tree.active_tree.children.len(), 1);
        assert_eq!(
            tree.active_tree.children[0].name.as_deref(),
            Some("Chapter 1")
        );
    }

    #[test]
    fn get_document_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/documents/abc123");
            then.status(200).json_body(document_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let doc = client.get_document("abc123").unwrap();
        assert_eq!(doc.id, "abc123");
        assert_eq!(doc.title, "My Doc");
    }

    #[test]
    fn create_document_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/documents");
            then.status(200).json_body(document_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let doc = client
            .create_document(&[
                ("projectId".to_string(), "1".to_string()),
                ("title".to_string(), "My Doc".to_string()),
            ])
            .unwrap();
        assert_eq!(doc.id, "abc123");
    }

    #[test]
    fn delete_document_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(DELETE).path("/documents/abc123");
            then.status(200).json_body(document_json());
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let doc = client.delete_document("abc123").unwrap();
        assert_eq!(doc.id, "abc123");
    }

    #[test]
    fn download_document_attachment_returns_bytes() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/documents/abc123/attachments/1");
            then.status(200)
                .header("Content-Disposition", "attachment; filename=\"file.txt\"")
                .body("hello");
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let (bytes, filename) = client.download_document_attachment("abc123", 1).unwrap();
        assert_eq!(bytes, b"hello");
        assert_eq!(filename, "file.txt");
    }
}
