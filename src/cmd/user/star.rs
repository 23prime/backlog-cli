use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, user::Star};

pub struct UserStarListArgs {
    user_id: u64,
    json: bool,
}

impl UserStarListArgs {
    pub fn new(user_id: u64, json: bool) -> Self {
        Self { user_id, json }
    }
}

pub fn star_list(args: &UserStarListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    star_list_with(args, &client)
}

pub fn star_list_with(args: &UserStarListArgs, api: &dyn BacklogApi) -> Result<()> {
    let stars = api.get_user_stars(args.user_id, &[])?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&stars).context("Failed to serialize JSON")?
        );
    } else {
        for star in &stars {
            println!("{}", format_star_row(star));
        }
    }
    Ok(())
}

fn format_star_row(star: &Star) -> String {
    format!("[{}] {} — {}", star.id, star.title, star.url)
}

pub struct UserStarCountArgs {
    user_id: u64,
    json: bool,
}

impl UserStarCountArgs {
    pub fn new(user_id: u64, json: bool) -> Self {
        Self { user_id, json }
    }
}

pub fn star_count(args: &UserStarCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    star_count_with(args, &client)
}

pub fn star_count_with(args: &UserStarCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let count = api.count_user_stars(args.user_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&count).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", count.count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::{RecentlyViewedProject, RecentlyViewedWiki, StarCount, User};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        stars: Option<Vec<Star>>,
        count: Option<StarCount>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn get_users(&self) -> anyhow::Result<Vec<User>> {
            unimplemented!()
        }
        fn get_user(&self, _user_id: u64) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn add_user(&self, _params: &[(String, String)]) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn update_user(&self, _user_id: u64, _params: &[(String, String)]) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn delete_user(&self, _user_id: u64) -> anyhow::Result<User> {
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
        fn get_recently_viewed_projects(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<RecentlyViewedProject>> {
            unimplemented!()
        }
        fn get_recently_viewed_wikis(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<RecentlyViewedWiki>> {
            unimplemented!()
        }
        fn get_user_stars(
            &self,
            _user_id: u64,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<Star>> {
            self.stars.clone().ok_or_else(|| anyhow!("no stars"))
        }
        fn count_user_stars(&self, _user_id: u64) -> anyhow::Result<StarCount> {
            self.count.clone().ok_or_else(|| anyhow!("no count"))
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

    fn sample_presenter() -> User {
        User {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John".to_string(),
            mail_address: None,
            role_type: 1,
            lang: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    fn sample_star() -> Star {
        Star {
            id: 5,
            comment: None,
            url: "https://example.backlog.com/view/TEST-1".to_string(),
            title: "Fix bug".to_string(),
            presenter: sample_presenter(),
            created: "2024-06-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn format_star_row_contains_fields() {
        let text = format_star_row(&sample_star());
        assert!(text.contains("[5]"));
        assert!(text.contains("Fix bug"));
        assert!(text.contains("https://example.backlog.com/view/TEST-1"));
    }

    #[test]
    fn star_list_with_text_output_succeeds() {
        let api = MockApi {
            stars: Some(vec![sample_star()]),
            count: None,
        };
        assert!(star_list_with(&UserStarListArgs::new(1, false), &api).is_ok());
    }

    #[test]
    fn star_list_with_json_output_succeeds() {
        let api = MockApi {
            stars: Some(vec![sample_star()]),
            count: None,
        };
        assert!(star_list_with(&UserStarListArgs::new(1, true), &api).is_ok());
    }

    #[test]
    fn star_list_with_propagates_api_error() {
        let api = MockApi {
            stars: None,
            count: None,
        };
        let err = star_list_with(&UserStarListArgs::new(1, false), &api).unwrap_err();
        assert!(err.to_string().contains("no stars"));
    }

    #[test]
    fn star_count_with_text_output_succeeds() {
        let api = MockApi {
            stars: None,
            count: Some(StarCount { count: 42 }),
        };
        assert!(star_count_with(&UserStarCountArgs::new(1, false), &api).is_ok());
    }

    #[test]
    fn star_count_with_json_output_succeeds() {
        let api = MockApi {
            stars: None,
            count: Some(StarCount { count: 42 }),
        };
        assert!(star_count_with(&UserStarCountArgs::new(1, true), &api).is_ok());
    }

    #[test]
    fn star_count_with_propagates_api_error() {
        let api = MockApi {
            stars: None,
            count: None,
        };
        let err = star_count_with(&UserStarCountArgs::new(1, false), &api).unwrap_err();
        assert!(err.to_string().contains("no count"));
    }
}
