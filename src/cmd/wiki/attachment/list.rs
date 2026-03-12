use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, wiki::WikiAttachment};

pub struct WikiAttachmentListArgs {
    wiki_id: u64,
    json: bool,
}

impl WikiAttachmentListArgs {
    pub fn new(wiki_id: u64, json: bool) -> Self {
        Self { wiki_id, json }
    }
}

pub fn list(args: &WikiAttachmentListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &WikiAttachmentListArgs, api: &dyn BacklogApi) -> Result<()> {
    let attachments = api.get_wiki_attachments(args.wiki_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&attachments).context("Failed to serialize JSON")?
        );
    } else {
        for a in &attachments {
            println!("{}", format_attachment_row(a));
        }
    }
    Ok(())
}

pub fn format_attachment_row(a: &WikiAttachment) -> String {
    format!(
        "[{}] {} ({} bytes)",
        a.id.to_string().cyan(),
        a.name,
        a.size
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem};
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use anyhow::anyhow;

    struct MockApi {
        attachments: Option<Vec<WikiAttachment>>,
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
        fn get_space_activities(&self) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
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
            unimplemented!()
        }
        fn get_wiki_attachments(&self, _wiki_id: u64) -> anyhow::Result<Vec<WikiAttachment>> {
            self.attachments
                .clone()
                .ok_or_else(|| anyhow!("no attachments"))
        }
    }

    fn sample_attachment() -> WikiAttachment {
        WikiAttachment {
            id: 1,
            name: "image.png".to_string(),
            size: 2048,
            created_user: sample_wiki_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    fn args(json: bool) -> WikiAttachmentListArgs {
        WikiAttachmentListArgs::new(1, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_attachment()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_attachment()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { attachments: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no attachments"));
    }

    #[test]
    fn format_attachment_row_contains_id_name_size() {
        let row = format_attachment_row(&sample_attachment());
        assert!(row.contains("1"));
        assert!(row.contains("image.png"));
        assert!(row.contains("2048"));
    }
}
