use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::ParentChild;

pub struct IssueCountArgs {
    project_ids: Vec<u64>,
    status_ids: Vec<u64>,
    assignee_ids: Vec<u64>,
    issue_type_ids: Vec<u64>,
    category_ids: Vec<u64>,
    milestone_ids: Vec<u64>,
    parent_child: Option<ParentChild>,
    keyword: Option<String>,
    json: bool,
}

impl IssueCountArgs {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_ids: Vec<u64>,
        status_ids: Vec<u64>,
        assignee_ids: Vec<u64>,
        issue_type_ids: Vec<u64>,
        category_ids: Vec<u64>,
        milestone_ids: Vec<u64>,
        parent_child: Option<ParentChild>,
        keyword: Option<String>,
        json: bool,
    ) -> Self {
        Self {
            project_ids,
            status_ids,
            assignee_ids,
            issue_type_ids,
            category_ids,
            milestone_ids,
            parent_child,
            keyword,
            json,
        }
    }
}

pub fn count(args: &IssueCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &IssueCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = build_params(args);
    let result = api.count_issues(&params)?;
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

fn build_params(args: &IssueCountArgs) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    for id in &args.project_ids {
        params.push(("projectId[]".to_string(), id.to_string()));
    }
    for id in &args.status_ids {
        params.push(("statusId[]".to_string(), id.to_string()));
    }
    for id in &args.assignee_ids {
        params.push(("assigneeId[]".to_string(), id.to_string()));
    }
    for id in &args.issue_type_ids {
        params.push(("issueTypeId[]".to_string(), id.to_string()));
    }
    for id in &args.category_ids {
        params.push(("categoryId[]".to_string(), id.to_string()));
    }
    for id in &args.milestone_ids {
        params.push(("milestoneId[]".to_string(), id.to_string()));
    }
    if let Some(pc) = &args.parent_child {
        params.push(("parentChild".to_string(), pc.to_api_value().to_string()));
    }
    if let Some(kw) = &args.keyword {
        params.push(("keyword".to_string(), kw.clone()));
    }
    params
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::IssueCount;
    use anyhow::anyhow;

    struct MockApi {
        count: Option<u64>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn count_issues(&self, _params: &[(String, String)]) -> anyhow::Result<IssueCount> {
            self.count
                .map(|c| IssueCount { count: c })
                .ok_or_else(|| anyhow!("no count"))
        }
    }

    fn args(json: bool) -> IssueCountArgs {
        IssueCountArgs::new(
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            None,
            None,
            json,
        )
    }

    #[test]
    fn count_with_text_output_succeeds() {
        let api = MockApi { count: Some(42) };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi { count: Some(0) };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi { count: None };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no count"));
    }
}
