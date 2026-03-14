use anyhow::{Context, Result};

use crate::api::notification::Notification;
use crate::api::{BacklogApi, BacklogClient};

pub struct NotificationListArgs {
    pub json: bool,
    pub min_id: Option<u64>,
    pub max_id: Option<u64>,
    pub count: u32,
    pub order: Option<String>,
    pub sender_id: Option<u64>,
    pub unread: bool,
}

impl NotificationListArgs {
    pub fn try_new(
        json: bool,
        min_id: Option<u64>,
        max_id: Option<u64>,
        count: u32,
        order: Option<String>,
        sender_id: Option<u64>,
        unread: bool,
    ) -> anyhow::Result<Self> {
        if count > 100 {
            anyhow::bail!("count must be between 1 and 100");
        }
        if let (Some(min), Some(max)) = (min_id, max_id)
            && min > max
        {
            anyhow::bail!("min-id must be less than or equal to max-id");
        }
        Ok(Self {
            json,
            min_id,
            max_id,
            count,
            order,
            sender_id,
            unread,
        })
    }
}

pub fn list(args: &NotificationListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &NotificationListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(min) = args.min_id {
        params.push(("minId".to_string(), min.to_string()));
    }
    if let Some(max) = args.max_id {
        params.push(("maxId".to_string(), max.to_string()));
    }
    params.push(("count".to_string(), args.count.to_string()));
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    if let Some(sid) = args.sender_id {
        params.push(("senderId".to_string(), sid.to_string()));
    }
    if args.unread {
        params.push(("alreadyRead".to_string(), "false".to_string()));
    }
    let notifications = api.get_notifications(&params)?;
    if args.json {
        anstream::println!(
            "{}",
            serde_json::to_string_pretty(&notifications).context("Failed to serialize JSON")?
        );
    } else {
        for n in &notifications {
            anstream::println!("{}", format_notification(n));
        }
    }
    Ok(())
}

