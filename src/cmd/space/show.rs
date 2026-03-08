use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, space::Space};

pub struct SpaceShowArgs {
    pub json: bool,
}

pub fn show(args: &SpaceShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &SpaceShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let space = api.get_space()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&space).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_space_text(&space));
    }
    Ok(())
}

fn format_space_text(space: &Space) -> String {
    format!(
        "Space key:  {}\nName:       {}\nLanguage:   {}\nTimezone:   {}\nFormatting: {}\nCreated:    {}\nUpdated:    {}",
        space.space_key,
        space.name,
        space.lang,
        space.timezone,
        space.text_formatting_rule,
        space.created,
        space.updated,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        space: Option<Space>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> Result<Space> {
            self.space.clone().ok_or_else(|| anyhow!("no space"))
        }
        fn get_myself(&self) -> Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(&self) -> Result<Vec<crate::api::activity::Activity>> {
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
    }

    fn sample_space() -> Space {
        Space {
            space_key: "mycompany".to_string(),
            name: "My Company".to_string(),
            owner_id: 1,
            lang: "ja".to_string(),
            timezone: "Asia/Tokyo".to_string(),
            text_formatting_rule: "markdown".to_string(),
            created: "2020-01-01T00:00:00Z".to_string(),
            updated: "2024-06-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            space: Some(sample_space()),
        };
        assert!(show_with(&SpaceShowArgs { json: false }, &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            space: Some(sample_space()),
        };
        assert!(show_with(&SpaceShowArgs { json: true }, &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { space: None };
        let err = show_with(&SpaceShowArgs { json: false }, &api).unwrap_err();
        assert!(err.to_string().contains("no space"));
    }

    #[test]
    fn format_space_text_contains_all_fields() {
        let text = format_space_text(&sample_space());
        assert!(text.contains("mycompany"));
        assert!(text.contains("My Company"));
        assert!(text.contains("ja"));
        assert!(text.contains("Asia/Tokyo"));
        assert!(text.contains("markdown"));
        assert!(text.contains("2020-01-01T00:00:00Z"));
        assert!(text.contains("2024-06-01T00:00:00Z"));
    }

    #[test]
    fn format_space_text_label_alignment() {
        let text = format_space_text(&sample_space());
        assert!(text.contains("Space key:  mycompany"));
        assert!(text.contains("Name:       My Company"));
    }
}
