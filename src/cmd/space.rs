use anyhow::{Context, Result};

use crate::api::{
    BacklogApi, BacklogClient, activity::Activity, disk_usage::DiskUsage, space::Space,
    space_notification::SpaceNotification,
};

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

pub fn disk_usage(json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    disk_usage_with(json, &client)
}

pub fn disk_usage_with(json: bool, api: &dyn BacklogApi) -> Result<()> {
    let usage = api.get_space_disk_usage()?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&usage).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_disk_usage_text(&usage));
    }
    Ok(())
}

fn format_disk_usage_text(usage: &DiskUsage) -> String {
    format!(
        "Capacity:   {} bytes\nIssue:      {} bytes\nWiki:       {} bytes\nFile:       {} bytes\nSubversion: {} bytes\nGit:        {} bytes\nGit LFS:    {} bytes\nDetails:    {} project(s) — use --json for breakdown",
        usage.capacity,
        usage.issue,
        usage.wiki,
        usage.file,
        usage.subversion,
        usage.git,
        usage.git_lfs,
        usage.details.len(),
    )
}

pub fn notification(json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    notification_with(json, &client)
}

pub fn notification_with(json: bool, api: &dyn BacklogApi) -> Result<()> {
    let n = api.get_space_notification()?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&n).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_notification_text(&n));
    }
    Ok(())
}

fn format_notification_text(n: &SpaceNotification) -> String {
    let updated = n.updated.as_deref().unwrap_or("(not set)");
    let content = if n.content.trim().is_empty() {
        "(no notification set)"
    } else {
        n.content.as_str()
    };
    format!("Updated: {}\n\n{}", updated, content)
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
        disk_usage: Option<DiskUsage>,
        notification: Option<SpaceNotification>,
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

        fn get_space_disk_usage(&self) -> Result<DiskUsage> {
            self.disk_usage
                .clone()
                .ok_or_else(|| anyhow!("no disk usage"))
        }

        fn get_space_notification(&self) -> Result<SpaceNotification> {
            self.notification
                .clone()
                .ok_or_else(|| anyhow!("no notification"))
        }

        fn get_projects(&self) -> Result<Vec<crate::api::project::Project>> {
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
            disk_usage: None,
            notification: None,
        };
        assert!(show_with(false, &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            space: Some(sample_space()),
            activities: None,
            disk_usage: None,
            notification: None,
        };
        assert!(show_with(true, &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: None,
            notification: None,
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
            disk_usage: None,
            notification: None,
        };
        assert!(activities_with(false, &api).is_ok());
    }

    #[test]
    fn activities_with_json_output_succeeds() {
        let api = MockApi {
            space: None,
            activities: Some(vec![sample_activity()]),
            disk_usage: None,
            notification: None,
        };
        assert!(activities_with(true, &api).is_ok());
    }

    #[test]
    fn activities_with_propagates_api_error() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: None,
            notification: None,
        };
        let err = activities_with(false, &api).unwrap_err();
        assert!(err.to_string().contains("no activities"));
    }

    fn sample_disk_usage() -> DiskUsage {
        use crate::api::disk_usage::DiskUsageDetail;
        DiskUsage {
            capacity: 5242880,
            issue: 2048,
            wiki: 512,
            file: 1024,
            subversion: 64,
            git: 256,
            git_lfs: 128,
            details: vec![DiskUsageDetail {
                project_id: 1,
                issue: 1024,
                wiki: 256,
                document: 0,
                file: 512,
                subversion: 32,
                git: 128,
                git_lfs: 64,
            }],
        }
    }

    #[test]
    fn disk_usage_with_text_output_succeeds() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: Some(sample_disk_usage()),
            notification: None,
        };
        assert!(disk_usage_with(false, &api).is_ok());
    }

    #[test]
    fn disk_usage_with_json_output_succeeds() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: Some(sample_disk_usage()),
            notification: None,
        };
        assert!(disk_usage_with(true, &api).is_ok());
    }

    #[test]
    fn disk_usage_with_propagates_api_error() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: None,
            notification: None,
        };
        let err = disk_usage_with(false, &api).unwrap_err();
        assert!(err.to_string().contains("no disk usage"));
    }

    fn sample_notification() -> SpaceNotification {
        SpaceNotification {
            content: "Scheduled maintenance on 2024-07-01.".to_string(),
            updated: Some("2024-06-18T07:55:37Z".to_string()),
        }
    }

    #[test]
    fn notification_with_text_output_succeeds() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: None,
            notification: Some(sample_notification()),
        };
        assert!(notification_with(false, &api).is_ok());
    }

    #[test]
    fn notification_with_json_output_succeeds() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: None,
            notification: Some(sample_notification()),
        };
        assert!(notification_with(true, &api).is_ok());
    }

    #[test]
    fn notification_with_propagates_api_error() {
        let api = MockApi {
            space: None,
            activities: None,
            disk_usage: None,
            notification: None,
        };
        let err = notification_with(false, &api).unwrap_err();
        assert!(err.to_string().contains("no notification"));
    }

    #[test]
    fn format_notification_text_contains_fields() {
        let text = format_notification_text(&sample_notification());
        assert!(text.contains("2024-06-18T07:55:37Z"));
        assert!(text.contains("Scheduled maintenance on 2024-07-01."));
    }

    #[test]
    fn format_notification_text_with_null_updated() {
        let n = SpaceNotification {
            content: "Hello".to_string(),
            updated: None,
        };
        let text = format_notification_text(&n);
        assert!(text.contains("(not set)"));
        assert!(text.contains("Hello"));
    }

    #[test]
    fn format_notification_text_with_empty_content() {
        let n = SpaceNotification {
            content: "".to_string(),
            updated: None,
        };
        let text = format_notification_text(&n);
        assert!(text.contains("(no notification set)"));
    }

    #[test]
    fn format_disk_usage_text_contains_fields() {
        let text = format_disk_usage_text(&sample_disk_usage());
        assert!(text.contains("5242880"));
        assert!(text.contains("2048"));
        assert!(text.contains("128"));
        assert!(text.contains("1 project(s)"));
        assert!(text.contains("--json"));
    }
}
