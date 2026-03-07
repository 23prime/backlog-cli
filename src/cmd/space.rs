use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, activity::Activity, space::Space};

pub fn show(json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(json, &client)
}

pub fn show_with(json: bool, api: &dyn BacklogApi) -> Result<()> {
    let space = api.get_space()?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&space).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_space_text(&space));
    }
    Ok(())
}

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
    use crate::api::activity::{Activity, ActivityUser};
    use crate::api::user::User;
    use anyhow::anyhow;

    struct MockApi {
        space: Option<Space>,
        activities: Option<Vec<Activity>>,
    }

    impl BacklogApi for MockApi {
        fn get_space(&self) -> Result<Space> {
            self.space.clone().ok_or_else(|| anyhow!("no space"))
        }

        fn get_myself(&self) -> Result<User> {
            unimplemented!()
        }

        fn get_space_activities(&self) -> Result<Vec<Activity>> {
            self.activities
                .clone()
                .ok_or_else(|| anyhow!("no activities"))
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
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            space: Some(sample_space()),
            activities: None,
        };
        assert!(show_with(false, &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            space: Some(sample_space()),
            activities: None,
        };
        assert!(show_with(true, &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi {
            space: None,
            activities: None,
        };
        let err = show_with(false, &api).unwrap_err();
        assert!(err.to_string().contains("no space"));
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
            space: None,
            activities: Some(vec![sample_activity()]),
        };
        assert!(activities_with(false, &api).is_ok());
    }

    #[test]
    fn activities_with_json_output_succeeds() {
        let api = MockApi {
            space: None,
            activities: Some(vec![sample_activity()]),
        };
        assert!(activities_with(true, &api).is_ok());
    }

    #[test]
    fn activities_with_propagates_api_error() {
        let api = MockApi {
            space: None,
            activities: None,
        };
        let err = activities_with(false, &api).unwrap_err();
        assert!(err.to_string().contains("no activities"));
    }
}
