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

fn format_status_row(s: &ProjectStatus) -> String {
    format!("[{}] {}", s.id, s.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        statuses: Option<Vec<ProjectStatus>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_statuses(&self, _key: &str) -> anyhow::Result<Vec<ProjectStatus>> {
            self.statuses.clone().ok_or_else(|| anyhow!("no statuses"))
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

    #[test]
    fn format_status_row_contains_fields() {
        let text = format_status_row(&sample_status());
        assert!(text.contains("[1]"));
        assert!(text.contains("Open"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            statuses: Some(vec![sample_status()]),
        };
        assert!(list_with(&ProjectStatusListArgs::new("TEST".to_string(), false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            statuses: Some(vec![sample_status()]),
        };
        assert!(list_with(&ProjectStatusListArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { statuses: None };
        let err =
            list_with(&ProjectStatusListArgs::new("TEST".to_string(), false), &api).unwrap_err();
        assert!(err.to_string().contains("no statuses"));
    }
}
