use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, activity::Activity};

pub struct SpaceActivitiesArgs {
    pub json: bool,
}

pub fn activities(args: &SpaceActivitiesArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    activities_with(args, &client)
}

pub fn activities_with(args: &SpaceActivitiesArgs, api: &dyn BacklogApi) -> Result<()> {
    let activities = api.get_space_activities()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&activities).context("Failed to serialize JSON")?
        );
    } else {
        for a in &activities {
            println!("{}", format_activity_text(a));
        }
    }
    Ok(())
}

fn format_activity_text(a: &Activity) -> String {
    let project = a
        .project
        .as_ref()
        .map(|p| p.project_key.as_str())
        .unwrap_or("-");
    format!(
        "[{}] type={} project={} user={} created={}",
        a.id, a.activity_type, project, a.created_user.name, a.created,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::activity::ActivityUser;
    use anyhow::anyhow;

    struct MockApi {
        activities: Option<Vec<Activity>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(&self) -> Result<Vec<Activity>> {
            self.activities
                .clone()
                .ok_or_else(|| anyhow!("no activities"))
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

    fn sample_activity() -> Activity {
        Activity {
            id: 1,
            project: None,
            activity_type: 1,
            content: serde_json::Value::Null,
            created_user: ActivityUser {
                id: 100,
                user_id: Some("john".to_string()),
                name: "John Doe".to_string(),
                extra: Default::default(),
            },
            created: "2024-01-01T00:00:00Z".to_string(),
            extra: Default::default(),
        }
    }

    #[test]
    fn format_activity_text_contains_fields() {
        let text = format_activity_text(&sample_activity());
        assert!(text.contains("[1]"));
        assert!(text.contains("type=1"));
        assert!(text.contains("project=-"));
        assert!(text.contains("John Doe"));
        assert!(text.contains("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn activities_with_text_output_succeeds() {
        let api = MockApi {
            activities: Some(vec![sample_activity()]),
        };
        assert!(activities_with(&SpaceActivitiesArgs { json: false }, &api).is_ok());
    }

    #[test]
    fn activities_with_json_output_succeeds() {
        let api = MockApi {
            activities: Some(vec![sample_activity()]),
        };
        assert!(activities_with(&SpaceActivitiesArgs { json: true }, &api).is_ok());
    }

    #[test]
    fn activities_with_propagates_api_error() {
        let api = MockApi { activities: None };
        let err = activities_with(&SpaceActivitiesArgs { json: false }, &api).unwrap_err();
        assert!(err.to_string().contains("no activities"));
    }
}
