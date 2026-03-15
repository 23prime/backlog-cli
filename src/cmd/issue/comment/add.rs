use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::comment::list::format_comment_row;

pub struct IssueCommentAddArgs {
    key: String,
    content: String,
    json: bool,
}

impl IssueCommentAddArgs {
    pub fn new(key: String, content: String, json: bool) -> Self {
        Self { key, content, json }
    }
}

pub fn add(args: &IssueCommentAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &IssueCommentAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![("content".to_string(), args.content.clone())];
    let comment = api.add_issue_comment(&args.key, &params)?;
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
        fn add_issue_comment(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            self.comment.clone().ok_or_else(|| anyhow!("add failed"))
        }
    }

    fn args(json: bool) -> IssueCommentAddArgs {
        IssueCommentAddArgs::new("TEST-1".to_string(), "hello".to_string(), json)
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(add_with(&args(true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi { comment: None };
        let err = add_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }
}
