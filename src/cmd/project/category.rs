use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectCategory};

pub struct ProjectCategoryListArgs {
    key: String,
    json: bool,
}

impl ProjectCategoryListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &ProjectCategoryListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectCategoryListArgs, api: &dyn BacklogApi) -> Result<()> {
    let categories = api.get_project_categories(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&categories).context("Failed to serialize JSON")?
        );
    } else {
        for c in &categories {
            println!("{}", format_category_row(c));
        }
    }
    Ok(())
}

fn format_category_row(c: &ProjectCategory) -> String {
    format!("[{}] {}", c.id, c.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        categories: Option<Vec<ProjectCategory>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_categories(&self, _key: &str) -> anyhow::Result<Vec<ProjectCategory>> {
            self.categories
                .clone()
                .ok_or_else(|| anyhow!("no categories"))
        }
    }

    fn sample_category() -> ProjectCategory {
        ProjectCategory {
            id: 11,
            name: "Development".to_string(),
            display_order: 0,
        }
    }

    #[test]
    fn format_category_row_contains_fields() {
        let text = format_category_row(&sample_category());
        assert!(text.contains("[11]"));
        assert!(text.contains("Development"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            categories: Some(vec![sample_category()]),
        };
        assert!(
            list_with(
                &ProjectCategoryListArgs::new("TEST".to_string(), false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            categories: Some(vec![sample_category()]),
        };
        assert!(
            list_with(
                &ProjectCategoryListArgs::new("TEST".to_string(), true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { categories: None };
        let err = list_with(
            &ProjectCategoryListArgs::new("TEST".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no categories"));
    }
}
