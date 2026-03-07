use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, activity::Activity};

pub fn activities(json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    activities_with(json, &client)
}

pub fn activities_with(json: bool, api: &dyn BacklogApi) -> Result<()> {
    let activities = api.get_space_activities()?;
    if json {
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
        assert!(activities_with(false, &api).is_ok());
    }

    #[test]
    fn activities_with_json_output_succeeds() {
        let api = MockApi {
            activities: Some(vec![sample_activity()]),
        };
        assert!(activities_with(true, &api).is_ok());
    }

    #[test]
    fn activities_with_propagates_api_error() {
        let api = MockApi { activities: None };
        let err = activities_with(false, &api).unwrap_err();
        assert!(err.to_string().contains("no activities"));
    }
}
