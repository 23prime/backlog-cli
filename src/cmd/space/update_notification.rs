use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, space_notification::SpaceNotification};

pub struct SpaceUpdateNotificationArgs {
    pub content: String,
    json: bool,
}

impl SpaceUpdateNotificationArgs {
    pub fn new(content: String, json: bool) -> Self {
        Self { content, json }
    }
}

pub fn update_notification(args: &SpaceUpdateNotificationArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_notification_with(args, &client)
}

pub fn update_notification_with(
    args: &SpaceUpdateNotificationArgs,
    api: &dyn BacklogApi,
) -> Result<()> {
    let n = api.put_space_notification(&args.content)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&n).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_notification_text(&n));
    }
    Ok(())
}

fn format_notification_text(n: &SpaceNotification) -> String {
    let updated = n.updated.as_deref().unwrap_or("(not set)");
    let content = if n.content.trim().is_empty() {
        "(no notification set)"
    } else {
        n.content.as_str()
    };
    format!("Updated: {}\n\n{}", updated, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    use std::cell::RefCell;

    struct MockApi {
        result: Option<SpaceNotification>,
        captured_content: RefCell<Option<String>>,
    }

    impl MockApi {
        fn new(result: Option<SpaceNotification>) -> Self {
            Self {
                result,
                captured_content: RefCell::new(None),
            }
        }
    }

    impl crate::api::BacklogApi for MockApi {
        fn put_space_notification(&self, content: &str) -> Result<SpaceNotification> {
            *self.captured_content.borrow_mut() = Some(content.to_string());
            self.result
                .clone()
                .ok_or_else(|| anyhow!("put notification failed"))
        }
    }

    fn sample_notification() -> SpaceNotification {
        SpaceNotification {
            content: "Hello world.".to_string(),
            updated: Some("2024-07-01T00:00:00Z".to_string()),
        }
    }

    #[test]
    fn update_notification_with_text_output_succeeds() {
        let api = MockApi::new(Some(sample_notification()));
        assert!(
            update_notification_with(
                &SpaceUpdateNotificationArgs::new("Hello world.".to_string(), false),
                &api
            )
            .is_ok()
        );
        assert_eq!(
            api.captured_content.borrow().as_deref(),
            Some("Hello world.")
        );
    }

    #[test]
    fn update_notification_with_json_output_succeeds() {
        let api = MockApi::new(Some(sample_notification()));
        assert!(
            update_notification_with(
                &SpaceUpdateNotificationArgs::new("Hello world.".to_string(), true),
                &api
            )
            .is_ok()
        );
        assert_eq!(
            api.captured_content.borrow().as_deref(),
            Some("Hello world.")
        );
    }

    #[test]
    fn update_notification_with_propagates_api_error() {
        let api = MockApi::new(None);
        let err = update_notification_with(
            &SpaceUpdateNotificationArgs::new("text".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("put notification failed"));
    }

    #[test]
    fn format_notification_text_contains_fields() {
        let text = format_notification_text(&sample_notification());
        assert!(text.contains("2024-07-01T00:00:00Z"));
        assert!(text.contains("Hello world."));
    }
}
