use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueCommentDeleteArgs {
    key: String,
    comment_id: u64,
    json: bool,
}

impl IssueCommentDeleteArgs {
    pub fn new(key: String, comment_id: u64, json: bool) -> Self {
        Self {
            key,
            comment_id,
            json,
        }
    }
}

pub fn delete(args: &IssueCommentDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &IssueCommentDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let comment = api.delete_issue_comment(&args.key, args.comment_id)?;
    if args.json {
        crate::cmd::print_json(&comment)?;
    } else {
        println!("Deleted comment {} from {}", comment.id, args.key);
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
        fn delete_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<IssueComment> {
            self.comment.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn args(json: bool) -> IssueCommentDeleteArgs {
        IssueCommentDeleteArgs::new("TEST-1".to_string(), 1, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { comment: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
