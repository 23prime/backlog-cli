use anstream::println;
use anyhow::Result;
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient};

pub struct ProjectListArgs {
    json: bool,
}

impl ProjectListArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn list(args: &ProjectListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectListArgs, api: &dyn BacklogApi) -> Result<()> {
    let projects = api.get_projects()?;
    if args.json {
        crate::cmd::print_json(&projects)?;
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
    use crate::cmd::project::sample_project;
    use anyhow::anyhow;

    fn format_project_row(p: &Project) -> String {
        let archived = if p.archived { " [archived]" } else { "" };
        format!("[{}] {}{}", p.project_key, p.name, archived)
    }

    struct MockApi {
        projects: Option<Vec<Project>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_projects(&self) -> anyhow::Result<Vec<Project>> {
            self.projects.clone().ok_or_else(|| anyhow!("no projects"))
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
        assert!(list_with(&ProjectListArgs::new(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            projects: Some(vec![sample_project()]),
        };
        assert!(list_with(&ProjectListArgs::new(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { projects: None };
        let err = list_with(&ProjectListArgs::new(false), &api).unwrap_err();
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
