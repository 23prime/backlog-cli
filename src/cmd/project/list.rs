use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient};

pub struct ProjectListArgs {
    json: bool,
}

impl ProjectListArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn list(args: &ProjectListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectListArgs, api: &dyn BacklogApi) -> Result<()> {
    let projects = api.get_projects()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&projects).context("Failed to serialize JSON")?
        );
    } else {
        for p in &projects {
            let archived = if p.archived {
                format!(" {}", "[archived]".yellow())
            } else {
                String::new()
            };
            println!("[{}] {}{}", p.project_key.cyan().bold(), p.name, archived);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::project::Project;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    fn format_project_row(p: &Project) -> String {
        let archived = if p.archived { " [archived]" } else { "" };
        format!("[{}] {}{}", p.project_key, p.name, archived)
    }

    struct MockApi {
        projects: Option<Vec<Project>>,
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
        fn get_projects(&self) -> anyhow::Result<Vec<Project>> {
            self.projects.clone().ok_or_else(|| anyhow!("no projects"))
        }
        fn get_project(&self, _key: &str) -> anyhow::Result<Project> {
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
        fn count_issue_comments(
            &self,
            _key: &str,
        ) -> anyhow::Result<crate::api::issue::IssueCommentCount> {
            unimplemented!()
        }
        fn get_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn get_issue_comment_notifications(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueCommentNotification>> {
            unimplemented!()
        }
        fn add_issue_comment_notifications(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::issue::IssueCommentNotification>> {
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

    fn sample_project() -> Project {
        Project {
            id: 1,
            project_key: "TEST".to_string(),
            name: "Test Project".to_string(),
            chart_enabled: false,
            subtasking_enabled: false,
            project_leader_can_edit_project_leader: false,
            text_formatting_rule: "markdown".to_string(),
            archived: false,
            extra: BTreeMap::new(),
        }
    }

    fn sample_archived_project() -> Project {
        Project {
            archived: true,
            ..sample_project()
        }
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            projects: Some(vec![sample_project()]),
        };
        assert!(list_with(&ProjectListArgs::new(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            projects: Some(vec![sample_project()]),
        };
        assert!(list_with(&ProjectListArgs::new(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { projects: None };
        let err = list_with(&ProjectListArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no projects"));
    }

    #[test]
    fn format_project_row_active() {
        let text = format_project_row(&sample_project());
        assert!(text.contains("[TEST]"));
        assert!(text.contains("Test Project"));
        assert!(!text.contains("[archived]"));
    }

    #[test]
    fn format_project_row_archived() {
        let text = format_project_row(&sample_archived_project());
        assert!(text.contains("[archived]"));
    }
}
