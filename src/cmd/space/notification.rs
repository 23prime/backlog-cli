use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, space_notification::SpaceNotification};

pub struct SpaceNotificationArgs {
    json: bool,
}

impl SpaceNotificationArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn notification(args: &SpaceNotificationArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    notification_with(args, &client)
}

pub fn notification_with(args: &SpaceNotificationArgs, api: &dyn BacklogApi) -> Result<()> {
    let n = api.get_space_notification()?;
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
    use crate::cmd::space::sample_notification;
    use anyhow::anyhow;

    struct MockApi {
        notification: Option<SpaceNotification>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space_notification(&self) -> Result<SpaceNotification> {
            self.notification
                .clone()
                .ok_or_else(|| anyhow!("no notification"))
        }
    }

    #[test]
    fn notification_with_text_output_succeeds() {
        let api = MockApi {
            notification: Some(sample_notification()),
        };
        assert!(notification_with(&SpaceNotificationArgs::new(false), &api).is_ok());
    }

    #[test]
    fn notification_with_json_output_succeeds() {
        let api = MockApi {
            notification: Some(sample_notification()),
        };
        assert!(notification_with(&SpaceNotificationArgs::new(true), &api).is_ok());
    }

    #[test]
    fn notification_with_propagates_api_error() {
        let api = MockApi { notification: None };
        let err = notification_with(&SpaceNotificationArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no notification"));
    }

    #[test]
    fn format_notification_text_contains_fields() {
        let text = format_notification_text(&sample_notification());
        assert!(text.contains("2024-06-18T07:55:37Z"));
        assert!(text.contains("Scheduled maintenance on 2024-07-01."));
    }

    #[test]
    fn format_notification_text_with_null_updated() {
        let n = SpaceNotification {
            content: "Hello".to_string(),
            updated: None,
        };
        let text = format_notification_text(&n);
        assert!(text.contains("(not set)"));
        assert!(text.contains("Hello"));
    }

    #[test]
    fn format_notification_text_with_empty_content() {
        let n = SpaceNotification {
            content: "".to_string(),
            updated: None,
        };
        let text = format_notification_text(&n);
        assert!(text.contains("(no notification set)"));
    }
}
