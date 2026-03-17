use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectStatus};

pub struct ProjectStatusListArgs {
    key: String,
    json: bool,
}

impl ProjectStatusListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &ProjectStatusListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectStatusListArgs, api: &dyn BacklogApi) -> Result<()> {
    let statuses = api.get_project_statuses(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&statuses).context("Failed to serialize JSON")?
        );
    } else {
        for s in &statuses {
            println!("{}", format_status_row(s));
        }
    }
    Ok(())
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectStatusAddArgs {
    key: String,
    name: String,
    color: String,
    json: bool,
}

impl ProjectStatusAddArgs {
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

pub fn add(args: &ProjectStatusAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectStatusAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let status = api.add_project_status(&args.key, &args.name, &args.color)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&status).context("Failed to serialize JSON")?
        );
    } else {
        println!("Added: {}", format_status_row(&status));
    }
    Ok(())
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectStatusUpdateArgs {
    key: String,
    status_id: u64,
    name: Option<String>,
    color: Option<String>,
    json: bool,
}

impl ProjectStatusUpdateArgs {
    pub fn try_new(
        key: String,
        status_id: u64,
        name: Option<String>,
        color: Option<String>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if name.is_none() && color.is_none() {
            return Err(anyhow::anyhow!(
                "At least one of --name or --color must be specified for update"
            ));
        }
        if let Some(c) = &color {
            validate_color(c)?;
        }
        Ok(Self {
            key,
            status_id,
            name,
            color,
            json,
        })
    }
}

pub fn update(args: &ProjectStatusUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &ProjectStatusUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(name) = &args.name {
        params.push(("name".to_string(), name.clone()));
    }
    if let Some(color) = &args.color {
        params.push(("color".to_string(), color.clone()));
    }
    let status = api.update_project_status(&args.key, args.status_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&status).context("Failed to serialize JSON")?
        );
    } else {
        println!("Updated: {}", format_status_row(&status));
    }
    Ok(())
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectStatusDeleteArgs {
    key: String,
    status_id: u64,
    substitute_status_id: u64,
    json: bool,
}

impl ProjectStatusDeleteArgs {
    pub fn try_new(
        key: String,
        status_id: u64,
        substitute_status_id: u64,
        json: bool,
    ) -> anyhow::Result<Self> {
        if status_id == substitute_status_id {
            return Err(anyhow::anyhow!(
                "--substitute-status-id must differ from --status-id"
            ));
        }
        Ok(Self {
            key,
            status_id,
            substitute_status_id,
            json,
        })
    }
}

pub fn delete(args: &ProjectStatusDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectStatusDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let status = api.delete_project_status(&args.key, args.status_id, args.substitute_status_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&status).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", format_status_row(&status));
    }
    Ok(())
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectStatusReorderArgs {
    key: String,
    status_ids: Vec<u64>,
    json: bool,
}

impl ProjectStatusReorderArgs {
    pub fn try_new(key: String, status_ids: Vec<u64>, json: bool) -> anyhow::Result<Self> {
        if status_ids.is_empty() {
            return Err(anyhow::anyhow!(
                "At least one --status-id must be specified for reorder"
            ));
        }
        let unique_count = status_ids
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>()
            .len();
        if unique_count != status_ids.len() {
            return Err(anyhow::anyhow!(
                "--status-id values for reorder must be unique"
            ));
        }
        Ok(Self {
            key,
            status_ids,
            json,
        })
    }
}

pub fn reorder(args: &ProjectStatusReorderArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    reorder_with(args, &client)
}

pub fn reorder_with(args: &ProjectStatusReorderArgs, api: &dyn BacklogApi) -> Result<()> {
    let statuses = api.reorder_project_statuses(&args.key, &args.status_ids)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&statuses).context("Failed to serialize JSON")?
        );
    } else {
        for s in &statuses {
            println!("{}", format_status_row(s));
        }
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
            "Color must be a 6-digit hex code with # prefix (e.g. #ed8077)"
        ))
    }
}

