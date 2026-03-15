use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct ProjectDeleteArgs {
    key: String,
    json: bool,
}

impl ProjectDeleteArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn delete(args: &ProjectDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let project = api.delete_project(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&project).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {} ({})", project.name, project.project_key);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::project::Project;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        project: Option<Project>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_project(&self, _key: &str) -> anyhow::Result<Project> {
            self.project.clone().ok_or_else(|| anyhow!("delete failed"))
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

    fn args(json: bool) -> ProjectDeleteArgs {
        ProjectDeleteArgs::new("TEST".to_string(), json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { project: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
