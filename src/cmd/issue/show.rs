use anstream::println;
use anyhow::Result;
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, issue::Issue};

pub struct IssueShowArgs {
    key: String,
    json: bool,
}

impl IssueShowArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn show(args: &IssueShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &IssueShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let issue = api.get_issue(&args.key)?;
    if args.json {
        crate::cmd::print_json(&issue)?;
    } else {
        print_issue(&issue);
    }
    Ok(())
}

pub fn print_issue(issue: &Issue) {
    println!("{} {}", issue.issue_key.cyan().bold(), issue.summary.bold());
    println!("  Status:     {}", issue.status.name.yellow());
    println!("  Priority:   {}", issue.priority.name);
    println!("  Type:       {}", issue.issue_type.name);
    let assignee = issue
        .assignee
        .as_ref()
        .map(|u| u.name.as_str())
        .unwrap_or("-");
    println!("  Assignee:   {}", assignee);
    if let Some(due) = &issue.due_date {
        println!("  Due:        {}", due);
    }
    if let Some(desc) = &issue.description
        && !desc.is_empty()
    {
        println!("  Description:\n{}", desc);
    }
    println!("  Created:    {}", issue.created);
    println!("  Updated:    {}", issue.updated);
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
        fn get_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            self.issue.clone().ok_or_else(|| anyhow!("no issue"))
        }
    }

    fn args(json: bool) -> IssueShowArgs {
        IssueShowArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { issue: None };
        let err = show_with(&IssueShowArgs::new("TEST-999".to_string(), false), &api).unwrap_err();
        assert!(err.to_string().contains("no issue"));
    }
}
