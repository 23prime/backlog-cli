use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, wiki::WikiHistory};

pub struct WikiHistoryArgs {
    wiki_id: u64,
    json: bool,
}

impl WikiHistoryArgs {
    pub fn new(wiki_id: u64, json: bool) -> Self {
        Self { wiki_id, json }
    }
}

pub fn history(args: &WikiHistoryArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    history_with(args, &client)
}

pub fn history_with(args: &WikiHistoryArgs, api: &dyn BacklogApi) -> Result<()> {
    let entries = api.get_wiki_history(args.wiki_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&entries).context("Failed to serialize JSON")?
        );
    } else {
        for entry in &entries {
            println!("{}", format_history_row(entry));
        }
    }
    Ok(())
}

pub fn format_history_row(entry: &WikiHistory) -> String {
    format!(
        "{} {} — {}",
        format!("v{}", entry.version).cyan().bold(),
        entry.name,
        entry.created
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem, WikiUser};
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use anyhow::anyhow;

    struct MockApi {
        history: Option<Vec<WikiHistory>>,
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
        fn get_wikis(&self, _params: &[(String, String)]) -> anyhow::Result<Vec<WikiListItem>> {
            unimplemented!()
        }
        fn get_wiki(&self, _wiki_id: u64) -> anyhow::Result<Wiki> {
            unimplemented!()
        }
        fn create_wiki(&self, _params: &[(String, String)]) -> anyhow::Result<Wiki> {
            unimplemented!()
        }
        fn update_wiki(&self, _wiki_id: u64, _params: &[(String, String)]) -> anyhow::Result<Wiki> {
            unimplemented!()
        }
        fn delete_wiki(&self, _wiki_id: u64, _params: &[(String, String)]) -> anyhow::Result<Wiki> {
            unimplemented!()
        }
        fn get_wiki_history(&self, _wiki_id: u64) -> anyhow::Result<Vec<WikiHistory>> {
            self.history.clone().ok_or_else(|| anyhow!("no history"))
        }
        fn get_wiki_attachments(&self, _wiki_id: u64) -> anyhow::Result<Vec<WikiAttachment>> {
            unimplemented!()
        }
        fn get_teams(&self, _: &[(String, String)]) -> anyhow::Result<Vec<crate::api::team::Team>> {
            unimplemented!()
        }
        fn get_team(&self, _team_id: u64) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn add_team(&self, _params: &[(String, String)]) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn update_team(
            &self,
            _team_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn delete_team(&self, _team_id: u64) -> anyhow::Result<crate::api::team::Team> {
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

    fn sample_history(user: WikiUser) -> WikiHistory {
        WikiHistory {
            page_id: 1,
            version: 3,
            name: "Home".to_string(),
            content: "# Home v3".to_string(),
            created_user: user,
            created: "2024-03-01T00:00:00Z".to_string(),
        }
    }

    fn args(json: bool) -> WikiHistoryArgs {
        WikiHistoryArgs::new(1, json)
    }

    #[test]
    fn history_with_text_output_succeeds() {
        let api = MockApi {
            history: Some(vec![sample_history(sample_wiki_user())]),
        };
        assert!(history_with(&args(false), &api).is_ok());
    }

    #[test]
    fn history_with_json_output_succeeds() {
        let api = MockApi {
            history: Some(vec![sample_history(sample_wiki_user())]),
        };
        assert!(history_with(&args(true), &api).is_ok());
    }

    #[test]
    fn history_with_propagates_api_error() {
        let api = MockApi { history: None };
        let err = history_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no history"));
    }

    #[test]
    fn format_history_row_contains_version_and_name() {
        let entry = sample_history(sample_wiki_user());
        let row = format_history_row(&entry);
        assert!(row.contains("v3"));
        assert!(row.contains("Home"));
        assert!(row.contains("2024-03-01"));
    }
}
