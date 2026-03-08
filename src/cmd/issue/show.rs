use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, issue::Issue};

pub struct IssueShowArgs {
    pub key: String,
    pub json: bool,
}

pub fn show(args: &IssueShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &IssueShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let issue = api.get_issue(&args.key)?;
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
            self.issue.clone().ok_or_else(|| anyhow!("no issue"))
        }
        fn create_issue(&self, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn update_issue(&self, _key: &str, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            unimplemented!()
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

    fn args(json: bool) -> IssueShowArgs {
        IssueShowArgs {
            key: "TEST-1".to_string(),
            json,
        }
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
        let err = show_with(
            &IssueShowArgs {
                key: "TEST-999".to_string(),
                json: false,
            },
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no issue"));
    }
}
