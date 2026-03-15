use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, licence::Licence};

pub struct SpaceLicenceArgs {
    json: bool,
}

impl SpaceLicenceArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn licence(args: &SpaceLicenceArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    licence_with(args, &client)
}

pub fn licence_with(args: &SpaceLicenceArgs, api: &dyn BacklogApi) -> Result<()> {
    let l = api.get_space_licence()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&l).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_licence_text(&l));
    }
    Ok(())
}

fn format_licence_text(l: &Licence) -> String {
    let contract = l
        .contract_type
        .as_deref()
        .unwrap_or("(not set)");
    format!(
        "Contract:  {}\nStorage:   {} / {} bytes\nStart:     {}",
        contract, l.storage_usage, l.storage_limit, l.start_date
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        licence: Option<Licence>,
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
        fn get_space_notification(
            &self,
        ) -> Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
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
        fn get_space_licence(&self) -> Result<Licence> {
            self.licence.clone().ok_or_else(|| anyhow!("no licence"))
        }
        fn put_space_notification(
            &self,
            _content: &str,
        ) -> Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
        }
    }

    fn sample_licence() -> Licence {
        Licence {
            start_date: "2020-01-01".to_string(),
            contract_type: Some("premium".to_string()),
            storage_limit: 1073741824,
            storage_usage: 5242880,
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn licence_with_text_output_succeeds() {
        let api = MockApi {
            licence: Some(sample_licence()),
        };
        assert!(licence_with(&SpaceLicenceArgs::new(false), &api).is_ok());
    }

    #[test]
    fn licence_with_json_output_succeeds() {
        let api = MockApi {
            licence: Some(sample_licence()),
        };
        assert!(licence_with(&SpaceLicenceArgs::new(true), &api).is_ok());
    }

    #[test]
    fn licence_with_propagates_api_error() {
        let api = MockApi { licence: None };
        let err = licence_with(&SpaceLicenceArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no licence"));
    }

    #[test]
    fn format_licence_text_contains_fields() {
        let text = format_licence_text(&sample_licence());
        assert!(text.contains("premium"));
        assert!(text.contains("5242880"));
        assert!(text.contains("1073741824"));
        assert!(text.contains("2020-01-01"));
    }

    #[test]
    fn format_licence_text_with_null_contract_type() {
        let l = Licence {
            start_date: "2020-01-01".to_string(),
            contract_type: None,
            storage_limit: 1073741824,
            storage_usage: 0,
            extra: BTreeMap::new(),
        };
        let text = format_licence_text(&l);
        assert!(text.contains("(not set)"));
    }
}
