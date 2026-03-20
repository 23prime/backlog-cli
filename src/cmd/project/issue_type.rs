use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectIssueType};

pub struct ProjectIssueTypeListArgs {
    key: String,
    json: bool,
}

impl ProjectIssueTypeListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectIssueTypeAddArgs {
    key: String,
    name: String,
    color: String,
    json: bool,
}

impl ProjectIssueTypeAddArgs {
    pub fn try_new(key: String, name: String, color: String, json: bool) -> anyhow::Result<Self> {
        validate_color(&color)?;
        Ok(Self {
            key,
            name,
            color,
            json,
        })
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectIssueTypeUpdateArgs {
    key: String,
    issue_type_id: u64,
    name: Option<String>,
    color: Option<String>,
    json: bool,
}

impl ProjectIssueTypeUpdateArgs {
    pub fn try_new(
        key: String,
        issue_type_id: u64,
        name: Option<String>,
        color: Option<String>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if name.is_none() && color.is_none() {
            return Err(anyhow::anyhow!(
                "At least one of --name or --color must be specified"
            ));
        }
        if let Some(c) = &color {
            validate_color(c)?;
        }
        Ok(Self {
            key,
            issue_type_id,
            name,
            color,
            json,
        })
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectIssueTypeDeleteArgs {
    key: String,
    issue_type_id: u64,
    substitute_issue_type_id: u64,
    json: bool,
}

impl ProjectIssueTypeDeleteArgs {
    pub fn try_new(
        key: String,
        issue_type_id: u64,
        substitute_issue_type_id: u64,
        json: bool,
    ) -> anyhow::Result<Self> {
        if issue_type_id == substitute_issue_type_id {
            return Err(anyhow::anyhow!(
                "--substitute-issue-type-id must differ from --issue-type-id"
            ));
        }
        Ok(Self {
            key,
            issue_type_id,
            substitute_issue_type_id,
            json,
        })
    }
}

pub fn list(args: &ProjectIssueTypeListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectIssueTypeListArgs, api: &dyn BacklogApi) -> Result<()> {
    let issue_types = api.get_project_issue_types(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issue_types).context("Failed to serialize JSON")?
        );
    } else {
        for t in &issue_types {
            println!("{}", format_issue_type_row(t));
        }
    }
    Ok(())
}

pub fn add(args: &ProjectIssueTypeAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectIssueTypeAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let issue_type = api.add_project_issue_type(&args.key, &args.name, &args.color)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issue_type).context("Failed to serialize JSON")?
        );
    } else {
        println!("Added: {}", format_issue_type_row(&issue_type));
    }
    Ok(())
}

pub fn update(args: &ProjectIssueTypeUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &ProjectIssueTypeUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(name) = &args.name {
        params.push(("name".to_string(), name.clone()));
    }
    if let Some(color) = &args.color {
        params.push(("color".to_string(), color.clone()));
    }
    let issue_type = api.update_project_issue_type(&args.key, args.issue_type_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issue_type).context("Failed to serialize JSON")?
        );
    } else {
        println!("Updated: {}", format_issue_type_row(&issue_type));
    }
    Ok(())
}

pub fn delete(args: &ProjectIssueTypeDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectIssueTypeDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let issue_type = api.delete_project_issue_type(
        &args.key,
        args.issue_type_id,
        args.substitute_issue_type_id,
    )?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&issue_type).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", format_issue_type_row(&issue_type));
    }
    Ok(())
}

fn validate_color(color: &str) -> anyhow::Result<()> {
    if color.len() == 7
        && color.starts_with('#')
        && color[1..].chars().all(|c| c.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Color must be a 6-digit hex code with # prefix (e.g. #e30000)"
        ))
    }
}

