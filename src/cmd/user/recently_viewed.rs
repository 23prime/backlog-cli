use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, user::RecentlyViewedIssue};

pub struct UserRecentlyViewedArgs {
    json: bool,
}

impl UserRecentlyViewedArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn recently_viewed(args: &UserRecentlyViewedArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    recently_viewed_with(args, &client)
}

pub fn recently_viewed_with(args: &UserRecentlyViewedArgs, api: &dyn BacklogApi) -> Result<()> {
    let items = api.get_recently_viewed_issues()?;
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

fn format_row(item: &RecentlyViewedIssue) -> String {
    let status = &item.issue.status.name;
    let assignee = item
        .issue
        .assignee
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("-");
    format!(
        "[{}] {} ({}, {})",
        item.issue.issue_key, item.issue.summary, status, assignee,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::activity::Activity;
    use crate::api::issue::{Issue, IssuePriority, IssueStatus, IssueType, IssueUser};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        items: Option<Vec<RecentlyViewedIssue>>,
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
        fn get_space_activities(&self) -> anyhow::Result<Vec<Activity>> {
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
        fn get_project_activities(&self, _key: &str) -> anyhow::Result<Vec<Activity>> {
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
        fn get_teams(&self) -> anyhow::Result<Vec<crate::api::team::Team>> {
            unimplemented!()
        }
        fn get_team(&self, _team_id: u64) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn get_user_activities(&self, _user_id: u64) -> anyhow::Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(&self) -> anyhow::Result<Vec<RecentlyViewedIssue>> {
            self.items.clone().ok_or_else(|| anyhow!("no items"))
        }
    }

    fn sample_issue_user() -> IssueUser {
        IssueUser {
            id: 1,
            user_id: Some("admin".to_string()),
            name: "admin".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    fn sample_issue() -> Issue {
        Issue {
            id: 1,
            project_id: 1,
            issue_key: "BLG-1".to_string(),
            key_id: 1,
            summary: "Fix login".to_string(),
            description: None,
            resolution: None,
            status: IssueStatus {
                id: 1,
                project_id: 1,
                name: "Open".to_string(),
                color: "#ed8077".to_string(),
                display_order: 1000,
            },
            priority: IssuePriority {
                id: 2,
                name: "Normal".to_string(),
            },
            issue_type: IssueType {
                id: 1,
                project_id: 1,
                name: "Bug".to_string(),
                color: "#990000".to_string(),
                display_order: 0,
            },
            assignee: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            actual_hours: None,
            parent_issue_id: None,
            created_user: sample_issue_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_issue_user(),
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn sample_item() -> RecentlyViewedIssue {
        RecentlyViewedIssue {
            issue: sample_issue(),
            updated: "2024-06-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn format_row_contains_fields() {
        let text = format_row(&sample_item());
        assert!(text.contains("[BLG-1]"));
        assert!(text.contains("Fix login"));
        assert!(text.contains("Open"));
        assert!(text.contains('-'));
    }

    #[test]
    fn recently_viewed_with_text_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(recently_viewed_with(&UserRecentlyViewedArgs::new(false), &api).is_ok());
    }

    #[test]
    fn recently_viewed_with_json_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(recently_viewed_with(&UserRecentlyViewedArgs::new(true), &api).is_ok());
    }

    #[test]
    fn recently_viewed_with_propagates_api_error() {
        let api = MockApi { items: None };
        let err = recently_viewed_with(&UserRecentlyViewedArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no items"));
    }
}
