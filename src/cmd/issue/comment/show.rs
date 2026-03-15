use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::comment::list::format_comment_row;

pub struct IssueCommentShowArgs {
    key: String,
    comment_id: u64,
    json: bool,
}

impl IssueCommentShowArgs {
    pub fn new(key: String, comment_id: u64, json: bool) -> Self {
        Self {
            key,
            comment_id,
            json,
        }
    }
}

pub fn show(args: &IssueCommentShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &IssueCommentShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let comment = api.get_issue_comment(&args.key, args.comment_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&comment).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_comment_row(&comment));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::IssueComment;
    use crate::cmd::issue::comment::list::sample_comment;
    use anyhow::anyhow;

    struct MockApi {
        comment: Option<IssueComment>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_issue_comment(&self, _key: &str, _comment_id: u64) -> anyhow::Result<IssueComment> {
            self.comment.clone().ok_or_else(|| anyhow!("no comment"))
        }
    }

    fn args(json: bool) -> IssueCommentShowArgs {
        IssueCommentShowArgs::new("TEST-1".to_string(), 1, json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { comment: None };
        let err = show_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no comment"));
    }
}
