use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, wiki::WikiListItem};

pub struct WikiListArgs {
    project_id_or_key: String,
    keyword: Option<String>,
    json: bool,
}

impl WikiListArgs {
    pub fn new(project_id_or_key: String, keyword: Option<String>, json: bool) -> Self {
        Self {
            project_id_or_key,
            keyword,
            json,
        }
    }
}

pub fn list(args: &WikiListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &WikiListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> =
        vec![("projectIdOrKey".to_string(), args.project_id_or_key.clone())];
    if let Some(kw) = &args.keyword {
        params.push(("keyword".to_string(), kw.clone()));
    }
    let wikis = api.get_wikis(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&wikis).context("Failed to serialize JSON")?
        );
    } else {
        for wiki in &wikis {
            println!("{}", format_wiki_row(wiki));
        }
    }
    Ok(())
}

pub fn format_wiki_row(wiki: &WikiListItem) -> String {
    let tags = if wiki.tags.is_empty() {
        String::new()
    } else {
        format!(
            " [{}]",
            wiki.tags
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    format!("{}{}", wiki.name.cyan().bold(), tags)
}

#[cfg(test)]
pub(crate) mod tests_helper {
    use std::collections::BTreeMap;

    use crate::api::wiki::{WikiListItem, WikiTag, WikiUser};

    pub fn sample_wiki_user() -> WikiUser {
        WikiUser {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    pub fn sample_wiki_list_item() -> WikiListItem {
        WikiListItem {
            id: 1,
            project_id: 1,
            name: "Home".to_string(),
            tags: vec![WikiTag {
                id: 1,
                name: "guide".to_string(),
            }],
            created_user: sample_wiki_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_wiki_user(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem};
    use anyhow::anyhow;
    use tests_helper::{sample_wiki_list_item, sample_wiki_user};

    struct MockApi {
        wikis: Option<Vec<WikiListItem>>,
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
            self.wikis.clone().ok_or_else(|| anyhow!("no wikis"))
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
        fn get_space_licence(&self) -> anyhow::Result<crate::api::licence::Licence> {
            unimplemented!()
        }
        fn put_space_notification(&self, _content: &str) -> anyhow::Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
        }
    }

    fn args(json: bool) -> WikiListArgs {
        WikiListArgs::new("TEST".to_string(), None, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            wikis: Some(vec![sample_wiki_list_item()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            wikis: Some(vec![sample_wiki_list_item()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { wikis: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no wikis"));
    }

    #[test]
    fn format_wiki_row_with_tags() {
        let wiki = sample_wiki_list_item();
        let row = format_wiki_row(&wiki);
        assert!(row.contains("Home"));
        assert!(row.contains("guide"));
    }

    #[test]
    fn format_wiki_row_without_tags() {
        let mut wiki = sample_wiki_list_item();
        wiki.tags.clear();
        let row = format_wiki_row(&wiki);
        assert!(row.contains("Home"));
        assert!(!row.contains("guide"));
    }

    #[test]
    fn list_with_keyword_builds_params() {
        let api = MockApi {
            wikis: Some(vec![]),
        };
        let args = WikiListArgs::new("TEST".to_string(), Some("guide".to_string()), false);
        assert!(list_with(&args, &api).is_ok());
    }

    #[test]
    fn sample_wiki_user_has_expected_fields() {
        let u = sample_wiki_user();
        assert_eq!(u.name, "John Doe");
        assert_eq!(u.user_id.as_deref(), Some("john"));
    }
}
