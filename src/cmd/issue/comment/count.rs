use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueCommentCountArgs {
    key: String,
    json: bool,
}

impl IssueCommentCountArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn count(args: &IssueCommentCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &IssueCommentCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let result = api.count_issue_comments(&args.key)?;
    if args.json {
        crate::cmd::print_json(&result)?;
    } else {
        println!("{}", result.count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::IssueCommentCount;
    use anyhow::anyhow;

    struct MockApi {
        count: Option<u64>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn count_issue_comments(&self, _key: &str) -> anyhow::Result<IssueCommentCount> {
            self.count
                .map(|c| IssueCommentCount { count: c })
                .ok_or_else(|| anyhow!("no count"))
        }
    }

    fn args(json: bool) -> IssueCommentCountArgs {
        IssueCommentCountArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn count_with_text_output_succeeds() {
        let api = MockApi { count: Some(5) };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi { count: Some(5) };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi { count: None };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no count"));
    }
}
