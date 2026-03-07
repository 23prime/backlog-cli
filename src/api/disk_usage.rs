use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskUsage {
    pub capacity: u64,
    pub issue: u64,
    pub wiki: u64,
    pub file: u64,
    pub subversion: u64,
    pub git: u64,
    #[serde(rename = "gitLFS")]
    pub git_lfs: u64,
    pub details: Vec<DiskUsageDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskUsageDetail {
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
            "capacity": 5242880,
            "issue": 2048,
            "wiki": 512,
            "file": 1024,
            "subversion": 64,
            "git": 256,
            "gitLFS": 128,
            "details": [{
                "projectId": 1,
                "issue": 1024,
                "wiki": 256,
                "document": 0,
                "file": 512,
                "subversion": 32,
                "git": 128,
                "gitLFS": 64
            }]
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
        assert_eq!(usage.capacity, 5242880);
        assert_eq!(usage.file, 1024);
        assert_eq!(usage.git_lfs, 128);
        assert_eq!(usage.details.len(), 1);
        assert_eq!(usage.details[0].project_id, 1);
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
