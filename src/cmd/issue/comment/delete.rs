use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub fn delete(key: &str, comment_id: u64, json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(key, comment_id, json, &client)
}

pub fn delete_with(key: &str, comment_id: u64, json: bool, api: &dyn BacklogApi) -> Result<()> {
    let comment = api.delete_issue_comment(key, comment_id)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&comment).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted comment {} from {}", comment.id, key);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount};
    use crate::cmd::issue::comment::list::sample_comment;
    use anyhow::anyhow;

    struct MockApi {
        comment: Option<IssueComment>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<crate::api::user::User> {
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
            self.comment.clone().ok_or_else(|| anyhow!("delete failed"))
        }
        fn get_issue_attachments(&self, _key: &str) -> anyhow::Result<Vec<IssueAttachment>> {
            unimplemented!()
        }
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(delete_with("TEST-1", 1, false, &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            comment: Some(sample_comment()),
        };
        assert!(delete_with("TEST-1", 1, true, &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { comment: None };
        let err = delete_with("TEST-1", 1, false, &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
