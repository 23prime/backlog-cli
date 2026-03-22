use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub id: u64,
    pub name: String,
}

impl BacklogClient {
    pub fn get_resolutions(&self) -> Result<Vec<Resolution>> {
        let value = self.get_with_query("/resolutions", &[])?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn get_resolutions_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/resolutions");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!([
                    {"id": 0, "name": "Fixed"},
                    {"id": 1, "name": "Won't Fix"},
                    {"id": 2, "name": "Invalid"}
                ]));
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let result = client.get_resolutions().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, 0);
        assert_eq!(result[0].name, "Fixed");
    }
}
