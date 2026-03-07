use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskUsage {
    pub space_size: u64,
    pub file_size: u64,
    pub wiki_size: u64,
    pub git_size: u64,
    pub git_lfs_size: u64,
    pub svn_size: u64,
    pub issue_size: u64,
}

impl BacklogClient {
    pub fn get_space_disk_usage(&self) -> Result<DiskUsage> {
        let value = self.get("/space/diskUsage")?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize disk usage response: {}\nRaw JSON:\n{}",
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

    fn disk_usage_json() -> serde_json::Value {
        json!({
            "spaceSize": 5242880,
            "fileSize": 1024,
            "wikiSize": 512,
            "gitSize": 256,
            "gitLfsSize": 128,
            "svnSize": 64,
            "issueSize": 2048
        })
    }

    #[test]
    fn get_space_disk_usage_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/diskUsage");
            then.status(200).json_body(disk_usage_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let usage = client.get_space_disk_usage().unwrap();
        assert_eq!(usage.space_size, 5242880);
        assert_eq!(usage.file_size, 1024);
        assert_eq!(usage.git_lfs_size, 128);
    }

    #[test]
    fn get_space_disk_usage_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/diskUsage");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_space_disk_usage().unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }
}
