use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, issue::Issue};

#[allow(clippy::too_many_arguments)]
pub fn list(
    project_ids: &[u64],
    status_ids: &[u64],
    assignee_ids: &[u64],
    keyword: Option<&str>,
    count: u32,
    offset: u64,
    json: bool,
) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(
        project_ids,
        status_ids,
        assignee_ids,
        keyword,
        count,
        offset,
        json,
        &client,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn list_with(
    project_ids: &[u64],
    status_ids: &[u64],
    assignee_ids: &[u64],
    keyword: Option<&str>,
    count: u32,
    offset: u64,
    json: bool,
    api: &dyn BacklogApi,
) -> Result<()> {
    let params = build_params(
        project_ids,
        status_ids,
        assignee_ids,
        keyword,
        count,
        offset,
    );
    let issues = api.get_issues(&params)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issues).context("Failed to serialize JSON")?
        );
    } else {
        for issue in &issues {
            println!("{}", format_issue_row(issue));
        }
    }
    Ok(())
}

fn build_params(
    project_ids: &[u64],
    status_ids: &[u64],
    assignee_ids: &[u64],
    keyword: Option<&str>,
    count: u32,
    offset: u64,
) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    for id in project_ids {
        params.push(("projectId[]".to_string(), id.to_string()));
    }
    for id in status_ids {
        params.push(("statusId[]".to_string(), id.to_string()));
    }
    for id in assignee_ids {
        params.push(("assigneeId[]".to_string(), id.to_string()));
    }
    if let Some(kw) = keyword {
        params.push(("keyword".to_string(), kw.to_string()));
    }
    params.push(("count".to_string(), count.to_string()));
    params.push(("offset".to_string(), offset.to_string()));
    params
}

pub fn format_issue_row(issue: &Issue) -> String {
    let assignee = issue
        .assignee
        .as_ref()
        .map(|u| u.name.as_str())
        .unwrap_or("-");
    format!(
        "[{}] {} ({}, {}, {})",
        issue.issue_key.cyan().bold(),
        issue.summary,
        issue.status.name.yellow(),
        issue.priority.name,
        assignee
    )
}

#[cfg(test)]
use crate::api::issue::{IssuePriority, IssueStatus, IssueType, IssueUser};
#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
fn sample_user() -> IssueUser {
    IssueUser {
        id: 1,
        user_id: Some("john".to_string()),
        name: "John Doe".to_string(),
        role_type: 1,
        lang: None,
        mail_address: None,
        extra: BTreeMap::new(),
    }
}

#[cfg(test)]
pub(crate) fn sample_issue() -> crate::api::issue::Issue {
    use crate::api::issue::Issue;
    Issue {
        id: 1,
        project_id: 1,
        issue_key: "TEST-1".to_string(),
        key_id: 1,
        issue_type: IssueType {
            id: 1,
            project_id: 1,
            name: "Bug".to_string(),
            color: "#e30000".to_string(),
            display_order: 0,
        },
        summary: "Fix login".to_string(),
        description: None,
        resolutions: None,
        priority: IssuePriority {
            id: 2,
            name: "Normal".to_string(),
        },
        status: IssueStatus {
            id: 1,
            project_id: 1,
            name: "Open".to_string(),
            color: "#ed8077".to_string(),
            display_order: 1000,
        },
        assignee: None,
        start_date: None,
        due_date: None,
        estimated_hours: None,
        actual_hours: None,
        parent_issue_id: None,
        created_user: sample_user(),
        created: "2024-01-01T00:00:00Z".to_string(),
        updated_user: sample_user(),
        updated: "2024-01-01T00:00:00Z".to_string(),
        extra: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount, IssueUser};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        issues: Option<Vec<Issue>>,
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
            self.issues.clone().ok_or_else(|| anyhow!("no issues"))
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

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            issues: Some(vec![sample_issue()]),
        };
        assert!(list_with(&[], &[], &[], None, 20, 0, false, &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            issues: Some(vec![sample_issue()]),
        };
        assert!(list_with(&[], &[], &[], None, 20, 0, true, &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { issues: None };
        let err = list_with(&[], &[], &[], None, 20, 0, false, &api).unwrap_err();
        assert!(err.to_string().contains("no issues"));
    }

    #[test]
    fn format_issue_row_no_assignee() {
        let row = format_issue_row(&sample_issue());
        assert!(row.contains("TEST-1"));
        assert!(row.contains("Fix login"));
        assert!(row.contains("Open"));
        assert!(row.contains("Normal"));
        assert!(row.contains('-'));
    }

    #[test]
    fn format_issue_row_with_assignee() {
        let mut issue = sample_issue();
        issue.assignee = Some(IssueUser {
            id: 2,
            user_id: Some("jane".to_string()),
            name: "Jane Smith".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        });
        let row = format_issue_row(&issue);
        assert!(row.contains("Jane Smith"));
    }

    #[test]
    fn build_params_includes_all_fields() {
        let params = build_params(&[1, 2], &[3], &[4], Some("login"), 50, 10);
        assert!(params.iter().any(|(k, v)| k == "projectId[]" && v == "1"));
        assert!(params.iter().any(|(k, v)| k == "projectId[]" && v == "2"));
        assert!(params.iter().any(|(k, v)| k == "statusId[]" && v == "3"));
        assert!(params.iter().any(|(k, v)| k == "assigneeId[]" && v == "4"));
        assert!(params.iter().any(|(k, v)| k == "keyword" && v == "login"));
        assert!(params.iter().any(|(k, v)| k == "count" && v == "50"));
        assert!(params.iter().any(|(k, v)| k == "offset" && v == "10"));
    }
}
