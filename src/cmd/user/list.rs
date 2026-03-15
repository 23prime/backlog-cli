use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, user::User};

pub struct UserListArgs {
    json: bool,
}

impl UserListArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn list(args: &UserListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &UserListArgs, api: &dyn BacklogApi) -> Result<()> {
    let users = api.get_users()?;
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

fn format_user_row(u: &User) -> String {
    match u.user_id.as_deref() {
        Some(user_id) if !user_id.is_empty() => format!("[{}] {} ({})", u.id, u.name, user_id),
        _ => format!("[{}] {}", u.id, u.name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        users: Option<Vec<User>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_users(&self) -> anyhow::Result<Vec<User>> {
            self.users.clone().ok_or_else(|| anyhow!("no users"))
        }
    }

    fn sample_user() -> User {
        User {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            mail_address: Some("john@example.com".to_string()),
            role_type: 1,
            lang: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn format_user_row_with_user_id() {
        let text = format_user_row(&sample_user());
        assert!(text.contains("[1]"));
        assert!(text.contains("John Doe"));
        assert!(text.contains("(john)"));
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
        assert!(list_with(&UserListArgs::new(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            users: Some(vec![sample_user()]),
        };
        assert!(list_with(&UserListArgs::new(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { users: None };
        let err = list_with(&UserListArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no users"));
    }
}
