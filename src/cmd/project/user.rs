use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectUser};

pub struct ProjectUserListArgs {
    key: String,
    json: bool,
}

impl ProjectUserListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &ProjectUserListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectUserListArgs, api: &dyn BacklogApi) -> Result<()> {
    let users = api.get_project_users(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&users).context("Failed to serialize JSON")?
        );
    } else {
        for u in &users {
            println!("{}", format_user_row(u));
        }
    }
    Ok(())
}

fn format_user_row(u: &ProjectUser) -> String {
    match u.user_id.as_deref() {
        Some(user_id) if !user_id.is_empty() => format!("[{}] {}", user_id, u.name),
        _ => format!("[{}] {}", u.id, u.name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        users: Option<Vec<ProjectUser>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_users(&self, _key: &str) -> anyhow::Result<Vec<ProjectUser>> {
            self.users.clone().ok_or_else(|| anyhow!("no users"))
        }
    }

    fn sample_user() -> ProjectUser {
        ProjectUser {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            role_type: 1,
            lang: Some("ja".to_string()),
            mail_address: Some("john@example.com".to_string()),
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn format_user_row_with_user_id() {
        let text = format_user_row(&sample_user());
        assert!(text.contains("[john]"));
        assert!(text.contains("John Doe"));
    }

    #[test]
    fn format_user_row_without_user_id() {
        let mut u = sample_user();
        u.user_id = None;
        let text = format_user_row(&u);
        assert!(text.contains("[1]"));
        assert!(text.contains("John Doe"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            users: Some(vec![sample_user()]),
        };
        assert!(list_with(&ProjectUserListArgs::new("TEST".to_string(), false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            users: Some(vec![sample_user()]),
        };
        assert!(list_with(&ProjectUserListArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { users: None };
        let err =
            list_with(&ProjectUserListArgs::new("TEST".to_string(), false), &api).unwrap_err();
        assert!(err.to_string().contains("no users"));
    }
}
