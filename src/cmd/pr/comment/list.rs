use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct PrCommentListArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    json: bool,
}

impl PrCommentListArgs {
    pub fn new(
        project_id_or_key: String,
        repo_id_or_name: String,
        number: u64,
        json: bool,
    ) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            number,
            json,
        }
    }
}

pub fn list(args: &PrCommentListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &PrCommentListArgs, api: &dyn BacklogApi) -> Result<()> {
    let comments = api.get_pull_request_comments(
        &args.project_id_or_key,
        &args.repo_id_or_name,
        args.number,
        &[],
    )?;
    if args.json {
        crate::cmd::print_json(&comments)?;
    } else {
        for c in &comments {
            let content = c.content.as_deref().unwrap_or("(no content)");
            println!("[{}] {} — {}", c.id, c.created_user.name, content);
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests_helper {
    use std::collections::BTreeMap;

    use crate::api::pull_request::PullRequestComment;
    use crate::cmd::pr::list::tests_helper::sample_pr_user;

    pub fn sample_pr_comment() -> PullRequestComment {
        PullRequestComment {
            id: 1,
            content: Some("LGTM".to_string()),
            change_log: vec![],
            created_user: sample_pr_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            stars: vec![],
            extra: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::pull_request::PullRequestComment;
    use anyhow::anyhow;
    use tests_helper::sample_pr_comment;

    struct MockApi {
        comments: Option<Vec<PullRequestComment>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_pull_request_comments(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<PullRequestComment>> {
            self.comments.clone().ok_or_else(|| anyhow!("no comments"))
        }
    }

    fn args(json: bool) -> PrCommentListArgs {
        PrCommentListArgs::new("TEST".to_string(), "main".to_string(), 1, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            comments: Some(vec![sample_pr_comment()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            comments: Some(vec![sample_pr_comment()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { comments: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no comments"));
    }
}
