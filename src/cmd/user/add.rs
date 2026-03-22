use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct UserAddArgs {
    user_id: String,
    password: String,
    name: String,
    mail_address: String,
    role_type: u8,
    json: bool,
}

impl UserAddArgs {
    pub fn new(
        user_id: String,
        password: String,
        name: String,
        mail_address: String,
        role_type: u8,
        json: bool,
    ) -> Self {
        Self {
            user_id,
            password,
            name,
            mail_address,
            role_type,
            json,
        }
    }
}

pub fn add(args: &UserAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &UserAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![
        ("userId".to_string(), args.user_id.clone()),
        ("password".to_string(), args.password.clone()),
        ("name".to_string(), args.name.clone()),
        ("mailAddress".to_string(), args.mail_address.clone()),
        ("roleType".to_string(), args.role_type.to_string()),
    ];
    let user = api.add_user(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&user).context("Failed to serialize JSON")?
        );
    } else {
        println!(
            "Added: {} ({}) [roleType: {}]",
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
    use crate::cmd::user::sample_user;
    use anyhow::anyhow;

    struct MockApi {
        user: Option<User>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn add_user(&self, _params: &[(String, String)]) -> anyhow::Result<User> {
            self.user.clone().ok_or_else(|| anyhow!("add failed"))
        }
    }

    fn args(json: bool) -> UserAddArgs {
        UserAddArgs::new(
            "john".to_string(),
            "secret".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
            2,
            json,
        )
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(add_with(&args(true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi { user: None };
        let err = add_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }
}
