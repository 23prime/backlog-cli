use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct PrCommentAddArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    content: String,
    json: bool,
}

impl PrCommentAddArgs {
    pub fn new(
        project_id_or_key: String,
        repo_id_or_name: String,
        number: u64,
        content: String,
        json: bool,
    ) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            number,
            content,
            json,
        }
    }
}

pub fn add(args: &PrCommentAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &PrCommentAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![("content".to_string(), args.content.clone())];
    let comment = api.add_pull_request_comment(
        &args.project_id_or_key,
        &args.repo_id_or_name,
        args.number,
        &params,
    )?;
    if args.json {
        crate::cmd::print_json(&comment)?;
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
        fn add_pull_request_comment(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<PullRequestComment> {
            self.comment.clone().ok_or_else(|| anyhow!("add failed"))
        }
    }

    fn args(json: bool) -> PrCommentAddArgs {
        PrCommentAddArgs::new(
            "TEST".to_string(),
            "main".to_string(),
            1,
            "LGTM".to_string(),
            json,
        )
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_pr_comment()),
        };
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_pr_comment()),
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
