use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct UserUpdateArgs {
    user_id: u64,
    name: Option<String>,
    password: Option<String>,
    mail_address: Option<String>,
    role_type: Option<u8>,
    json: bool,
}

impl UserUpdateArgs {
    pub fn new(
        user_id: u64,
        name: Option<String>,
        password: Option<String>,
        mail_address: Option<String>,
        role_type: Option<u8>,
        json: bool,
    ) -> Self {
        Self {
            user_id,
            name,
            password,
            mail_address,
            role_type,
            json,
        }
    }
}

pub fn update(args: &UserUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &UserUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(ref name) = args.name {
        params.push(("name".to_string(), name.clone()));
    }
    if let Some(ref password) = args.password {
        params.push(("password".to_string(), password.clone()));
    }
    if let Some(ref mail_address) = args.mail_address {
        params.push(("mailAddress".to_string(), mail_address.clone()));
    }
    if let Some(role_type) = args.role_type {
        params.push(("roleType".to_string(), role_type.to_string()));
    }
    let user = api.update_user(args.user_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&user).context("Failed to serialize JSON")?
        );
    } else {
        println!(
            "Updated: {} ({}) [roleType: {}]",
            user.user_id.as_deref().unwrap_or("-"),
            user.name,
            user.role_type
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::User;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        user: Option<User>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn update_user(&self, _user_id: u64, _params: &[(String, String)]) -> anyhow::Result<User> {
            self.user.clone().ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn sample_user() -> User {
        User {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            mail_address: Some("john@example.com".to_string()),
            role_type: 2,
            lang: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> UserUpdateArgs {
        UserUpdateArgs::new(1, Some("John Doe".to_string()), None, None, None, json)
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { user: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }
}
