use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient, project::Project};

pub struct ProjectShowArgs {
    key: String,
    json: bool,
}

impl ProjectShowArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn show(args: &ProjectShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &ProjectShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let project = api.get_project(&args.key)?;
    if args.json {
        crate::cmd::print_json(&project)?;
    } else {
        println!("{}", format_project_detail(&project));
    }
    Ok(())
}

fn format_project_detail(p: &Project) -> String {
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
        fn get_project(&self, _key: &str) -> anyhow::Result<Project> {
            self.project.clone().ok_or_else(|| anyhow!("no project"))
        }
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(show_with(&ProjectShowArgs::new("TEST".to_string(), false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(show_with(&ProjectShowArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { project: None };
        let err = show_with(&ProjectShowArgs::new("MISSING".to_string(), false), &api).unwrap_err();
        assert!(err.to_string().contains("no project"));
    }

    #[test]
    fn format_project_detail_contains_fields() {
        let text = format_project_detail(&sample_project());
        assert!(text.contains("TEST"));
        assert!(text.contains("Test Project"));
        assert!(text.contains("markdown"));
        assert!(text.contains("false"));
    }
}
