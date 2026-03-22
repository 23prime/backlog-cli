use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::pr::show::print_pr;

pub struct PrCreateArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    summary: String,
    description: Option<String>,
    base: String,
    branch: String,
    issue_id: Option<u64>,
    assignee_id: Option<u64>,
    json: bool,
}

impl PrCreateArgs {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_id_or_key: String,
        repo_id_or_name: String,
        summary: String,
        description: Option<String>,
        base: String,
        branch: String,
        issue_id: Option<u64>,
        assignee_id: Option<u64>,
        json: bool,
    ) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            summary,
            description,
            base,
            branch,
            issue_id,
            assignee_id,
            json,
        }
    }
}

pub fn create(args: &PrCreateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    create_with(args, &client)
}

pub fn create_with(args: &PrCreateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = vec![
        ("summary".to_string(), args.summary.clone()),
        ("base".to_string(), args.base.clone()),
        ("branch".to_string(), args.branch.clone()),
    ];
    if let Some(desc) = &args.description {
        params.push(("description".to_string(), desc.clone()));
    }
    if let Some(issue_id) = args.issue_id {
        params.push(("issueId".to_string(), issue_id.to_string()));
    }
    if let Some(assignee_id) = args.assignee_id {
        params.push(("assigneeId".to_string(), assignee_id.to_string()));
    }
    let pr = api.create_pull_request(&args.project_id_or_key, &args.repo_id_or_name, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&pr).context("Failed to serialize JSON")?
        );
    } else {
        print_pr(&pr);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::pull_request::PullRequest;
    use crate::cmd::pr::list::tests_helper::sample_pr;
    use anyhow::anyhow;

    struct MockApi {
        pr: Option<PullRequest>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn create_pull_request(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<PullRequest> {
            self.pr.clone().ok_or_else(|| anyhow!("create failed"))
        }
    }

    fn args(json: bool) -> PrCreateArgs {
        PrCreateArgs::new(
            "TEST".to_string(),
            "main".to_string(),
            "Fix bug".to_string(),
            None,
            "main".to_string(),
            "feature/fix".to_string(),
            None,
            None,
            json,
        )
    }

    #[test]
    fn create_with_text_output_succeeds() {
        let api = MockApi {
            pr: Some(sample_pr()),
        };
        assert!(create_with(&args(false), &api).is_ok());
    }

    #[test]
    fn create_with_json_output_succeeds() {
        let api = MockApi {
            pr: Some(sample_pr()),
        };
        assert!(create_with(&args(true), &api).is_ok());
    }

    #[test]
    fn create_with_propagates_api_error() {
        let api = MockApi { pr: None };
        let err = create_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("create failed"));
    }
}
