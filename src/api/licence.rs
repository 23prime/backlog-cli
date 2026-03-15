use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Licence {
    pub start_date: String,
    pub contract_type: Option<String>,
    pub storage_limit: u64,
    pub storage_usage: u64,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl BacklogClient {
    pub fn get_space_licence(&self) -> Result<Licence> {
        let value = self.get("/space/licence")?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize space licence response: {}\nRaw JSON:\n{}",
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

    fn licence_json() -> serde_json::Value {
        json!({
            "startDate": "2020-01-01",
            "contractType": "premium",
            "storageLimit": 1073741824u64,
            "storageUsage": 5242880u64,
            "active": true,
            "nulabAccount": false
        })
    }

    #[test]
    fn get_space_licence_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/licence");
            then.status(200).json_body(licence_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let l = client.get_space_licence().unwrap();
        assert_eq!(l.start_date, "2020-01-01");
        assert_eq!(l.contract_type, Some("premium".to_string()));
        assert_eq!(l.storage_limit, 1073741824);
        assert_eq!(l.storage_usage, 5242880);
    }

    #[test]
    fn get_space_licence_with_null_contract_type() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/licence");
            then.status(200).json_body(json!({
                "startDate": "2020-01-01",
                "contractType": null,
                "storageLimit": 1073741824u64,
                "storageUsage": 0u64
            }));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let l = client.get_space_licence().unwrap();
        assert_eq!(l.contract_type, None);
    }

    #[test]
    fn get_space_licence_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/licence");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_space_licence().unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }
}
