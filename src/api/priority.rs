use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priority {
    pub id: u64,
    pub name: String,
}

impl BacklogClient {
    pub fn get_priorities(&self) -> Result<Vec<Priority>> {
        let value = self.get_with_query("/priorities", &[])?;
        deserialize(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn get_priorities_parses_response() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/priorities");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!([
                    {"id": 2, "name": "High"},
                    {"id": 3, "name": "Normal"},
                    {"id": 4, "name": "Low"}
                ]));
        });
        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let result = client.get_priorities().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, 2);
        assert_eq!(result[0].name, "High");
    }
}
