use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct PrCommentUpdateArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    comment_id: u64,
    content: String,
    json: bool,
}

impl PrCommentUpdateArgs {
    pub fn new(
        project_id_or_key: String,
        repo_id_or_name: String,
        number: u64,
        comment_id: u64,
        content: String,
        json: bool,
    ) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            number,
            comment_id,
            content,
            json,
        }
    }
}

pub fn update(args: &PrCommentUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &PrCommentUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![("content".to_string(), args.content.clone())];
    let comment = api.update_pull_request_comment(
        &args.project_id_or_key,
        &args.repo_id_or_name,
        args.number,
        args.comment_id,
        &params,
    )?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&comment).context("Failed to serialize JSON")?
        );
    } else {
        let content = comment.content.as_deref().unwrap_or("(no content)");
        println!("[{}] {}", comment.id, content);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::pull_request::PullRequestComment;
    use crate::cmd::pr::comment::list::tests_helper::sample_pr_comment;
    use anyhow::anyhow;

    struct MockApi {
        comment: Option<PullRequestComment>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn update_pull_request_comment(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<PullRequestComment> {
            self.comment.clone().ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn args(json: bool) -> PrCommentUpdateArgs {
        PrCommentUpdateArgs::new(
            "TEST".to_string(),
            "main".to_string(),
            1,
            1,
            "Updated".to_string(),
            json,
        )
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_pr_comment()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_pr_comment()),
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
