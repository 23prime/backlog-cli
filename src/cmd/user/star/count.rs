use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct UserStarCountArgs {
    user_id: u64,
    since: Option<String>,
    until: Option<String>,
    json: bool,
}

impl UserStarCountArgs {
    pub fn new(user_id: u64, since: Option<String>, until: Option<String>, json: bool) -> Self {
        Self {
            user_id,
            since,
            until,
            json,
        }
    }
}

pub fn count(args: &UserStarCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &UserStarCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(ref since) = args.since {
        params.push(("since".to_string(), since.clone()));
    }
    if let Some(ref until) = args.until {
        params.push(("until".to_string(), until.clone()));
    }
    let result = api.count_user_stars(args.user_id, &params)?;
    if args.json {
        crate::cmd::print_json(&result)?;
    } else {
        println!("{}", result.count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::StarCount;
    use anyhow::anyhow;

    struct MockApi {
        result: Option<StarCount>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn count_user_stars(
            &self,
            _user_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<StarCount> {
            self.result.clone().ok_or_else(|| anyhow!("count failed"))
        }
    }

    fn args(json: bool) -> UserStarCountArgs {
        UserStarCountArgs::new(1, None, None, json)
    }

    #[test]
    fn count_with_text_output_succeeds() {
        let api = MockApi {
            result: Some(StarCount { count: 42 }),
        };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi {
            result: Some(StarCount { count: 42 }),
        };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi { result: None };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("count failed"));
    }
}
