use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectIssueType};

pub struct ProjectIssueTypeListArgs {
    key: String,
    json: bool,
}

impl ProjectIssueTypeListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &ProjectIssueTypeListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectIssueTypeListArgs, api: &dyn BacklogApi) -> Result<()> {
    let issue_types = api.get_project_issue_types(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issue_types).context("Failed to serialize JSON")?
        );
    } else {
        for t in &issue_types {
            println!("{}", format_issue_type_row(t));
        }
    }
    Ok(())
}

fn format_issue_type_row(t: &ProjectIssueType) -> String {
    format!("[{}] {}", t.id, t.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        issue_types: Option<Vec<ProjectIssueType>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_issue_types(&self, _key: &str) -> anyhow::Result<Vec<ProjectIssueType>> {
            self.issue_types
                .clone()
                .ok_or_else(|| anyhow!("no issue types"))
        }
    }

    fn sample_issue_type() -> ProjectIssueType {
        ProjectIssueType {
            id: 1,
            project_id: 10,
            name: "Bug".to_string(),
            color: "#e30000".to_string(),
            display_order: 0,
            template_summary: None,
            template_description: None,
        }
    }

    #[test]
    fn format_issue_type_row_contains_fields() {
        let text = format_issue_type_row(&sample_issue_type());
        assert!(text.contains("[1]"));
        assert!(text.contains("Bug"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            issue_types: Some(vec![sample_issue_type()]),
        };
        assert!(
            list_with(
                &ProjectIssueTypeListArgs::new("TEST".to_string(), false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            issue_types: Some(vec![sample_issue_type()]),
        };
        assert!(
            list_with(
                &ProjectIssueTypeListArgs::new("TEST".to_string(), true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { issue_types: None };
        let err = list_with(
            &ProjectIssueTypeListArgs::new("TEST".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no issue types"));
    }
}
