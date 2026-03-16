use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct UserRecentlyViewedProjectsArgs {
    json: bool,
    pub count: u32,
    pub offset: u64,
    pub order: Option<String>,
}

impl UserRecentlyViewedProjectsArgs {
    pub fn try_new(
        json: bool,
        count: u32,
        offset: u64,
        order: Option<String>,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        Ok(Self {
            json,
            count,
            offset,
            order,
        })
    }
}

pub fn recently_viewed_projects(args: &UserRecentlyViewedProjectsArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    recently_viewed_projects_with(args, &client)
}

pub fn recently_viewed_projects_with(
    args: &UserRecentlyViewedProjectsArgs,
    api: &dyn BacklogApi,
) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("count".to_string(), args.count.to_string()));
    params.push(("offset".to_string(), args.offset.to_string()));
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    let items = api.get_recently_viewed_projects(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&items).context("Failed to serialize JSON")?
        );
    } else {
        for item in &items {
            println!("[{}] {}", item.project.project_key, item.project.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::project::Project;
    use crate::api::user::RecentlyViewedProject;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        items: Option<Vec<RecentlyViewedProject>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_recently_viewed_projects(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<RecentlyViewedProject>> {
            self.items.clone().ok_or_else(|| anyhow!("no items"))
        }
    }

    fn sample_item() -> RecentlyViewedProject {
        RecentlyViewedProject {
            project: Project {
                id: 1,
                project_key: "TEST".to_string(),
                name: "Test Project".to_string(),
                chart_enabled: false,
                subtasking_enabled: false,
                project_leader_can_edit_project_leader: false,
                text_formatting_rule: "markdown".to_string(),
                archived: false,
                extra: BTreeMap::new(),
            },
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> UserRecentlyViewedProjectsArgs {
        UserRecentlyViewedProjectsArgs::try_new(json, 20, 0, None).unwrap()
    }

    #[test]
    fn recently_viewed_projects_with_text_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(recently_viewed_projects_with(&args(false), &api).is_ok());
    }

    #[test]
    fn recently_viewed_projects_with_json_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(recently_viewed_projects_with(&args(true), &api).is_ok());
    }

    #[test]
    fn recently_viewed_projects_with_propagates_api_error() {
        let api = MockApi { items: None };
        let err = recently_viewed_projects_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no items"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(UserRecentlyViewedProjectsArgs::try_new(false, 101, 0, None).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(UserRecentlyViewedProjectsArgs::try_new(false, 0, 0, None).is_err());
    }
}
