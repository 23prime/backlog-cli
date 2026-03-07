use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectUser};

pub struct ProjectUserListArgs {
    pub key: String,
    pub json: bool,
}

pub fn list(args: &ProjectUserListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectUserListArgs, api: &dyn BacklogApi) -> Result<()> {
    let users = api.get_project_users(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&users).context("Failed to serialize JSON")?
        );
    } else {
        for u in &users {
            println!("{}", format_user_row(u));
        }
    }
    Ok(())
}

fn format_user_row(u: &ProjectUser) -> String {
    match u.user_id.as_deref() {
        Some(user_id) if !user_id.is_empty() => format!("[{}] {}", user_id, u.name),
        _ => format!("[{}] {}", u.id, u.name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        users: Option<Vec<ProjectUser>>,
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
        fn get_project_users(&self, _key: &str) -> anyhow::Result<Vec<ProjectUser>> {
            self.users.clone().ok_or_else(|| anyhow!("no users"))
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

    fn sample_user() -> ProjectUser {
        ProjectUser {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            role_type: 1,
            lang: Some("ja".to_string()),
            mail_address: Some("john@example.com".to_string()),
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn format_user_row_with_user_id() {
        let text = format_user_row(&sample_user());
        assert!(text.contains("[john]"));
        assert!(text.contains("John Doe"));
    }

    #[test]
    fn format_user_row_without_user_id() {
        let mut u = sample_user();
        u.user_id = None;
        let text = format_user_row(&u);
        assert!(text.contains("[1]"));
        assert!(text.contains("John Doe"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            users: Some(vec![sample_user()]),
        };
        assert!(
            list_with(
                &ProjectUserListArgs {
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
            users: Some(vec![sample_user()]),
        };
        assert!(
            list_with(
                &ProjectUserListArgs {
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
        let api = MockApi { users: None };
        let err = list_with(
            &ProjectUserListArgs {
                key: "TEST".to_string(),
                json: false,
            },
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no users"));
    }
}