fn format_notification(n: &Notification) -> String {
    let issue_key = n
        .issue
        .as_ref()
        .map(|i| i.issue_key.as_str())
        .unwrap_or("-");
    format!(
        "[{}] reason={} project={} issue={} read={} created={}",
        n.id, n.reason, n.project.project_key, issue_key, n.already_read, n.created
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::activity::Activity;
    use crate::api::disk_usage::DiskUsage;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount};
    use crate::api::notification::{Notification, NotificationCount};
    use crate::api::project::{
        Project, ProjectCategory, ProjectDiskUsage, ProjectIssueType, ProjectStatus, ProjectUser,
        ProjectVersion,
    };
    use crate::api::space::Space;
    use crate::api::space_notification::SpaceNotification;
    use crate::api::team::Team;
    use crate::api::user::{RecentlyViewedIssue, User};
    use crate::api::wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem};
    use std::collections::BTreeMap;

    struct MockApi {
        notifications: Vec<Notification>,
    }

    impl BacklogApi for MockApi {
        fn get_space(&self) -> Result<Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> Result<User> {
            unimplemented!()
        }
        fn get_users(&self) -> Result<Vec<User>> {
            unimplemented!()
        }
        fn get_user(&self, _: u64) -> Result<User> {
            unimplemented!()
        }
        fn get_space_activities(&self, _: &[(String, String)]) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_space_disk_usage(&self) -> Result<DiskUsage> {
            unimplemented!()
        }
        fn get_space_notification(&self) -> Result<SpaceNotification> {
            unimplemented!()
        }
        fn get_projects(&self) -> Result<Vec<Project>> {
            unimplemented!()
        }
        fn get_project(&self, _: &str) -> Result<Project> {
            unimplemented!()
        }
        fn get_project_activities(&self, _: &str, _: &[(String, String)]) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(&self, _: &str) -> Result<ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(&self, _: &str) -> Result<Vec<ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(&self, _: &str) -> Result<Vec<ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(&self, _: &str) -> Result<Vec<ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(&self, _: &str) -> Result<Vec<ProjectCategory>> {
            unimplemented!()
        }
        fn get_project_versions(&self, _: &str) -> Result<Vec<ProjectVersion>> {
            unimplemented!()
        }
        fn get_issues(&self, _: &[(String, String)]) -> Result<Vec<Issue>> {
            unimplemented!()
        }
        fn count_issues(&self, _: &[(String, String)]) -> Result<IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _: &str) -> Result<Issue> {
            unimplemented!()
        }
        fn create_issue(&self, _: &[(String, String)]) -> Result<Issue> {
            unimplemented!()
        }
        fn update_issue(&self, _: &str, _: &[(String, String)]) -> Result<Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _: &str) -> Result<Issue> {
            unimplemented!()
        }
        fn get_issue_comments(&self, _: &str) -> Result<Vec<IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(&self, _: &str, _: &[(String, String)]) -> Result<IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _: &str,
            _: u64,
            _: &[(String, String)],
        ) -> Result<IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(&self, _: &str, _: u64) -> Result<IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(&self, _: &str) -> Result<Vec<IssueAttachment>> {
            unimplemented!()
        }
        fn get_wikis(&self, _: &[(String, String)]) -> Result<Vec<WikiListItem>> {
            unimplemented!()
        }
        fn get_wiki(&self, _: u64) -> Result<Wiki> {
            unimplemented!()
        }
        fn create_wiki(&self, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn update_wiki(&self, _: u64, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn delete_wiki(&self, _: u64, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn get_wiki_history(&self, _: u64) -> Result<Vec<WikiHistory>> {
            unimplemented!()
        }
        fn get_wiki_attachments(&self, _: u64) -> Result<Vec<WikiAttachment>> {
            unimplemented!()
        }
        fn get_teams(&self, _: &[(String, String)]) -> Result<Vec<Team>> {
            unimplemented!()
        }
        fn get_team(&self, _: u64) -> Result<Team> {
            unimplemented!()
        }
        fn get_user_activities(&self, _: u64, _: &[(String, String)]) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> Result<Vec<RecentlyViewedIssue>> {
            unimplemented!()
        }
        fn get_notifications(&self, _: &[(String, String)]) -> Result<Vec<Notification>> {
            Ok(self.notifications.clone())
        }
        fn count_notifications(&self) -> Result<NotificationCount> {
            unimplemented!()
        }
        fn read_notification(&self, _: u64) -> Result<()> {
            unimplemented!()
        }
        fn reset_unread_notifications(&self) -> Result<NotificationCount> {
            unimplemented!()
        }
    }

    fn make_notification(id: u64, already_read: bool, issue_key: Option<&str>) -> Notification {
        use crate::api::issue::{IssuePriority, IssueStatus, IssueType, IssueUser};
        let project = Project {
            id: 1,
            project_key: "TEST".to_string(),
            name: "Test".to_string(),
            chart_enabled: false,
            subtasking_enabled: false,
            project_leader_can_edit_project_leader: false,
            text_formatting_rule: "markdown".to_string(),
            archived: false,
            extra: BTreeMap::new(),
        };
        let sender = User {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John".to_string(),
            mail_address: None,
            role_type: 1,
            lang: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        };
        let issue = issue_key.map(|key| {
            let u = IssueUser {
                id: 1,
                user_id: Some("john".to_string()),
                name: "John".to_string(),
                role_type: 1,
                lang: None,
                mail_address: None,
                extra: BTreeMap::new(),
            };
            Issue {
                id: 10,
                project_id: 1,
                issue_key: key.to_string(),
                key_id: 1,
                issue_type: IssueType {
                    id: 1,
                    project_id: 1,
                    name: "Bug".to_string(),
                    color: "#ff0000".to_string(),
                    display_order: 0,
                },
                summary: "summary".to_string(),
                description: None,
                resolution: None,
                priority: IssuePriority {
                    id: 2,
                    name: "Normal".to_string(),
                },
                status: IssueStatus {
                    id: 1,
                    project_id: 1,
                    name: "Open".to_string(),
                    color: "#ff0000".to_string(),
                    display_order: 0,
                },
                assignee: None,
                start_date: None,
                due_date: None,
                estimated_hours: None,
                actual_hours: None,
                parent_issue_id: None,
                created_user: u.clone(),
                created: "2024-01-01T00:00:00Z".to_string(),
                updated_user: u,
                updated: "2024-01-01T00:00:00Z".to_string(),
                extra: BTreeMap::new(),
            }
        });
        Notification {
            id,
            already_read,
            reason: 2,
            resource_already_read: false,
            project,
            issue,
            comment: None,
            pull_request: None,
            pull_request_comment: None,
            sender,
            created: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn list_with_issue_formats_correctly() {
        let api = MockApi {
            notifications: vec![make_notification(101, false, Some("TEST-1"))],
        };
        // smoke test: should not panic
        list_with(
            &NotificationListArgs::try_new(false, None, None, 20, None, None, false).unwrap(),
            &api,
        )
        .unwrap();
    }

    #[test]
    fn list_without_issue_shows_dash() {
        let n = make_notification(102, true, None);
        let formatted = format_notification(&n);
        assert!(formatted.contains("issue=-"));
        assert!(formatted.contains("read=true"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(NotificationListArgs::try_new(false, None, None, 101, None, None, false).is_err());
    }

    #[test]
    fn try_new_rejects_min_id_greater_than_max_id() {
        assert!(
            NotificationListArgs::try_new(false, Some(20), Some(10), 20, None, None, false)
                .is_err()
        );
    }

    struct MockApiCapture {
        captured: std::cell::RefCell<Vec<(String, String)>>,
    }

    impl BacklogApi for MockApiCapture {
        fn get_space(&self) -> Result<Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> Result<User> {
            unimplemented!()
        }
        fn get_users(&self) -> Result<Vec<User>> {
            unimplemented!()
        }
        fn get_user(&self, _: u64) -> Result<User> {
            unimplemented!()
        }
        fn get_space_activities(&self, _: &[(String, String)]) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_space_disk_usage(&self) -> Result<DiskUsage> {
            unimplemented!()
        }
        fn get_space_notification(&self) -> Result<SpaceNotification> {
            unimplemented!()
        }
        fn get_projects(&self) -> Result<Vec<Project>> {
            unimplemented!()
        }
        fn get_project(&self, _: &str) -> Result<Project> {
            unimplemented!()
        }
        fn get_project_activities(&self, _: &str, _: &[(String, String)]) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(&self, _: &str) -> Result<ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(&self, _: &str) -> Result<Vec<ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(&self, _: &str) -> Result<Vec<ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(&self, _: &str) -> Result<Vec<ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(&self, _: &str) -> Result<Vec<ProjectCategory>> {
            unimplemented!()
        }
        fn get_project_versions(&self, _: &str) -> Result<Vec<ProjectVersion>> {
            unimplemented!()
        }
        fn get_issues(&self, _: &[(String, String)]) -> Result<Vec<Issue>> {
            unimplemented!()
        }
        fn count_issues(&self, _: &[(String, String)]) -> Result<IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _: &str) -> Result<Issue> {
            unimplemented!()
        }
        fn create_issue(&self, _: &[(String, String)]) -> Result<Issue> {
            unimplemented!()
        }
        fn update_issue(&self, _: &str, _: &[(String, String)]) -> Result<Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _: &str) -> Result<Issue> {
            unimplemented!()
        }
        fn get_issue_comments(&self, _: &str) -> Result<Vec<IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(&self, _: &str, _: &[(String, String)]) -> Result<IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _: &str,
            _: u64,
            _: &[(String, String)],
        ) -> Result<IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(&self, _: &str, _: u64) -> Result<IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(&self, _: &str) -> Result<Vec<IssueAttachment>> {
            unimplemented!()
        }
        fn get_wikis(&self, _: &[(String, String)]) -> Result<Vec<WikiListItem>> {
            unimplemented!()
        }
        fn get_wiki(&self, _: u64) -> Result<Wiki> {
            unimplemented!()
        }
        fn create_wiki(&self, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn update_wiki(&self, _: u64, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn delete_wiki(&self, _: u64, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn get_wiki_history(&self, _: u64) -> Result<Vec<WikiHistory>> {
            unimplemented!()
        }
        fn get_wiki_attachments(&self, _: u64) -> Result<Vec<WikiAttachment>> {
            unimplemented!()
        }
        fn get_teams(&self, _: &[(String, String)]) -> Result<Vec<Team>> {
            unimplemented!()
        }
        fn get_team(&self, _: u64) -> Result<Team> {
            unimplemented!()
        }
        fn get_user_activities(&self, _: u64, _: &[(String, String)]) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> Result<Vec<RecentlyViewedIssue>> {
            unimplemented!()
        }
        fn get_notifications(&self, params: &[(String, String)]) -> Result<Vec<Notification>> {
            *self.captured.borrow_mut() = params.to_vec();
            Ok(vec![])
        }
        fn count_notifications(&self) -> Result<NotificationCount> {
            unimplemented!()
        }
        fn read_notification(&self, _: u64) -> Result<()> {
            unimplemented!()
        }
        fn reset_unread_notifications(&self) -> Result<NotificationCount> {
            unimplemented!()
        }
    }

    #[test]
    fn list_with_builds_correct_query_params() {
        let api = MockApiCapture {
            captured: std::cell::RefCell::new(vec![]),
        };
        let args = NotificationListArgs::try_new(
            false,
            Some(10),
            Some(20),
            50,
            Some("asc".to_string()),
            Some(123),
            true,
        )
        .unwrap();
        list_with(&args, &api).unwrap();
        let params = api.captured.borrow();
        assert!(params.iter().any(|(k, v)| k == "minId" && v == "10"));
        assert!(params.iter().any(|(k, v)| k == "maxId" && v == "20"));
        assert!(params.iter().any(|(k, v)| k == "count" && v == "50"));
        assert!(params.iter().any(|(k, v)| k == "order" && v == "asc"));
        assert!(params.iter().any(|(k, v)| k == "senderId" && v == "123"));
        assert!(
            params
                .iter()
                .any(|(k, v)| k == "alreadyRead" && v == "false")
        );
    }
}