fn format_status_row(s: &ProjectStatus) -> String {
    format!("[{}] {}", s.id, s.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        statuses: Option<Vec<ProjectStatus>>,
        status: Option<ProjectStatus>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_statuses(&self, _key: &str) -> anyhow::Result<Vec<ProjectStatus>> {
            self.statuses.clone().ok_or_else(|| anyhow!("no statuses"))
        }

        fn add_project_status(
            &self,
            _key: &str,
            _name: &str,
            _color: &str,
        ) -> anyhow::Result<ProjectStatus> {
            self.status.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn update_project_status(
            &self,
            _key: &str,
            _status_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<ProjectStatus> {
            self.status.clone().ok_or_else(|| anyhow!("update failed"))
        }

        fn delete_project_status(
            &self,
            _key: &str,
            _status_id: u64,
            _substitute_status_id: u64,
        ) -> anyhow::Result<ProjectStatus> {
            self.status.clone().ok_or_else(|| anyhow!("delete failed"))
        }

        fn reorder_project_statuses(
            &self,
            _key: &str,
            _status_ids: &[u64],
        ) -> anyhow::Result<Vec<ProjectStatus>> {
            self.statuses
                .clone()
                .ok_or_else(|| anyhow!("reorder failed"))
        }
    }

    fn sample_status() -> ProjectStatus {
        ProjectStatus {
            id: 1,
            project_id: 10,
            name: "Open".to_string(),
            color: "#ed8077".to_string(),
            display_order: 1000,
        }
    }

    fn mock(statuses: Option<Vec<ProjectStatus>>, status: Option<ProjectStatus>) -> MockApi {
        MockApi { statuses, status }
    }

    #[test]
    fn format_status_row_contains_fields() {
        let text = format_status_row(&sample_status());
        assert!(text.contains("[1]"));
        assert!(text.contains("Open"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = mock(Some(vec![sample_status()]), None);
        assert!(list_with(&ProjectStatusListArgs::new("TEST".to_string(), false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = mock(Some(vec![sample_status()]), None);
        assert!(list_with(&ProjectStatusListArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = mock(None, None);
        let err =
            list_with(&ProjectStatusListArgs::new("TEST".to_string(), false), &api).unwrap_err();
        assert!(err.to_string().contains("no statuses"));
    }

    #[test]
    fn add_try_new_rejects_invalid_color() {
        let err = ProjectStatusAddArgs::try_new(
            "TEST".to_string(),
            "Open".to_string(),
            "ed8077".to_string(),
            false,
        )
        .unwrap_err();
        assert!(err.to_string().contains("hex code"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = mock(None, Some(sample_status()));
        let args = ProjectStatusAddArgs::try_new(
            "TEST".to_string(),
            "Open".to_string(),
            "#ed8077".to_string(),
            false,
        )
        .unwrap();
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = mock(None, Some(sample_status()));
        let args = ProjectStatusAddArgs::try_new(
            "TEST".to_string(),
            "Open".to_string(),
            "#ed8077".to_string(),
            true,
        )
        .unwrap();
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectStatusAddArgs::try_new(
            "TEST".to_string(),
            "Open".to_string(),
            "#ed8077".to_string(),
            false,
        )
        .unwrap();
        let err = add_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn update_try_new_rejects_empty() {
        let err =
            ProjectStatusUpdateArgs::try_new("TEST".to_string(), 1, None, None, false).unwrap_err();
        assert!(err.to_string().contains("At least one"));
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = mock(None, Some(sample_status()));
        let args = ProjectStatusUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("Closed".to_string()),
            None,
            false,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = mock(None, Some(sample_status()));
        let args = ProjectStatusUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            None,
            Some("#f42858".to_string()),
            true,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectStatusUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("Closed".to_string()),
            None,
            false,
        )
        .unwrap();
        let err = update_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn delete_try_new_rejects_same_ids() {
        let err = ProjectStatusDeleteArgs::try_new("TEST".to_string(), 1, 1, false).unwrap_err();
        assert!(err.to_string().contains("must differ"));
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = mock(None, Some(sample_status()));
        let args = ProjectStatusDeleteArgs::try_new("TEST".to_string(), 1, 2, false).unwrap();
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = mock(None, Some(sample_status()));
        let args = ProjectStatusDeleteArgs::try_new("TEST".to_string(), 1, 2, true).unwrap();
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectStatusDeleteArgs::try_new("TEST".to_string(), 1, 2, false).unwrap();
        let err = delete_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }

    #[test]
    fn reorder_try_new_rejects_empty() {
        let err = ProjectStatusReorderArgs::try_new("TEST".to_string(), vec![], false).unwrap_err();
        assert!(err.to_string().contains("At least one"));
    }

    #[test]
    fn reorder_try_new_rejects_duplicate_ids() {
        let err =
            ProjectStatusReorderArgs::try_new("TEST".to_string(), vec![1, 1], false).unwrap_err();
        assert!(err.to_string().contains("unique"));
    }

    #[test]
    fn reorder_with_text_output_succeeds() {
        let api = mock(Some(vec![sample_status()]), None);
        let args =
            ProjectStatusReorderArgs::try_new("TEST".to_string(), vec![1, 2], false).unwrap();
        assert!(reorder_with(&args, &api).is_ok());
    }

    #[test]
    fn reorder_with_json_output_succeeds() {
        let api = mock(Some(vec![sample_status()]), None);
        let args = ProjectStatusReorderArgs::try_new("TEST".to_string(), vec![1, 2], true).unwrap();
        assert!(reorder_with(&args, &api).is_ok());
    }

    #[test]
    fn reorder_with_propagates_api_error() {
        let api = mock(None, None);
        let args =
            ProjectStatusReorderArgs::try_new("TEST".to_string(), vec![1, 2], false).unwrap();
        let err = reorder_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("reorder failed"));
    }
}
