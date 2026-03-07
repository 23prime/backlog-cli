use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::Project};

pub fn list(json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(json, &client)
}

pub fn list_with(json: bool, api: &dyn BacklogApi) -> Result<()> {
    let projects = api.get_projects()?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&projects).context("Failed to serialize JSON")?
        );
    } else {
        for p in &projects {
            println!("{}", format_project_text(p));
        }
    }
    Ok(())
}

fn format_project_text(p: &Project) -> String {
    let archived = if p.archived { " [archived]" } else { "" };
    format!("[{}] {}{}", p.project_key, p.name, archived)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        activity::Activity, disk_usage::DiskUsage, project::Project, space::Space,
        space_notification::SpaceNotification, user::User,
    };
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        projects: Option<Vec<Project>>,
    }

    impl BacklogApi for MockApi {
        fn get_space(&self) -> anyhow::Result<Space> {
            unimplemented!()
        }

        fn get_myself(&self) -> anyhow::Result<User> {
            unimplemented!()
        }

        fn get_space_activities(&self) -> anyhow::Result<Vec<Activity>> {
            unimplemented!()
        }

        fn get_space_disk_usage(&self) -> anyhow::Result<DiskUsage> {
            unimplemented!()
        }

        fn get_space_notification(&self) -> anyhow::Result<SpaceNotification> {
            unimplemented!()
        }

        fn get_projects(&self) -> anyhow::Result<Vec<Project>> {
            self.projects.clone().ok_or_else(|| anyhow!("no projects"))
        }
    }

    fn sample_project() -> Project {
        Project {
            id: 1,
            project_key: "TEST".to_string(),
            name: "Test Project".to_string(),
            chart_enabled: false,
            subtasking_enabled: false,
            project_leader_can_edit_project_leader: false,
            text_formatting_rule: "markdown".to_string(),
            archived: false,
            extra: BTreeMap::new(),
        }
    }

    fn sample_archived_project() -> Project {
        Project {
            archived: true,
            ..sample_project()
        }
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            projects: Some(vec![sample_project()]),
        };
        assert!(list_with(false, &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            projects: Some(vec![sample_project()]),
        };
        assert!(list_with(true, &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { projects: None };
        let err = list_with(false, &api).unwrap_err();
        assert!(err.to_string().contains("no projects"));
    }

    #[test]
    fn format_project_text_active() {
        let text = format_project_text(&sample_project());
        assert!(text.contains("[TEST]"));
        assert!(text.contains("Test Project"));
        assert!(!text.contains("[archived]"));
    }

    #[test]
    fn format_project_text_archived() {
        let text = format_project_text(&sample_archived_project());
        assert!(text.contains("[archived]"));
    }
}
