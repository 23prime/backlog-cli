use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueParticipantListArgs {
    key: String,
    json: bool,
}

impl IssueParticipantListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &IssueParticipantListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &IssueParticipantListArgs, api: &dyn BacklogApi) -> Result<()> {
    let participants = api.get_issue_participants(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&participants).context("Failed to serialize JSON")?
        );
    } else {
        for p in &participants {
            let uid = p
                .user_id
                .as_deref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| p.id.to_string());
            println!("[{}] {}", uid.cyan().bold(), p.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount, IssueParticipant};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    fn sample_participant() -> IssueParticipant {
        IssueParticipant {
            id: 1,
            user_id: Some("alice".to_string()),
            name: "Alice".to_string(),
            role_type: Some(1),
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    fn bot_participant() -> IssueParticipant {
        IssueParticipant {
            id: 99,
            user_id: None,
            name: "Bot".to_string(),
            role_type: None,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    struct MockApi {
        participants: Option<Vec<IssueParticipant>>,
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
        fn get_issues(&self, _params: &[(String, String)]) -> anyhow::Result<Vec<Issue>> {
            unimplemented!()
        }
        fn count_issues(&self, _params: &[(String, String)]) -> anyhow::Result<IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn create_issue(&self, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn update_issue(&self, _key: &str, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn get_issue_comments(&self, _key: &str) -> anyhow::Result<Vec<IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(&self, _key: &str) -> anyhow::Result<Vec<IssueAttachment>> {
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
            self.participants
                .clone()
                .ok_or_else(|| anyhow!("no participants"))
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

    fn args(json: bool) -> IssueParticipantListArgs {
        IssueParticipantListArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            participants: Some(vec![sample_participant()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_bot_user_id_falls_back_to_numeric_id() {
        let api = MockApi {
            participants: Some(vec![bot_participant()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            participants: Some(vec![sample_participant()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { participants: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no participants"));
    }
}
