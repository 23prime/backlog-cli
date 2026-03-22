use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct PrCountArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    json: bool,
}

impl PrCountArgs {
    pub fn new(project_id_or_key: String, repo_id_or_name: String, json: bool) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            json,
        }
    }
}

pub fn count(args: &PrCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &PrCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let result = api.count_pull_requests(&args.project_id_or_key, &args.repo_id_or_name, &[])?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", result.count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::pull_request::PullRequestCount;
    use anyhow::anyhow;

    struct MockApi {
        count: Option<PullRequestCount>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn count_pull_requests(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<PullRequestCount> {
            self.count.clone().ok_or_else(|| anyhow!("no count"))
        }
    }

    fn args(json: bool) -> PrCountArgs {
        PrCountArgs::new("TEST".to_string(), "main".to_string(), json)
    }

    #[test]
    fn count_with_text_output_succeeds() {
        let api = MockApi {
            count: Some(PullRequestCount { count: 5 }),
        };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi {
            count: Some(PullRequestCount { count: 5 }),
        };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi { count: None };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no count"));
    }
}
