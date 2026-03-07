use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient};

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
            let archived = if p.archived {
                format!(" {}", "[archived]".yellow())
            } else {
                String::new()
            };
            println!("[{}] {}{}", p.project_key.cyan().bold(), p.name, archived);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::project::Project;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    fn format_project_row(p: &Project) -> String {
        let archived = if p.archived { " [archived]" } else { "" };
        format!("[{}] {}{}", p.project_key, p.name, archived)
    }

    struct MockApi {
        projects: Option<Vec<Project>>,
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
        fn get_projects(&self) -> anyhow::Result<Vec<Project>> {
            self.projects.clone().ok_or_else(|| anyhow!("no projects"))
        }
        fn get_project(&self, _key: &str) -> anyhow::Result<Project> {
            unimplemented!()
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
    fn format_project_row_active() {
        let text = format_project_row(&sample_project());
        assert!(text.contains("[TEST]"));
        assert!(text.contains("Test Project"));
        assert!(!text.contains("[archived]"));
    }

    #[test]
    fn format_project_row_archived() {
        let text = format_project_row(&sample_archived_project());
        assert!(text.contains("[archived]"));
    }
}
