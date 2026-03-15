use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, space_notification::SpaceNotification};

pub struct SpaceNotificationArgs {
    json: bool,
}

impl SpaceNotificationArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn notification(args: &SpaceNotificationArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    notification_with(args, &client)
}

pub fn notification_with(args: &SpaceNotificationArgs, api: &dyn BacklogApi) -> Result<()> {
    let n = api.get_space_notification()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&n).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_notification_text(&n));
    }
    Ok(())
}

fn format_notification_text(n: &SpaceNotification) -> String {
    let updated = n.updated.as_deref().unwrap_or("(not set)");
    let content = if n.content.trim().is_empty() {
        "(no notification set)"
    } else {
        n.content.as_str()
    };
    format!("Updated: {}\n\n{}", updated, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        notification: Option<SpaceNotification>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> Result<crate::api::user::User> {
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
        ) -> Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_space_disk_usage(&self) -> Result<crate::api::disk_usage::DiskUsage> {
            unimplemented!()
        }
        fn get_space_notification(&self) -> Result<SpaceNotification> {
            self.notification
                .clone()
                .ok_or_else(|| anyhow!("no notification"))
        }
        fn get_projects(&self) -> Result<Vec<crate::api::project::Project>> {
            unimplemented!()
        }
        fn get_project(&self, _key: &str) -> Result<crate::api::project::Project> {
            unimplemented!()
        }
        fn get_project_activities(
            &self,
            _key: &str,
            _: &[(String, String)],
        ) -> Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(
            &self,
            _key: &str,
        ) -> Result<crate::api::project::ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(&self, _key: &str) -> Result<Vec<crate::api::project::ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(
            &self,
            _key: &str,
        ) -> Result<Vec<crate::api::project::ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(
            &self,
            _key: &str,
        ) -> Result<Vec<crate::api::project::ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(
            &self,
            _key: &str,
        ) -> Result<Vec<crate::api::project::ProjectCategory>> {
            unimplemented!()
        }
        fn get_project_versions(
            &self,
            _key: &str,
        ) -> Result<Vec<crate::api::project::ProjectVersion>> {
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

    fn sample_notification() -> SpaceNotification {
        SpaceNotification {
            content: "Scheduled maintenance on 2024-07-01.".to_string(),
            updated: Some("2024-06-18T07:55:37Z".to_string()),
        }
    }

    #[test]
    fn notification_with_text_output_succeeds() {
        let api = MockApi {
            notification: Some(sample_notification()),
        };
        assert!(notification_with(&SpaceNotificationArgs::new(false), &api).is_ok());
    }

    #[test]
    fn notification_with_json_output_succeeds() {
        let api = MockApi {
            notification: Some(sample_notification()),
        };
        assert!(notification_with(&SpaceNotificationArgs::new(true), &api).is_ok());
    }

    #[test]
    fn notification_with_propagates_api_error() {
        let api = MockApi { notification: None };
        let err = notification_with(&SpaceNotificationArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no notification"));
    }

    #[test]
    fn format_notification_text_contains_fields() {
        let text = format_notification_text(&sample_notification());
        assert!(text.contains("2024-06-18T07:55:37Z"));
        assert!(text.contains("Scheduled maintenance on 2024-07-01."));
    }

    #[test]
    fn format_notification_text_with_null_updated() {
        let n = SpaceNotification {
            content: "Hello".to_string(),
            updated: None,
        };
        let text = format_notification_text(&n);
        assert!(text.contains("(not set)"));
        assert!(text.contains("Hello"));
    }

    #[test]
    fn format_notification_text_with_empty_content() {
        let n = SpaceNotification {
            content: "".to_string(),
            updated: None,
        };
        let text = format_notification_text(&n);
        assert!(text.contains("(no notification set)"));
    }
}
