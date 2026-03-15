use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, user::RecentlyViewedProject};

pub struct UserRecentlyViewedProjectsArgs {
    json: bool,
    pub count: u32,
    pub offset: u64,
    pub order: Option<String>,
}

impl UserRecentlyViewedProjectsArgs {
    pub fn try_new(
        json: bool,
        count: u32,
        offset: u64,
        order: Option<String>,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        Ok(Self {
            json,
            count,
            offset,
            order,
        })
    }
}

pub fn recently_viewed_projects(args: &UserRecentlyViewedProjectsArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    recently_viewed_projects_with(args, &client)
}

pub fn recently_viewed_projects_with(
    args: &UserRecentlyViewedProjectsArgs,
    api: &dyn BacklogApi,
) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("count".to_string(), args.count.to_string()));
    params.push(("offset".to_string(), args.offset.to_string()));
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    let items = api.get_recently_viewed_projects(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&items).context("Failed to serialize JSON")?
        );
    } else {
        for item in &items {
            println!("{}", format_row(item));
        }
    }
    Ok(())
}

fn format_row(item: &RecentlyViewedProject) -> String {
    format!("[{}] {}", item.project.project_key, item.project.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::{ProjectSummary, RecentlyViewedWiki, Star, StarCount};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        items: Option<Vec<RecentlyViewedProject>>,
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
            self.items.clone().ok_or_else(|| anyhow!("no items"))
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
            unimplemented!()
        }
        fn count_user_stars(&self, _user_id: u64) -> anyhow::Result<StarCount> {
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

    fn sample_item() -> RecentlyViewedProject {
        RecentlyViewedProject {
            project: ProjectSummary {
                id: 1,
                project_key: "TEST".to_string(),
                name: "Test Project".to_string(),
                extra: BTreeMap::new(),
            },
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn format_row_contains_fields() {
        let text = format_row(&sample_item());
        assert!(text.contains("[TEST]"));
        assert!(text.contains("Test Project"));
    }

    #[test]
    fn recently_viewed_projects_with_text_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(
            recently_viewed_projects_with(
                &UserRecentlyViewedProjectsArgs::try_new(false, 20, 0, None).unwrap(),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn recently_viewed_projects_with_json_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(
            recently_viewed_projects_with(
                &UserRecentlyViewedProjectsArgs::try_new(true, 20, 0, None).unwrap(),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn recently_viewed_projects_with_propagates_api_error() {
        let api = MockApi { items: None };
        let err = recently_viewed_projects_with(
            &UserRecentlyViewedProjectsArgs::try_new(false, 20, 0, None).unwrap(),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no items"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(UserRecentlyViewedProjectsArgs::try_new(false, 101, 0, None).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(UserRecentlyViewedProjectsArgs::try_new(false, 0, 0, None).is_err());
    }
}
