use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::show::print_issue;

pub struct IssueCreateArgs {
    project_id: u64,
    summary: String,
    issue_type_id: u64,
    priority_id: u64,
    description: Option<String>,
    assignee_id: Option<u64>,
    due_date: Option<String>,
    json: bool,
}

impl IssueCreateArgs {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_id: u64,
        summary: String,
        issue_type_id: u64,
        priority_id: u64,
        description: Option<String>,
        assignee_id: Option<u64>,
        due_date: Option<String>,
        json: bool,
    ) -> Self {
        Self {
            project_id,
            summary,
            issue_type_id,
            priority_id,
            description,
            assignee_id,
            due_date,
            json,
        }
    }
}

pub fn create(args: &IssueCreateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    create_with(args, &client)
}

pub fn create_with(args: &IssueCreateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = vec![
        ("projectId".to_string(), args.project_id.to_string()),
        ("summary".to_string(), args.summary.clone()),
        ("issueTypeId".to_string(), args.issue_type_id.to_string()),
        ("priorityId".to_string(), args.priority_id.to_string()),
    ];
    if let Some(d) = &args.description {
        params.push(("description".to_string(), d.clone()));
    }
    if let Some(id) = args.assignee_id {
        params.push(("assigneeId".to_string(), id.to_string()));
    }
    if let Some(date) = &args.due_date {
        params.push(("dueDate".to_string(), date.clone()));
    }

    let issue = api.create_issue(&params)?;
    if args.json {
        crate::cmd::print_json(&issue)?;
    } else {
        print_issue(&issue);
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
        fn create_issue(&self, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            self.issue.clone().ok_or_else(|| anyhow!("create failed"))
        }
    }

    fn args(json: bool) -> IssueCreateArgs {
        IssueCreateArgs::new(1, "Bug".to_string(), 1, 2, None, None, None, json)
    }

    #[test]
    fn create_with_text_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(create_with(&args(false), &api).is_ok());
    }

    #[test]
    fn create_with_json_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(create_with(&args(true), &api).is_ok());
    }

    #[test]
    fn create_with_propagates_api_error() {
        let api = MockApi { issue: None };
        let err = create_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("create failed"));
    }
}
