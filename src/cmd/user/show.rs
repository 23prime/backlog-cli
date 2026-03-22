use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, user::User};

pub struct UserShowArgs {
    id: u64,
    json: bool,
}

impl UserShowArgs {
    pub fn new(id: u64, json: bool) -> Self {
        Self { id, json }
    }
}

pub fn show(args: &UserShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &UserShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let user = api.get_user(args.id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&user).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_user_text(&user));
    }
    Ok(())
}

fn format_user_text(u: &User) -> String {
    let user_id = u.user_id.as_deref().unwrap_or("-");
    let mail = u.mail_address.as_deref().unwrap_or("-");
    let lang = u.lang.as_deref().unwrap_or("-");
    let last_login = u.last_login_time.as_deref().unwrap_or("-");
    format!(
        "ID:           {}\nUser ID:      {}\nName:         {}\nMail:         {}\nRole:         {}\nLang:         {}\nLast login:   {}",
        u.id.to_string().bold(),
        user_id,
        u.name,
        mail,
        u.role_type,
        lang,
        last_login,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::cmd::user::sample_user;
    use anyhow::anyhow;

    struct MockApi {
        user: Option<User>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_user(&self, _user_id: u64) -> anyhow::Result<User> {
            self.user.clone().ok_or_else(|| anyhow!("no user"))
        }
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(show_with(&UserShowArgs::new(123, false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(show_with(&UserShowArgs::new(123, true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { user: None };
        let err = show_with(&UserShowArgs::new(999, false), &api).unwrap_err();
        assert!(err.to_string().contains("no user"));
    }

    #[test]
    fn format_user_text_contains_fields() {
        let text = format_user_text(&sample_user());
        assert!(text.contains("User ID:      john"));
        assert!(text.contains("john"));
        assert!(text.contains("John Doe"));
        assert!(text.contains("john@example.com"));
    }

    #[test]
    fn format_user_text_handles_nulls() {
        let user = User {
            id: 1,
            user_id: None,
            name: "Bot".to_string(),
            mail_address: None,
            role_type: 2,
            lang: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        };
        let text = format_user_text(&user);
        assert!(text.contains("Bot"));
        assert!(text.contains('-'));
    }
}
