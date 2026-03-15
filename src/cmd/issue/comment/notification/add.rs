use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::comment::notification::list::format_notification_row;

pub struct IssueCommentNotificationAddArgs {
    key: String,
    comment_id: u64,
    notified_user_ids: Vec<u64>,
    json: bool,
}

impl IssueCommentNotificationAddArgs {
    pub fn new(key: String, comment_id: u64, notified_user_ids: Vec<u64>, json: bool) -> Self {
        Self {
            key,
            comment_id,
            notified_user_ids,
            json,
        }
    }
}

pub fn add(args: &IssueCommentNotificationAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &IssueCommentNotificationAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let params: Vec<(String, String)> = args
        .notified_user_ids
        .iter()
        .map(|id| ("notifiedUserId[]".to_string(), id.to_string()))
        .collect();
    let notifications = api.add_issue_comment_notifications(&args.key, args.comment_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&notifications).context("Failed to serialize JSON")?
        );
    } else {
        for n in &notifications {
            println!("{}", format_notification_row(n));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::IssueCommentNotification;
    use crate::cmd::issue::comment::notification::list::sample_notification;
    use anyhow::anyhow;

    struct MockApi {
        notifications: Option<Vec<IssueCommentNotification>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn add_issue_comment_notifications(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<IssueCommentNotification>> {
            self.notifications
                .clone()
                .ok_or_else(|| anyhow!("no notifications"))
        }
    }

    fn args(json: bool) -> IssueCommentNotificationAddArgs {
        IssueCommentNotificationAddArgs::new("TEST-1".to_string(), 1, vec![2], json)
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            notifications: Some(vec![sample_notification()]),
        };
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            notifications: Some(vec![sample_notification()]),
        };
        assert!(add_with(&args(true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi {
            notifications: None,
        };
        let err = add_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no notifications"));
    }
}