fn format_issue_type_row(t: &ProjectIssueType) -> String {
    format!("[{}] {}", t.id, t.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        list: Option<Vec<ProjectIssueType>>,
        single: Option<ProjectIssueType>,
    }

    fn mock(list: Option<Vec<ProjectIssueType>>, single: Option<ProjectIssueType>) -> MockApi {
        MockApi { list, single }
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_issue_types(&self, _key: &str) -> anyhow::Result<Vec<ProjectIssueType>> {
            self.list.clone().ok_or_else(|| anyhow!("list failed"))
        }

        fn add_project_issue_type(
            &self,
            _key: &str,
            _name: &str,
            _color: &str,
        ) -> anyhow::Result<ProjectIssueType> {
            self.single.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn update_project_issue_type(
            &self,
            _key: &str,
            _issue_type_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<ProjectIssueType> {
            self.single.clone().ok_or_else(|| anyhow!("update failed"))
        }

        fn delete_project_issue_type(
            &self,
            _key: &str,
            _issue_type_id: u64,
            _substitute_issue_type_id: u64,
        ) -> anyhow::Result<ProjectIssueType> {
            self.single.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_issue_type() -> ProjectIssueType {
        ProjectIssueType {
            id: 1,
            project_id: 10,
            name: "Bug".to_string(),
            color: "#e30000".to_string(),
            display_order: 0,
            template_summary: None,
            template_description: None,
        }
    }

    #[test]
    fn format_issue_type_row_contains_fields() {
        let text = format_issue_type_row(&sample_issue_type());
        assert!(text.contains("[1]"));
        assert!(text.contains("Bug"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = mock(Some(vec![sample_issue_type()]), None);
        assert!(
            list_with(
                &ProjectIssueTypeListArgs::new("TEST".to_string(), false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = mock(Some(vec![sample_issue_type()]), None);
        assert!(
            list_with(
                &ProjectIssueTypeListArgs::new("TEST".to_string(), true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = mock(None, None);
        let err = list_with(
            &ProjectIssueTypeListArgs::new("TEST".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("list failed"));
    }

    #[test]
    fn add_try_new_rejects_invalid_color() {
        let err = ProjectIssueTypeAddArgs::try_new(
            "TEST".to_string(),
            "Bug".to_string(),
            "e30000".to_string(),
            false,
        )
        .unwrap_err();
        assert!(err.to_string().contains("hex code"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = mock(None, Some(sample_issue_type()));
        let args = ProjectIssueTypeAddArgs::try_new(
            "TEST".to_string(),
            "Bug".to_string(),
            "#e30000".to_string(),
            false,
        )
        .unwrap();
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = mock(None, Some(sample_issue_type()));
        let args = ProjectIssueTypeAddArgs::try_new(
            "TEST".to_string(),
            "Bug".to_string(),
            "#e30000".to_string(),
            true,
        )
        .unwrap();
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectIssueTypeAddArgs::try_new(
            "TEST".to_string(),
            "Bug".to_string(),
            "#e30000".to_string(),
            false,
        )
        .unwrap();
        let err = add_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn update_try_new_rejects_no_fields() {
        let err = ProjectIssueTypeUpdateArgs::try_new("TEST".to_string(), 1, None, None, false)
            .unwrap_err();
        assert!(err.to_string().contains("At least one"));
    }

    #[test]
    fn update_try_new_rejects_invalid_color() {
        let err = ProjectIssueTypeUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            None,
            Some("e30000".to_string()),
            false,
        )
        .unwrap_err();
        assert!(err.to_string().contains("hex code"));
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = mock(None, Some(sample_issue_type()));
        let args = ProjectIssueTypeUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("Bugfix".to_string()),
            None,
            false,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = mock(None, Some(sample_issue_type()));
        let args = ProjectIssueTypeUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("Bugfix".to_string()),
            None,
            true,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectIssueTypeUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("Bugfix".to_string()),
            None,
            false,
        )
        .unwrap();
        let err = update_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn delete_try_new_rejects_same_ids() {
        let err = ProjectIssueTypeDeleteArgs::try_new("TEST".to_string(), 1, 1, false).unwrap_err();
        assert!(err.to_string().contains("must differ"));
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = mock(None, Some(sample_issue_type()));
        let args = ProjectIssueTypeDeleteArgs::try_new("TEST".to_string(), 1, 2, false).unwrap();
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = mock(None, Some(sample_issue_type()));
        let args = ProjectIssueTypeDeleteArgs::try_new("TEST".to_string(), 1, 2, true).unwrap();
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectIssueTypeDeleteArgs::try_new("TEST".to_string(), 1, 2, false).unwrap();
        let err = delete_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
