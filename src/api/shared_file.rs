use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;
use crate::api::user::User;

/// Percent-encode a path for use in a URL, preserving `/` separators.
/// Each segment is encoded individually; unreserved characters are left as-is.
fn encode_path(path: &str) -> String {
    // Strip leading slash so callers can pass "/docs" or "docs" interchangeably.
    let path = path.trim_start_matches('/');
    path.split('/')
        .map(|segment| {
            segment
                .bytes()
                .flat_map(|b| {
                    if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
                        vec![b as char]
                    } else {
                        format!("%{b:02X}").chars().collect()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedFile {
    pub id: u64,
    pub project_id: u64,
    #[serde(rename = "type")]
    pub file_type: String,
    pub dir: String,
    pub name: String,
    /// `null` for directory entries.
    pub size: Option<u64>,
    pub created_user: User,
    pub created: String,
    pub updated_user: Option<User>,
    pub updated: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BacklogClient {
    pub fn list_shared_files(
        &self,
        project_id_or_key: &str,
        path: &str,
        params: &[(String, String)],
    ) -> Result<Vec<SharedFile>> {
        let encoded = encode_path(path);
        let value = self.get_with_query(
            &format!("/projects/{project_id_or_key}/files/metadata/{encoded}"),
            params,
        )?;
        deserialize(value)
    }

    pub fn download_shared_file(
        &self,
        project_id_or_key: &str,
        shared_file_id: u64,
    ) -> Result<(Vec<u8>, String)> {
        self.download(&format!(
            "/projects/{project_id_or_key}/files/{shared_file_id}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-key";

    fn shared_file_json() -> serde_json::Value {
        json!({
            "id": 1,
            "projectId": 10,
            "type": "file",
            "dir": "/",
            "name": "test.txt",
            "size": 1024_u64,
            "createdUser": {
                "id": 1, "userId": "admin", "name": "Admin", "roleType": 1
            },
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": null,
            "updated": "2024-06-01T00:00:00Z"
        })
    }

    #[test]
    fn list_shared_files_returns_list() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/files/metadata/");
            then.status(200).json_body(json!([shared_file_json()]));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let files = client.list_shared_files("TEST", "", &[]).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "test.txt");
        assert!(files[0].updated_user.is_none());
    }

    #[test]
    fn list_shared_files_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/files/metadata/");
            then.status(404)
                .json_body(json!({"errors": [{"message": "No project"}]}));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.list_shared_files("TEST", "", &[]).unwrap_err();
        assert!(err.to_string().contains("No project"));
    }

    #[test]
    fn download_shared_file_returns_bytes() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/files/1");
            then.status(200)
                .header("Content-Disposition", "attachment; filename=\"test.txt\"")
                .body(b"hello");
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let (bytes, filename) = client.download_shared_file("TEST", 1).unwrap();
        assert_eq!(bytes, b"hello");
        assert_eq!(filename, "test.txt");
    }

    #[test]
    fn download_shared_file_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/projects/TEST/files/99");
            then.status(404)
                .json_body(json!({"errors": [{"message": "Not found"}]}));
        });
        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.download_shared_file("TEST", 99).unwrap_err();
        assert!(err.to_string().contains("Not found"));
    }

    #[test]
    fn encode_path_encodes_non_ascii() {
        assert_eq!(encode_path("非機能"), "%E9%9D%9E%E6%A9%9F%E8%83%BD");
    }

    #[test]
    fn encode_path_preserves_slash_separators() {
        assert_eq!(encode_path("docs/api"), "docs/api");
    }

    #[test]
    fn encode_path_strips_leading_slash() {
        assert_eq!(encode_path("/docs"), "docs");
    }

    #[test]
    fn encode_path_encodes_spaces() {
        assert_eq!(encode_path("my docs"), "my%20docs");
    }

    #[test]
    fn encode_path_empty_is_empty() {
        assert_eq!(encode_path(""), "");
    }
}
