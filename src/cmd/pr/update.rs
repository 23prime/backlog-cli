use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::pr::show::print_pr;

#[cfg_attr(test, derive(Debug))]
pub struct PrUpdateArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    summary: Option<String>,
    description: Option<String>,
    base: Option<String>,
    issue_id: Option<u64>,
    assignee_id: Option<u64>,
    comment: Option<String>,
    json: bool,
}

impl PrUpdateArgs {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        project_id_or_key: String,
        repo_id_or_name: String,
        number: u64,
        summary: Option<String>,
        description: Option<String>,
        base: Option<String>,
        issue_id: Option<u64>,
        assignee_id: Option<u64>,
        comment: Option<String>,
        json: bool,
    ) -> Result<Self> {
        if summary.is_none()
            && description.is_none()
            && base.is_none()
            && issue_id.is_none()
            && assignee_id.is_none()
            && comment.is_none()
        {
            return Err(anyhow::anyhow!(
                "at least one of --summary, --description, --base, --issue-id, --assignee-id, or --comment must be specified"
            ));
        }
        Ok(Self {
            project_id_or_key,
            repo_id_or_name,
            number,
            summary,
            description,
            base,
            issue_id,
            assignee_id,
            comment,
            json,
        })
    }
}

pub fn update(args: &PrUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &PrUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(summary) = &args.summary {
        params.push(("summary".to_string(), summary.clone()));
    }
    if let Some(description) = &args.description {
        params.push(("description".to_string(), description.clone()));
    }
    if let Some(base) = &args.base {
        params.push(("base".to_string(), base.clone()));
    }
    if let Some(issue_id) = args.issue_id {
        params.push(("issueId".to_string(), issue_id.to_string()));
    }
    if let Some(assignee_id) = args.assignee_id {
        params.push(("assigneeId".to_string(), assignee_id.to_string()));
    }
    if let Some(comment) = &args.comment {
        params.push(("comment".to_string(), comment.clone()));
    }
    let pr = api.update_pull_request(
        &args.project_id_or_key,
        &args.repo_id_or_name,
        args.number,
        &params,
    )?;
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
        fn update_pull_request(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<PullRequest> {
            self.pr.clone().ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn args(json: bool) -> PrUpdateArgs {
        PrUpdateArgs::try_new(
            "TEST".to_string(),
            "main".to_string(),
            1,
            Some("Updated summary".to_string()),
            None,
            None,
            None,
            None,
            None,
            json,
        )
        .unwrap()
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            pr: Some(sample_pr()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            pr: Some(sample_pr()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { pr: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn update_rejects_no_fields() {
        let err = PrUpdateArgs::try_new(
            "TEST".to_string(),
            "main".to_string(),
            1,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap_err();
        assert!(err.to_string().contains("at least one of"));
    }
}
