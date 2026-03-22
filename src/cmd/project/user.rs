use anstream::println;
use anyhow::{Context, Result};

use super::format_project_user_row;
use crate::api::{BacklogApi, BacklogClient};

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
            println!("{}", format_project_user_row(u));
        }
    }
    Ok(())
}

pub struct ProjectUserAddArgs {
    key: String,
    user_id: u64,
    json: bool,
}

impl ProjectUserAddArgs {
    pub fn new(key: String, user_id: u64, json: bool) -> Self {
        Self { key, user_id, json }
    }
}

pub fn add(args: &ProjectUserAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectUserAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let user = api.add_project_user(&args.key, args.user_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&user).context("Failed to serialize JSON")?
        );
    } else {
        println!("Added: {}", format_project_user_row(&user));
    }
    Ok(())
}

pub struct ProjectUserDeleteArgs {
    key: String,
    user_id: u64,
    json: bool,
}

impl ProjectUserDeleteArgs {
    pub fn new(key: String, user_id: u64, json: bool) -> Self {
        Self { key, user_id, json }
    }
}

pub fn delete(args: &ProjectUserDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectUserDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let user = api.delete_project_user(&args.key, args.user_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&user).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", format_project_user_row(&user));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::project::ProjectUser;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        users: Option<Vec<ProjectUser>>,
        user: Option<ProjectUser>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_users(&self, _key: &str) -> anyhow::Result<Vec<ProjectUser>> {
            self.users.clone().ok_or_else(|| anyhow!("no users"))
        }

        fn add_project_user(&self, _key: &str, _user_id: u64) -> anyhow::Result<ProjectUser> {
            self.user.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn delete_project_user(&self, _key: &str, _user_id: u64) -> anyhow::Result<ProjectUser> {
            self.user.clone().ok_or_else(|| anyhow!("delete failed"))
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
        let text = format_project_user_row(&sample_user());
        assert!(text.contains("[john]"));
        assert!(text.contains("John Doe"));
    }

    #[test]
    fn format_user_row_without_user_id() {
        let mut u = sample_user();
        u.user_id = None;
        let text = format_project_user_row(&u);
        assert!(text.contains("[1]"));
        assert!(text.contains("John Doe"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            users: Some(vec![sample_user()]),
            user: None,
        };
        assert!(list_with(&ProjectUserListArgs::new("TEST".to_string(), false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            users: Some(vec![sample_user()]),
            user: None,
        };
        assert!(list_with(&ProjectUserListArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi {
            users: None,
            user: None,
        };
        let err =
            list_with(&ProjectUserListArgs::new("TEST".to_string(), false), &api).unwrap_err();
        assert!(err.to_string().contains("no users"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            users: None,
            user: Some(sample_user()),
        };
        assert!(add_with(&ProjectUserAddArgs::new("TEST".to_string(), 1, false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            users: None,
            user: Some(sample_user()),
        };
        assert!(add_with(&ProjectUserAddArgs::new("TEST".to_string(), 1, true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi {
            users: None,
            user: None,
        };
        let err =
            add_with(&ProjectUserAddArgs::new("TEST".to_string(), 1, false), &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            users: None,
            user: Some(sample_user()),
        };
        assert!(
            delete_with(
                &ProjectUserDeleteArgs::new("TEST".to_string(), 1, false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            users: None,
            user: Some(sample_user()),
        };
        assert!(
            delete_with(
                &ProjectUserDeleteArgs::new("TEST".to_string(), 1, true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi {
            users: None,
            user: None,
        };
        let err = delete_with(
            &ProjectUserDeleteArgs::new("TEST".to_string(), 1, false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
