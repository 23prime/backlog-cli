use anstream::println;
use anyhow::Result;

use crate::api::{
    BacklogApi, BacklogClient,
    project::{ProjectVersion, UpdateProjectVersionParams},
};

pub struct ProjectVersionListArgs {
    key: String,
    json: bool,
}

impl ProjectVersionListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub struct ProjectVersionAddArgs {
    key: String,
    name: String,
    description: Option<String>,
    start_date: Option<String>,
    release_due_date: Option<String>,
    json: bool,
}

impl ProjectVersionAddArgs {
    pub fn new(
        key: String,
        name: String,
        description: Option<String>,
        start_date: Option<String>,
        release_due_date: Option<String>,
        json: bool,
    ) -> Self {
        Self {
            key,
            name,
            description,
            start_date,
            release_due_date,
            json,
        }
    }
}

pub struct ProjectVersionUpdateArgs {
    key: String,
    version_id: u64,
    name: String,
    description: Option<String>,
    start_date: Option<String>,
    release_due_date: Option<String>,
    archived: Option<bool>,
    json: bool,
}

impl ProjectVersionUpdateArgs {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        key: String,
        version_id: u64,
        name: String,
        description: Option<String>,
        start_date: Option<String>,
        release_due_date: Option<String>,
        archived: Option<bool>,
        json: bool,
    ) -> Self {
        Self {
            key,
            version_id,
            name,
            description,
            start_date,
            release_due_date,
            archived,
            json,
        }
    }
}

pub struct ProjectVersionDeleteArgs {
    key: String,
    version_id: u64,
    json: bool,
}

impl ProjectVersionDeleteArgs {
    pub fn new(key: String, version_id: u64, json: bool) -> Self {
        Self {
            key,
            version_id,
            json,
        }
    }
}

pub fn list(args: &ProjectVersionListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectVersionListArgs, api: &dyn BacklogApi) -> Result<()> {
    let versions = api.get_project_versions(&args.key)?;
    if args.json {
        crate::cmd::print_json(&versions)?;
    } else {
        for v in &versions {
            println!("{}", format_version_row(v));
        }
    }
    Ok(())
}

pub fn add(args: &ProjectVersionAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectVersionAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let version = api.add_project_version(
        &args.key,
        &args.name,
        args.description.as_deref(),
        args.start_date.as_deref(),
        args.release_due_date.as_deref(),
    )?;
    if args.json {
        crate::cmd::print_json(&version)?;
    } else {
        println!("Added: {}", format_version_row(&version));
    }
    Ok(())
}

pub fn update(args: &ProjectVersionUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &ProjectVersionUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = UpdateProjectVersionParams {
        name: &args.name,
        description: args.description.as_deref(),
        start_date: args.start_date.as_deref(),
        release_due_date: args.release_due_date.as_deref(),
        archived: args.archived,
    };
    let version = api.update_project_version(&args.key, args.version_id, &params)?;
    if args.json {
        crate::cmd::print_json(&version)?;
    } else {
        println!("Updated: {}", format_version_row(&version));
    }
    Ok(())
}

pub fn delete(args: &ProjectVersionDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectVersionDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let version = api.delete_project_version(&args.key, args.version_id)?;
    if args.json {
        crate::cmd::print_json(&version)?;
    } else {
        println!("Deleted: {}", format_version_row(&version));
    }
    Ok(())
}

