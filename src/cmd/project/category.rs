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

pub struct ProjectCategoryAddArgs {
    key: String,
    name: String,
    json: bool,
}

impl ProjectCategoryAddArgs {
    pub fn new(key: String, name: String, json: bool) -> Self {
        Self { key, name, json }
    }
}

pub struct ProjectCategoryUpdateArgs {
    key: String,
    category_id: u64,
    name: String,
    json: bool,
}

impl ProjectCategoryUpdateArgs {
    pub fn new(key: String, category_id: u64, name: String, json: bool) -> Self {
        Self {
            key,
            category_id,
            name,
            json,
        }
    }
}

pub struct ProjectCategoryDeleteArgs {
    key: String,
    category_id: u64,
    json: bool,
}

impl ProjectCategoryDeleteArgs {
    pub fn new(key: String, category_id: u64, json: bool) -> Self {
        Self {
            key,
            category_id,
            json,
        }
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

pub fn add(args: &ProjectCategoryAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectCategoryAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let category = api.add_project_category(&args.key, &args.name)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&category).context("Failed to serialize JSON")?
        );
    } else {
        println!("Added: {}", format_category_row(&category));
    }
    Ok(())
}

pub fn update(args: &ProjectCategoryUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &ProjectCategoryUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let category = api.update_project_category(&args.key, args.category_id, &args.name)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&category).context("Failed to serialize JSON")?
        );
    } else {
        println!("Updated: {}", format_category_row(&category));
    }
    Ok(())
}

pub fn delete(args: &ProjectCategoryDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectCategoryDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let category = api.delete_project_category(&args.key, args.category_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&category).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", format_category_row(&category));
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
        list: Option<Vec<ProjectCategory>>,
        single: Option<ProjectCategory>,
    }

    fn mock(list: Option<Vec<ProjectCategory>>, single: Option<ProjectCategory>) -> MockApi {
        MockApi { list, single }
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_categories(&self, _key: &str) -> anyhow::Result<Vec<ProjectCategory>> {
            self.list.clone().ok_or_else(|| anyhow!("list failed"))
        }

        fn add_project_category(&self, _key: &str, _name: &str) -> anyhow::Result<ProjectCategory> {
            self.single.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn update_project_category(
            &self,
            _key: &str,
            _category_id: u64,
            _name: &str,
        ) -> anyhow::Result<ProjectCategory> {
            self.single.clone().ok_or_else(|| anyhow!("update failed"))
        }

        fn delete_project_category(
            &self,
            _key: &str,
            _category_id: u64,
        ) -> anyhow::Result<ProjectCategory> {
            self.single.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_category() -> ProjectCategory {
        ProjectCategory {
            id: 11,
            project_id: 1,
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
        let api = mock(Some(vec![sample_category()]), None);
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
        let api = mock(Some(vec![sample_category()]), None);
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
        let api = mock(None, None);
        let err = list_with(
            &ProjectCategoryListArgs::new("TEST".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("list failed"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = mock(None, Some(sample_category()));
        let args = ProjectCategoryAddArgs::new("TEST".to_string(), "Dev".to_string(), false);
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = mock(None, Some(sample_category()));
        let args = ProjectCategoryAddArgs::new("TEST".to_string(), "Dev".to_string(), true);
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectCategoryAddArgs::new("TEST".to_string(), "Dev".to_string(), false);
        let err = add_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = mock(None, Some(sample_category()));
        let args =
            ProjectCategoryUpdateArgs::new("TEST".to_string(), 11, "Dev2".to_string(), false);
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = mock(None, Some(sample_category()));
        let args = ProjectCategoryUpdateArgs::new("TEST".to_string(), 11, "Dev2".to_string(), true);
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = mock(None, None);
        let args =
            ProjectCategoryUpdateArgs::new("TEST".to_string(), 11, "Dev2".to_string(), false);
        let err = update_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = mock(None, Some(sample_category()));
        let args = ProjectCategoryDeleteArgs::new("TEST".to_string(), 11, false);
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = mock(None, Some(sample_category()));
        let args = ProjectCategoryDeleteArgs::new("TEST".to_string(), 11, true);
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectCategoryDeleteArgs::new("TEST".to_string(), 11, false);
        let err = delete_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
