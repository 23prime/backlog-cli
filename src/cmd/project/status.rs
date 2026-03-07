use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectStatus};

pub fn list(key: &str, json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(key, json, &client)
}

pub fn list_with(key: &str, json: bool, api: &dyn BacklogApi) -> Result<()> {
    let statuses = api.get_project_statuses(key)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&statuses).context("Failed to serialize JSON")?
        );
    } else {
        for s in &statuses {
            println!("{}", format_status_row(s));
        }
    }
    Ok(())
}

fn format_status_row(s: &ProjectStatus) -> String {
    format!("[{}] {}", s.id, s.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        statuses: Option<Vec<ProjectStatus>>,
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
        fn get_project_statuses(&self, _key: &str) -> anyhow::Result<Vec<ProjectStatus>> {
            self.statuses.clone().ok_or_else(|| anyhow!("no statuses"))
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
        fn get_issues(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::issue::Issue>> {
            unimplemented!()
        }
        fn count_issues(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _key: &str) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn create_issue(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn update_issue(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _key: &str) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn get_issue_comments(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueAttachment>> {
            unimplemented!()
        }
    }

    fn sample_status() -> ProjectStatus {
        ProjectStatus {
            id: 1,
            project_id: 10,
            name: "Open".to_string(),
            color: "#ed8077".to_string(),
            display_order: 1000,
        }
    }

    #[test]
    fn format_status_row_contains_fields() {
        let text = format_status_row(&sample_status());
        assert!(text.contains("[1]"));
        assert!(text.contains("Open"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            statuses: Some(vec![sample_status()]),
        };
        assert!(list_with("TEST", false, &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            statuses: Some(vec![sample_status()]),
        };
        assert!(list_with("TEST", true, &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { statuses: None };
        let err = list_with("TEST", false, &api).unwrap_err();
        assert!(err.to_string().contains("no statuses"));
    }
}
