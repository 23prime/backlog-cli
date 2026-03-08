use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectIssueType};

pub struct ProjectIssueTypeListArgs {
    pub key: String,
    pub json: bool,
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
        fn get_project_issue_types(&self, _key: &str) -> anyhow::Result<Vec<ProjectIssueType>> {
            self.issue_types
                .clone()
                .ok_or_else(|| anyhow!("no issue types"))
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
                &ProjectIssueTypeListArgs {
                    key: "TEST".to_string(),
                    json: false
                },
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
                &ProjectIssueTypeListArgs {
                    key: "TEST".to_string(),
                    json: true
                },
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { issue_types: None };
        let err = list_with(
            &ProjectIssueTypeListArgs {
                key: "TEST".to_string(),
                json: false,
            },
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no issue types"));
    }
}
