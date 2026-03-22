use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::comment::notification::list::format_notification_row;

#[derive(Debug)]
pub struct IssueCommentNotificationAddArgs {
    key: String,
    comment_id: u64,
    notified_user_ids: Vec<u64>,
    json: bool,
}

impl IssueCommentNotificationAddArgs {
    pub fn try_new(
        key: String,
        comment_id: u64,
        notified_user_ids: Vec<u64>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if notified_user_ids.is_empty() {
            anyhow::bail!("at least one --notified-user-id is required");
        }
        Ok(Self {
            key,
            comment_id,
            notified_user_ids,
            json,
        })
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
        crate::cmd::print_json(&notifications)?;
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

    use std::cell::RefCell;

    struct MockApi {
        notifications: Option<Vec<IssueCommentNotification>>,
        captured_params: RefCell<Vec<(String, String)>>,
    }

    impl MockApi {
        fn new(notifications: Option<Vec<IssueCommentNotification>>) -> Self {
            Self {
                notifications,
                captured_params: RefCell::new(vec![]),
            }
        }
    }

    impl crate::api::BacklogApi for MockApi {
        fn add_issue_comment_notifications(
            &self,
            _key: &str,
            _comment_id: u64,
            params: &[(String, String)],
        ) -> anyhow::Result<Vec<IssueCommentNotification>> {
            *self.captured_params.borrow_mut() = params.to_vec();
            self.notifications
                .clone()
                .ok_or_else(|| anyhow!("no notifications"))
        }
    }

    fn args(json: bool) -> IssueCommentNotificationAddArgs {
        IssueCommentNotificationAddArgs::try_new("TEST-1".to_string(), 1, vec![2], json).unwrap()
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi::new(Some(vec![sample_notification()]));
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi::new(Some(vec![sample_notification()]));
        assert!(add_with(&args(true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi::new(None);
        let err = add_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no notifications"));
    }

    #[test]
    fn add_with_builds_correct_params() {
        let api = MockApi::new(Some(vec![sample_notification()]));
        let a =
            IssueCommentNotificationAddArgs::try_new("TEST-1".to_string(), 1, vec![1, 2], false)
                .unwrap();
        add_with(&a, &api).unwrap();
        let params = api.captured_params.borrow();
        assert_eq!(
            *params,
            vec![
                ("notifiedUserId[]".to_string(), "1".to_string()),
                ("notifiedUserId[]".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn try_new_rejects_empty_user_ids() {
        let err = IssueCommentNotificationAddArgs::try_new("TEST-1".to_string(), 1, vec![], false)
            .unwrap_err();
        assert!(err.to_string().contains("at least one"));
    }
}
