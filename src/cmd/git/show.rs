use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, git::GitRepository};

pub struct GitRepoShowArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    json: bool,
}

impl GitRepoShowArgs {
    pub fn new(project_id_or_key: String, repo_id_or_name: String, json: bool) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            json,
        }
    }
}

pub fn show(args: &GitRepoShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &GitRepoShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let repo = api.get_git_repository(&args.project_id_or_key, &args.repo_id_or_name)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&repo).context("Failed to serialize JSON")?
        );
    } else {
        print_repo(&repo);
    }
    Ok(())
}

pub fn print_repo(repo: &GitRepository) {
    println!("{}", repo.name.cyan().bold());
    if !repo.description.is_empty() {
        println!("  Description: {}", repo.description);
    }
    println!("  HTTP URL:    {}", repo.http_url);
    println!("  SSH URL:     {}", repo.ssh_url);
    if let Some(pushed_at) = &repo.pushed_at {
        println!("  Pushed at:   {}", pushed_at);
    }
    println!("  Created:     {}", repo.created);
    println!("  Updated:     {}", repo.updated);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::git::GitRepository;
    use crate::cmd::git::list::tests_helper::sample_git_repo;
    use anyhow::anyhow;

    struct MockApi {
        repo: Option<GitRepository>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_git_repository(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
        ) -> anyhow::Result<GitRepository> {
            self.repo.clone().ok_or_else(|| anyhow!("no repo"))
        }
    }

    fn args(json: bool) -> GitRepoShowArgs {
        GitRepoShowArgs::new("TEST".to_string(), "main".to_string(), json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            repo: Some(sample_git_repo()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            repo: Some(sample_git_repo()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { repo: None };
        let err = show_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no repo"));
    }
}
