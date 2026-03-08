use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::show::print_issue;

pub struct IssueUpdateArgs {
    pub key: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub status_id: Option<u64>,
    pub priority_id: Option<u64>,
    pub assignee_id: Option<u64>,
    pub due_date: Option<String>,
    pub comment: Option<String>,
    pub json: bool,
}

pub fn update(args: &IssueUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &IssueUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(s) = &args.summary {
        params.push(("summary".to_string(), s.clone()));
    }
    if let Some(d) = &args.description {
        params.push(("description".to_string(), d.clone()));
    }
    if let Some(id) = args.status_id {
        params.push(("statusId".to_string(), id.to_string()));
    }
    if let Some(id) = args.priority_id {
        params.push(("priorityId".to_string(), id.to_string()));
    }
    if let Some(id) = args.assignee_id {
        params.push(("assigneeId".to_string(), id.to_string()));
    }
    if let Some(date) = &args.due_date {
        params.push(("dueDate".to_string(), date.clone()));
    }
    if let Some(c) = &args.comment {
        params.push(("comment".to_string(), c.clone()));
    }

    if params.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one field must be specified for update"
        ));
    }
    let issue = api.update_issue(&args.key, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issue).context("Failed to serialize JSON")?
        );
    } else {
        print_issue(&issue);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount};
    use crate::cmd::issue::list::sample_issue;
    use anyhow::anyhow;

    struct MockApi {
        issue: Option<Issue>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(&self) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_space_disk_usage(&self) -> anyhow::Result<crate::api::disk_usage::DiskUsage> {
            unimplemented!()
        }
        fn get_space_notification(
            &self,
        ) -> anyhow::Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
        }
        fn get_projects(&self) -> anyhow::Result<Vec<crate::api::project::Project>> {
            unimplemented!()
        }
        fn get_project(&self, _key: &str) -> anyhow::Result<crate::api::project::Project> {
            unimplemented!()
        }
        fn get_project_activities(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(
            &self,
            _key: &str,
        ) -> anyhow::Result<crate::api::project::ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectCategory>> {
            unimplemented!()
        }
        fn get_project_versions(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectVersion>> {
            unimplemented!()
        }
        fn get_issues(&self, _params: &[(String, String)]) -> anyhow::Result<Vec<Issue>> {
            unimplemented!()
        }
        fn count_issues(&self, _params: &[(String, String)]) -> anyhow::Result<IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn create_issue(&self, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn update_issue(&self, _key: &str, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            self.issue.clone().ok_or_else(|| anyhow!("update failed"))
        }
        fn delete_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn get_issue_comments(&self, _key: &str) -> anyhow::Result<Vec<IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(&self, _key: &str) -> anyhow::Result<Vec<IssueAttachment>> {
            unimplemented!()
        }
    }

    fn args(json: bool) -> IssueUpdateArgs {
        IssueUpdateArgs {
            key: "TEST-1".to_string(),
            summary: Some("Updated summary".to_string()),
            description: None,
            status_id: None,
            priority_id: None,
            assignee_id: None,
            due_date: None,
            comment: None,
            json,
        }
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_rejects_empty_params() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        let empty = IssueUpdateArgs {
            key: "TEST-1".to_string(),
            summary: None,
            description: None,
            status_id: None,
            priority_id: None,
            assignee_id: None,
            due_date: None,
            comment: None,
            json: false,
        };
        let err = update_with(&empty, &api).unwrap_err();
        assert!(err.to_string().contains("At least one field"));
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { issue: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }
}
