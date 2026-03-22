use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient, issue::IssueCommentNotification};

pub struct IssueCommentNotificationListArgs {
    key: String,
    comment_id: u64,
    json: bool,
}

impl IssueCommentNotificationListArgs {
    pub fn new(key: String, comment_id: u64, json: bool) -> Self {
        Self {
            key,
            comment_id,
            json,
        }
    }
}

pub fn list(args: &IssueCommentNotificationListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &IssueCommentNotificationListArgs, api: &dyn BacklogApi) -> Result<()> {
    let notifications = api.get_issue_comment_notifications(&args.key, args.comment_id)?;
    if args.json {
        crate::cmd::print_json(&notifications)?;
    } else {
        for n in &notifications {
            println!("{}", format_notification_row(n));
        }
    }
    Ok(())
}

pub fn format_notification_row(n: &IssueCommentNotification) -> String {
    format!("[{}] {}", n.id, n.user.name)
}

#[cfg(test)]
use crate::api::issue::IssueUser;
#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
pub(crate) fn sample_notification() -> IssueCommentNotification {
    IssueCommentNotification {
        id: 1,
        already_read: false,
        reason: 2,
        user: IssueUser {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        },
        resource_already_read: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        notifications: Option<Vec<IssueCommentNotification>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_issue_comment_notifications(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<Vec<IssueCommentNotification>> {
            self.notifications
                .clone()
                .ok_or_else(|| anyhow!("no notifications"))
        }
    }

    fn args(json: bool) -> IssueCommentNotificationListArgs {
        IssueCommentNotificationListArgs::new("TEST-1".to_string(), 1, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            notifications: Some(vec![sample_notification()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            notifications: Some(vec![sample_notification()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi {
            notifications: None,
        };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no notifications"));
    }

    #[test]
    fn format_notification_row_contains_id_and_name() {
        let row = format_notification_row(&sample_notification());
        assert!(row.contains('1'));
        assert!(row.contains("John Doe"));
    }
}
