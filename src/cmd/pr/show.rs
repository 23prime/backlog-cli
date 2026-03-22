use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, pull_request::PullRequest};

pub struct PrShowArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    json: bool,
}

impl PrShowArgs {
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

pub fn show(args: &PrShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &PrShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let pr = api.get_pull_request(&args.project_id_or_key, &args.repo_id_or_name, args.number)?;
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

pub fn print_pr(pr: &PullRequest) {
    println!("[{}] {}", pr.number.to_string().cyan().bold(), pr.summary);
    println!("  Branch:  {} → {}", pr.branch, pr.base);
    println!("  Status:  {}", pr.status.name);
    if let Some(assignee) = &pr.assignee {
        println!("  Assignee: {}", assignee.name);
    }
    if let Some(issue) = &pr.issue {
        println!("  Issue:   #{}", issue.id);
    }
    if !pr.description.is_empty() {
        println!("\n{}", pr.description);
    }
    println!("  Created: {}", pr.created);
    println!("  Updated: {}", pr.updated);
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
        fn get_pull_request(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
        ) -> anyhow::Result<PullRequest> {
            self.pr.clone().ok_or_else(|| anyhow!("no pull request"))
        }
    }

    fn args(json: bool) -> PrShowArgs {
        PrShowArgs::new("TEST".to_string(), "main".to_string(), 1, json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            pr: Some(sample_pr()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            pr: Some(sample_pr()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { pr: None };
        let err = show_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no pull request"));
    }
}
