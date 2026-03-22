use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

#[cfg_attr(test, derive(Debug))]
pub struct ProjectUpdateArgs {
    key: String,
    name: Option<String>,
    new_key: Option<String>,
    chart_enabled: Option<bool>,
    subtasking_enabled: Option<bool>,
    text_formatting_rule: Option<String>,
    archived: Option<bool>,
    json: bool,
}

impl ProjectUpdateArgs {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        key: String,
        name: Option<String>,
        new_key: Option<String>,
        chart_enabled: Option<bool>,
        subtasking_enabled: Option<bool>,
        text_formatting_rule: Option<String>,
        archived: Option<bool>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if name.is_none()
            && new_key.is_none()
            && chart_enabled.is_none()
            && subtasking_enabled.is_none()
            && text_formatting_rule.is_none()
            && archived.is_none()
        {
            return Err(anyhow::anyhow!(
                "At least one field must be specified for update"
            ));
        }
        Ok(Self {
            key,
            name,
            new_key,
            chart_enabled,
            subtasking_enabled,
            text_formatting_rule,
            archived,
            json,
        })
    }
}

pub fn update(args: &ProjectUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &ProjectUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(n) = &args.name {
        params.push(("name".to_string(), n.clone()));
    }
    if let Some(k) = &args.new_key {
        params.push(("key".to_string(), k.clone()));
    }
    if let Some(v) = args.chart_enabled {
        params.push(("chartEnabled".to_string(), v.to_string()));
    }
    if let Some(v) = args.subtasking_enabled {
        params.push(("subtaskingEnabled".to_string(), v.to_string()));
    }
    if let Some(r) = &args.text_formatting_rule {
        params.push(("textFormattingRule".to_string(), r.clone()));
    }
    if let Some(v) = args.archived {
        params.push(("archived".to_string(), v.to_string()));
    }

    let project = api.update_project(&args.key, &params)?;
    if args.json {
        crate::cmd::print_json(&project)?;
    } else {
        println!(
            "ID:         {}\nKey:        {}\nName:       {}\nFormatting: {}\nArchived:   {}",
            project.id,
            project.project_key,
            project.name,
            project.text_formatting_rule,
            project.archived,
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::project::Project;
    use crate::cmd::project::sample_project;
    use anyhow::anyhow;

    struct MockApi {
        project: Option<Project>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn update_project(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<Project> {
            self.project.clone().ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn args(json: bool) -> ProjectUpdateArgs {
        ProjectUpdateArgs::try_new(
            "TEST".to_string(),
            Some("New Name".to_string()),
            None,
            None,
            None,
            None,
            None,
            json,
        )
        .unwrap()
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            project: Some(sample_project()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_rejects_empty_params() {
        let err = ProjectUpdateArgs::try_new(
            "TEST".to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap_err();
        assert!(err.to_string().contains("At least one field"));
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { project: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }
}
