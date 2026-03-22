use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::comment::list::format_comment_row;

pub struct IssueCommentUpdateArgs {
    key: String,
    comment_id: u64,
    content: String,
    json: bool,
}

impl IssueCommentUpdateArgs {
    pub fn new(key: String, comment_id: u64, content: String, json: bool) -> Self {
        Self {
            key,
            comment_id,
            content,
            json,
        }
    }
}

pub fn update(args: &IssueCommentUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &IssueCommentUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![("content".to_string(), args.content.clone())];
    let comment = api.update_issue_comment(&args.key, args.comment_id, &params)?;
    if args.json {
        crate::cmd::print_json(&comment)?;
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
        fn update_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            self.comment.clone().ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn args(json: bool) -> IssueCommentUpdateArgs {
        IssueCommentUpdateArgs::new("TEST-1".to_string(), 1, "updated".to_string(), json)
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { comment: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }
}
