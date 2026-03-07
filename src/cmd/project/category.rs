use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectCategory};

pub fn list(key: &str, json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(key, json, &client)
}

pub fn list_with(key: &str, json: bool, api: &dyn BacklogApi) -> Result<()> {
    let categories = api.get_project_categories(key)?;
    if json {
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
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(&self) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_space_disk_usage(&self) -> anyhow::Result<crate::api::disk_usage::DiskUsage> {
            unimplemented!()
        }
        fn get_space_notification(
            &self,
        ) -> anyhow::Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
        }
        fn get_projects(&self) -> anyhow::Result<Vec<crate::api::project::Project>> {
            unimplemented!()
        }
        fn get_project(&self, _key: &str) -> anyhow::Result<crate::api::project::Project> {
            unimplemented!()
        }
        fn get_project_activities(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(
            &self,
            _key: &str,
        ) -> anyhow::Result<crate::api::project::ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(&self, _key: &str) -> anyhow::Result<Vec<ProjectCategory>> {
            self.categories
                .clone()
                .ok_or_else(|| anyhow!("no categories"))
        }
        fn get_project_versions(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectVersion>> {
            unimplemented!()
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
        assert!(list_with("TEST", false, &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            categories: Some(vec![sample_category()]),
        };
        assert!(list_with("TEST", true, &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { categories: None };
        let err = list_with("TEST", false, &api).unwrap_err();
        assert!(err.to_string().contains("no categories"));
    }
}
