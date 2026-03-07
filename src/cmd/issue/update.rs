use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::show::print_issue;

#[allow(clippy::too_many_arguments)]
pub fn update(
    key: &str,
    summary: Option<&str>,
    description: Option<&str>,
    status_id: Option<u64>,
    priority_id: Option<u64>,
    assignee_id: Option<u64>,
    due_date: Option<&str>,
    comment: Option<&str>,
    json: bool,
) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(
        key,
        summary,
        description,
        status_id,
        priority_id,
        assignee_id,
        due_date,
        comment,
        json,
        &client,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn update_with(
    key: &str,
    summary: Option<&str>,
    description: Option<&str>,
    status_id: Option<u64>,
    priority_id: Option<u64>,
    assignee_id: Option<u64>,
    due_date: Option<&str>,
    comment: Option<&str>,
    json: bool,
    api: &dyn BacklogApi,
) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(s) = summary {
        params.push(("summary".to_string(), s.to_string()));
    }
    if let Some(d) = description {
        params.push(("description".to_string(), d.to_string()));
    }
    if let Some(id) = status_id {
        params.push(("statusId".to_string(), id.to_string()));
    }
    if let Some(id) = priority_id {
        params.push(("priorityId".to_string(), id.to_string()));
    }
    if let Some(id) = assignee_id {
        params.push(("assigneeId".to_string(), id.to_string()));
    }
    if let Some(date) = due_date {
        params.push(("dueDate".to_string(), date.to_string()));
    }
    if let Some(c) = comment {
        params.push(("comment".to_string(), c.to_string()));
    }

    let issue = api.update_issue(key, &params)?;
    if json {
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

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(
            update_with(
                "TEST-1", None, None, None, None, None, None, None, false, &api
            )
            .is_ok()
        );
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            issue: Some(sample_issue()),
        };
        assert!(
            update_with(
                "TEST-1", None, None, None, None, None, None, None, true, &api
            )
            .is_ok()
        );
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { issue: None };
        let err = update_with(
            "TEST-1", None, None, None, None, None, None, None, false, &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }
}
