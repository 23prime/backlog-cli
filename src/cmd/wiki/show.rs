use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, wiki::Wiki};

pub struct WikiShowArgs {
    wiki_id: u64,
    json: bool,
}

impl WikiShowArgs {
    pub fn new(wiki_id: u64, json: bool) -> Self {
        Self { wiki_id, json }
    }
}

pub fn show(args: &WikiShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &WikiShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let wiki = api.get_wiki(args.wiki_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&wiki).context("Failed to serialize JSON")?
        );
    } else {
        print_wiki(&wiki);
    }
    Ok(())
}

pub fn print_wiki(wiki: &Wiki) {
    println!("{}", wiki.name.cyan().bold());
    if !wiki.tags.is_empty() {
        let tag_names: Vec<&str> = wiki.tags.iter().map(|t| t.name.as_str()).collect();
        println!("  Tags:    {}", tag_names.join(", "));
    }
    println!("  Created: {}", wiki.created);
    println!("  Updated: {}", wiki.updated);
    if !wiki.content.is_empty() {
        println!("\n{}", wiki.content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem, WikiTag};
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        wiki: Option<Wiki>,
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
        fn add_user(&self, _params: &[(String, String)]) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn update_user(
            &self,
            _user_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn delete_user(&self, _user_id: u64) -> anyhow::Result<crate::api::user::User> {
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
            self.wiki.clone().ok_or_else(|| anyhow!("no wiki"))
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
            unimplemented!()
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
        fn get_recently_viewed_projects(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedProject>> {
            unimplemented!()
        }
        fn get_recently_viewed_wikis(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedWiki>> {
            unimplemented!()
        }
        fn get_user_stars(
            &self,
            _user_id: u64,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::Star>> {
            unimplemented!()
        }
        fn count_user_stars(&self, _user_id: u64) -> anyhow::Result<crate::api::user::StarCount> {
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

    fn sample_wiki() -> Wiki {
        Wiki {
            id: 1,
            project_id: 1,
            name: "Home".to_string(),
            content: "# Home\nWelcome!".to_string(),
            tags: vec![WikiTag {
                id: 1,
                name: "guide".to_string(),
            }],
            created_user: sample_wiki_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_wiki_user(),
            updated: "2024-01-02T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> WikiShowArgs {
        WikiShowArgs::new(1, json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { wiki: None };
        let err = show_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no wiki"));
    }
}
