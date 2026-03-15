use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueSharedFileUnlinkArgs {
    key: String,
    shared_file_id: u64,
    json: bool,
}

impl IssueSharedFileUnlinkArgs {
    pub fn new(key: String, shared_file_id: u64, json: bool) -> Self {
        Self {
            key,
            shared_file_id,
            json,
        }
    }
}

pub fn unlink(args: &IssueSharedFileUnlinkArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    unlink_with(args, &client)
}

pub fn unlink_with(args: &IssueSharedFileUnlinkArgs, api: &dyn BacklogApi) -> Result<()> {
    let file = api.unlink_issue_shared_file(&args.key, args.shared_file_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&file).context("Failed to serialize JSON")?
        );
    } else {
        println!("Unlinked: {}", file.name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount, IssueSharedFile};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    fn sample_shared_file() -> IssueSharedFile {
        IssueSharedFile {
            id: 1,
            dir: "/docs".to_string(),
            name: "spec.pdf".to_string(),
            size: 2048,
            extra: BTreeMap::new(),
        }
    }

    struct MockApi {
        file: Option<IssueSharedFile>,
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
            self.file.clone().ok_or_else(|| anyhow!("unlink failed"))
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

    fn args(json: bool) -> IssueSharedFileUnlinkArgs {
        IssueSharedFileUnlinkArgs::new("TEST-1".to_string(), 1, json)
    }

    #[test]
    fn unlink_with_text_output_succeeds() {
        let api = MockApi {
            file: Some(sample_shared_file()),
        };
        assert!(unlink_with(&args(false), &api).is_ok());
    }

    #[test]
    fn unlink_with_json_output_succeeds() {
        let api = MockApi {
            file: Some(sample_shared_file()),
        };
        assert!(unlink_with(&args(true), &api).is_ok());
    }

    #[test]
    fn unlink_with_propagates_api_error() {
        let api = MockApi { file: None };
        let err = unlink_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("unlink failed"));
    }
}