fn format_version_row(v: &ProjectVersion) -> String {
    let dates = match (&v.start_date, &v.release_due_date) {
        (Some(s), Some(e)) => format!(" ({} → {})", s, e),
        (Some(s), None) => format!(" (from {})", s),
        (None, Some(e)) => format!(" (until {})", e),
        (None, None) => String::new(),
    };
    let archived = if v.archived { " [archived]" } else { "" };
    format!("[{}] {}{}{}", v.id, v.name, dates, archived)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        list: Option<Vec<ProjectVersion>>,
        single: Option<ProjectVersion>,
    }

    fn mock(list: Option<Vec<ProjectVersion>>, single: Option<ProjectVersion>) -> MockApi {
        MockApi { list, single }
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_versions(&self, _key: &str) -> anyhow::Result<Vec<ProjectVersion>> {
            self.list.clone().ok_or_else(|| anyhow!("list failed"))
        }

        fn add_project_version(
            &self,
            _key: &str,
            _name: &str,
            _description: Option<&str>,
            _start_date: Option<&str>,
            _release_due_date: Option<&str>,
        ) -> anyhow::Result<ProjectVersion> {
            self.single.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn update_project_version(
            &self,
            _key: &str,
            _version_id: u64,
            _params: &crate::api::project::UpdateProjectVersionParams<'_>,
        ) -> anyhow::Result<ProjectVersion> {
            self.single.clone().ok_or_else(|| anyhow!("update failed"))
        }

        fn delete_project_version(
            &self,
            _key: &str,
            _version_id: u64,
        ) -> anyhow::Result<ProjectVersion> {
            self.single.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_version() -> ProjectVersion {
        ProjectVersion {
            id: 3,
            project_id: 1,
            name: "Version 0.1".to_string(),
            description: None,
            start_date: Some("2024-01-01T00:00:00Z".to_string()),
            release_due_date: Some("2024-01-31T00:00:00Z".to_string()),
            archived: false,
            display_order: 0,
        }
    }

    #[test]
    fn format_version_row_with_dates() {
        let text = format_version_row(&sample_version());
        assert!(text.contains("[3]"));
        assert!(text.contains("Version 0.1"));
        assert!(text.contains("2024-01-01T00:00:00Z"));
        assert!(text.contains("2024-01-31T00:00:00Z"));
        assert!(!text.contains("[archived]"));
    }

    #[test]
    fn format_version_row_archived() {
        let v = ProjectVersion {
            archived: true,
            ..sample_version()
        };
        let text = format_version_row(&v);
        assert!(text.contains("[archived]"));
    }

    #[test]
    fn format_version_row_no_dates() {
        let v = ProjectVersion {
            start_date: None,
            release_due_date: None,
            ..sample_version()
        };
        let text = format_version_row(&v);
        assert!(text.contains("[3] Version 0.1"));
        assert!(!text.contains("→"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = mock(Some(vec![sample_version()]), None);
        assert!(
            list_with(
                &ProjectVersionListArgs::new("TEST".to_string(), false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = mock(Some(vec![sample_version()]), None);
        assert!(list_with(&ProjectVersionListArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = mock(None, None);
        let err = list_with(
            &ProjectVersionListArgs::new("TEST".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("list failed"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = mock(None, Some(sample_version()));
        let args = ProjectVersionAddArgs::new(
            "TEST".to_string(),
            "v1.0".to_string(),
            None,
            None,
            None,
            false,
        );
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = mock(None, Some(sample_version()));
        let args = ProjectVersionAddArgs::new(
            "TEST".to_string(),
            "v1.0".to_string(),
            None,
            None,
            None,
            true,
        );
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectVersionAddArgs::new(
            "TEST".to_string(),
            "v1.0".to_string(),
            None,
            None,
            None,
            false,
        );
        let err = add_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = mock(None, Some(sample_version()));
        let args = ProjectVersionUpdateArgs::new(
            "TEST".to_string(),
            3,
            "v1.1".to_string(),
            None,
            None,
            None,
            None,
            false,
        );
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = mock(None, Some(sample_version()));
        let args = ProjectVersionUpdateArgs::new(
            "TEST".to_string(),
            3,
            "v1.1".to_string(),
            None,
            None,
            None,
            Some(true),
            true,
        );
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectVersionUpdateArgs::new(
            "TEST".to_string(),
            3,
            "v1.1".to_string(),
            None,
            None,
            None,
            None,
            false,
        );
        let err = update_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = mock(None, Some(sample_version()));
        let args = ProjectVersionDeleteArgs::new("TEST".to_string(), 3, false);
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = mock(None, Some(sample_version()));
        let args = ProjectVersionDeleteArgs::new("TEST".to_string(), 3, true);
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectVersionDeleteArgs::new("TEST".to_string(), 3, false);
        let err = delete_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
