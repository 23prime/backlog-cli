use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::wiki::show::print_wiki;

#[cfg_attr(test, derive(Debug))]
pub struct WikiUpdateArgs {
    wiki_id: u64,
    name: Option<String>,
    content: Option<String>,
    mail_notify: bool,
    json: bool,
}

impl WikiUpdateArgs {
    pub fn new(
        wiki_id: u64,
        name: Option<String>,
        content: Option<String>,
        mail_notify: bool,
        json: bool,
    ) -> Self {
        Self {
            wiki_id,
            name,
            content,
            mail_notify,
            json,
        }
    }
}

pub fn update(args: &WikiUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &WikiUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    if args.name.is_none() && args.content.is_none() {
        return Err(anyhow::anyhow!(
            "at least one of --name or --content must be specified"
        ));
    }
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(name) = &args.name {
        params.push(("name".to_string(), name.clone()));
    }
    if let Some(content) = &args.content {
        params.push(("content".to_string(), content.clone()));
    }
    if args.mail_notify {
        params.push(("mailNotify".to_string(), "true".to_string()));
    }
    let wiki = api.update_wiki(args.wiki_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&wiki).context("Failed to serialize JSON")?
        );
    } else {
        print_wiki(&wiki);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem};
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        wiki: Option<Wiki>,
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
            self.wiki.clone().ok_or_else(|| anyhow!("update failed"))
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
    }

    fn sample_wiki() -> Wiki {
        Wiki {
            id: 1,
            project_id: 1,
            name: "Updated".to_string(),
            content: "# Updated".to_string(),
            tags: vec![],
            created_user: sample_wiki_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_wiki_user(),
            updated: "2024-01-02T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> WikiUpdateArgs {
        WikiUpdateArgs::new(1, Some("Updated".to_string()), None, false, json)
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { wiki: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn update_rejects_no_fields() {
        let api = MockApi { wiki: None };
        let args = WikiUpdateArgs::new(1, None, None, false, false);
        let err = update_with(&args, &api).unwrap_err();
        assert!(
            err.to_string()
                .contains("at least one of --name or --content")
        );
    }
}
