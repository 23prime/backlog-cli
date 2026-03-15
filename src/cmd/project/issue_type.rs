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
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_users(&self) -> anyhow::Result<Vec<crate::api::user::User>> {
            unimplemented!()
        }
        fn get_user(&self, _user_id: u64) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
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
            _: &[(String, String)],
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
        fn delete_issue_attachment(
            &self,
            _key: &str,
            _attachment_id: u64,
        ) -> anyhow::Result<crate::api::issue::IssueAttachment> {
            unimplemented!()
        }
        fn get_issue_participants(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueParticipant>> {
            unimplemented!()
        }
        fn get_issue_shared_files(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueSharedFile>> {
            unimplemented!()
        }
        fn link_issue_shared_files(
            &self,
            _key: &str,
            _shared_file_ids: &[u64],
        ) -> anyhow::Result<Vec<crate::api::issue::IssueSharedFile>> {
            unimplemented!()
        }
        fn unlink_issue_shared_file(
            &self,
            _key: &str,
            _shared_file_id: u64,
        ) -> anyhow::Result<crate::api::issue::IssueSharedFile> {
            unimplemented!()
        }
        fn get_wikis(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiListItem>> {
            unimplemented!()
        }
        fn get_wiki(&self, _wiki_id: u64) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn create_wiki(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn update_wiki(
            &self,
            _wiki_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn delete_wiki(
            &self,
            _wiki_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn get_wiki_history(
            &self,
            _wiki_id: u64,
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiHistory>> {
            unimplemented!()
        }
        fn get_wiki_attachments(
            &self,
            _wiki_id: u64,
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiAttachment>> {
            unimplemented!()
        }
        fn get_teams(&self, _: &[(String, String)]) -> anyhow::Result<Vec<crate::api::team::Team>> {
            unimplemented!()
        }
        fn get_team(&self, _team_id: u64) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn get_user_activities(
            &self,
            _user_id: u64,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedIssue>> {
            unimplemented!()
        }
        fn get_notifications(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::notification::Notification>> {
            unimplemented!()
        }
        fn count_notifications(
            &self,
        ) -> anyhow::Result<crate::api::notification::NotificationCount> {
            unimplemented!()
        }
        fn read_notification(&self, _: u64) -> anyhow::Result<()> {
            unimplemented!()
        }
        fn reset_unread_notifications(
            &self,
        ) -> anyhow::Result<crate::api::notification::NotificationCount> {
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
