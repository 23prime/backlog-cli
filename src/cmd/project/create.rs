use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::Project};

pub struct ProjectCreateArgs {
    name: String,
    key: String,
    chart_enabled: bool,
    subtasking_enabled: bool,
    text_formatting_rule: String,
    json: bool,
}

impl ProjectCreateArgs {
    pub fn new(
        name: String,
        key: String,
        chart_enabled: bool,
        subtasking_enabled: bool,
        text_formatting_rule: String,
        json: bool,
    ) -> Self {
        Self {
            name,
            key,
            chart_enabled,
            subtasking_enabled,
            text_formatting_rule,
            json,
        }
    }
}

pub fn create(args: &ProjectCreateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    create_with(args, &client)
}

pub fn create_with(args: &ProjectCreateArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![
        ("name".to_string(), args.name.clone()),
        ("key".to_string(), args.key.clone()),
        ("chartEnabled".to_string(), args.chart_enabled.to_string()),
        (
            "subtaskingEnabled".to_string(),
            args.subtasking_enabled.to_string(),
        ),
        (
            "textFormattingRule".to_string(),
            args.text_formatting_rule.clone(),
        ),
    ];

    let project = api.create_project(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&project).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_project(&project));
    }
    Ok(())
}

fn format_project(p: &Project) -> String {
    format!(
        "ID:         {}\nKey:        {}\nName:       {}\nFormatting: {}\nArchived:   {}",
        p.id, p.project_key, p.name, p.text_formatting_rule, p.archived,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::project::sample_project;
    use anyhow::anyhow;

    struct MockApi {
        project: Option<Project>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn create_project(&self, _params: &[(String, String)]) -> anyhow::Result<Project> {
            self.project.clone().ok_or_else(|| anyhow!("create failed"))
        }
    }

    fn args(json: bool) -> ProjectCreateArgs {
        ProjectCreateArgs::new(
            "Test Project".to_string(),
            "TEST".to_string(),
            false,
            false,
            "markdown".to_string(),
            json,
        )
    }

    #[test]
    fn create_with_text_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(create_with(&args(false), &api).is_ok());
    }

    #[test]
    fn create_with_json_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(create_with(&args(true), &api).is_ok());
    }

    #[test]
    fn create_with_propagates_api_error() {
        let api = MockApi { project: None };
        let err = create_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("create failed"));
    }
}
