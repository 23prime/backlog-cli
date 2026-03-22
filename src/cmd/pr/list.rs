use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, pull_request::PullRequest};

pub struct PrListArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    json: bool,
}

impl PrListArgs {
    pub fn new(project_id_or_key: String, repo_id_or_name: String, json: bool) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            json,
        }
    }
}

pub fn list(args: &PrListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &PrListArgs, api: &dyn BacklogApi) -> Result<()> {
    let prs = api.get_pull_requests(&args.project_id_or_key, &args.repo_id_or_name, &[])?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&prs).context("Failed to serialize JSON")?
        );
    } else {
        for pr in &prs {
            println!("{}", format_pr_row(pr));
        }
    }
    Ok(())
}

pub fn format_pr_row(pr: &PullRequest) -> String {
    format!(
        "[{}] {} ({} → {}) [{}]",
        pr.number.to_string().cyan().bold(),
        pr.summary,
        pr.branch,
        pr.base,
        pr.status.name
    )
}

#[cfg(test)]
pub(crate) mod tests_helper {
    use std::collections::BTreeMap;

    use crate::api::pull_request::{PrUser, PullRequest, PullRequestStatus};

    pub fn sample_pr_user() -> PrUser {
        PrUser {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    pub fn sample_pr() -> PullRequest {
        PullRequest {
            id: 1,
            project_id: 10,
            repository_id: 2,
            number: 1,
            summary: "Fix bug".to_string(),
            description: "Fixes the bug".to_string(),
            base: "main".to_string(),
            branch: "feature/fix".to_string(),
            status: PullRequestStatus {
                id: 1,
                name: "Open".to_string(),
            },
            assignee: None,
            issue: None,
            base_commit: None,
            branch_commit: None,
            merge_commit: None,
            close_at: None,
            merge_at: None,
            created_user: sample_pr_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_pr_user(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            attachments: vec![],
            stars: vec![],
            extra: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::pull_request::PullRequest;
    use anyhow::anyhow;
    use tests_helper::sample_pr;

    struct MockApi {
        prs: Option<Vec<PullRequest>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_pull_requests(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<PullRequest>> {
            self.prs.clone().ok_or_else(|| anyhow!("no pull requests"))
        }
    }

    fn args(json: bool) -> PrListArgs {
        PrListArgs::new("TEST".to_string(), "main".to_string(), json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            prs: Some(vec![sample_pr()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            prs: Some(vec![sample_pr()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { prs: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no pull requests"));
    }

    #[test]
    fn format_pr_row_contains_expected_fields() {
        let pr = sample_pr();
        let row = format_pr_row(&pr);
        assert!(row.contains("1"));
        assert!(row.contains("Fix bug"));
        assert!(row.contains("feature/fix"));
        assert!(row.contains("main"));
        assert!(row.contains("Open"));
    }
}
