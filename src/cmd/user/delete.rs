use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct UserDeleteArgs {
    user_id: u64,
    json: bool,
}

impl UserDeleteArgs {
    pub fn new(user_id: u64, json: bool) -> Self {
        Self { user_id, json }
    }
}

pub fn delete(args: &UserDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &UserDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let user = api.delete_user(args.user_id)?;
    if args.json {
        crate::cmd::print_json(&user)?;
    } else {
        println!(
            "Deleted: {} ({})",
            user.user_id.as_deref().unwrap_or("-"),
            user.name
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
        fn delete_user(&self, _user_id: u64) -> anyhow::Result<User> {
            self.user.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn args(json: bool) -> UserDeleteArgs {
        UserDeleteArgs::new(1, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { user: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
