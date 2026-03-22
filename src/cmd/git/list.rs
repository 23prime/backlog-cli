use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, git::GitRepository};

pub struct GitRepoListArgs {
    project_id_or_key: String,
    json: bool,
}

impl GitRepoListArgs {
    pub fn new(project_id_or_key: String, json: bool) -> Self {
        Self {
            project_id_or_key,
            json,
        }
    }
}

pub fn list(args: &GitRepoListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &GitRepoListArgs, api: &dyn BacklogApi) -> Result<()> {
    let repos = api.get_git_repositories(&args.project_id_or_key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&repos).context("Failed to serialize JSON")?
        );
    } else {
        for repo in &repos {
            println!("{}", format_repo_row(repo));
        }
    }
    Ok(())
}

pub fn format_repo_row(repo: &GitRepository) -> String {
    format!("{}", repo.name.cyan().bold())
}

#[cfg(test)]
pub(crate) mod tests_helper {
    use std::collections::BTreeMap;

    use crate::api::git::{GitRepository, GitUser};

    pub fn sample_git_user() -> GitUser {
        GitUser {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    pub fn sample_git_repo() -> GitRepository {
        GitRepository {
            id: 1,
            project_id: 10,
            name: "main".to_string(),
            description: "My repository".to_string(),
            hook_url: None,
            http_url: "https://example.backlog.com/git/TEST/main.git".to_string(),
            ssh_url: "git@example.backlog.com:/TEST/main.git".to_string(),
            display_order: 0,
            pushed_at: None,
            created_user: sample_git_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_git_user(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::git::GitRepository;
    use anyhow::anyhow;
    use tests_helper::sample_git_repo;

    struct MockApi {
        repos: Option<Vec<GitRepository>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_git_repositories(
            &self,
            _project_id_or_key: &str,
        ) -> anyhow::Result<Vec<GitRepository>> {
            self.repos.clone().ok_or_else(|| anyhow!("no repos"))
        }
    }

    fn args(json: bool) -> GitRepoListArgs {
        GitRepoListArgs::new("TEST".to_string(), json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            repos: Some(vec![sample_git_repo()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            repos: Some(vec![sample_git_repo()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { repos: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no repos"));
    }

    #[test]
    fn format_repo_row_contains_name() {
        let repo = sample_git_repo();
        let row = format_repo_row(&repo);
        assert!(row.contains("main"));
    }
}
