use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueDeleteArgs {
    key: String,
    json: bool,
}

impl IssueDeleteArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn delete(args: &IssueDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &IssueDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let issue = api.delete_issue(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issue).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", issue.issue_key);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::Issue;
    use crate::cmd::issue::list::sample_issue;
    use anyhow::anyhow;

    struct MockApi {
        issue: Option<Issue>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            self.issue.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn args(json: bool) -> IssueDeleteArgs {
        IssueDeleteArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { issue: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
