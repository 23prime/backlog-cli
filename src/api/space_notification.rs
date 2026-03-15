use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceNotification {
    pub content: String,
    pub updated: Option<String>,
}

impl BacklogClient {
    pub fn get_space_notification(&self) -> Result<SpaceNotification> {
        let value = self.get("/space/notification")?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize space notification response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }

    pub fn put_space_notification(&self, content: &str) -> Result<SpaceNotification> {
        let params = vec![("content".to_string(), content.to_string())];
        let value = self.put_form("/space/notification", &params)?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize space notification response: {}\nRaw JSON:\n{}",
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

    fn notification_json() -> serde_json::Value {
        json!({
            "content": "Scheduled maintenance on 2024-07-01.",
            "updated": "2024-06-18T07:55:37Z"
        })
    }

    #[test]
    fn get_space_notification_returns_parsed_struct() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/notification");
            then.status(200).json_body(notification_json());
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let n = client.get_space_notification().unwrap();
        assert_eq!(n.content, "Scheduled maintenance on 2024-07-01.");
        assert_eq!(n.updated, Some("2024-06-18T07:55:37Z".to_string()));
    }

    #[test]
    fn get_space_notification_with_null_updated() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/notification");
            then.status(200).json_body(json!({
                "content": "",
                "updated": null
            }));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let n = client.get_space_notification().unwrap();
        assert_eq!(n.content, "");
        assert_eq!(n.updated, None);
    }

    #[test]
    fn put_space_notification_returns_updated_struct() {
        let server = MockServer::start();
        let put_mock = server.mock(|when, then| {
            when.method(PUT)
                .path("/space/notification")
                .body_contains("content=New+notification+text.");
            then.status(200).json_body(json!({
                "content": "New notification text.",
                "updated": "2024-07-01T00:00:00Z"
            }));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let n = client
            .put_space_notification("New notification text.")
            .unwrap();
        put_mock.assert();
        assert_eq!(n.content, "New notification text.");
        assert_eq!(n.updated, Some("2024-07-01T00:00:00Z".to_string()));
    }

    #[test]
    fn get_space_notification_returns_error_on_api_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space/notification");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
        let err = client.get_space_notification().unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }
}
