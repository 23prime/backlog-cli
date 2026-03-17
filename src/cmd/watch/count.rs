use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchCountArgs {
    user_id: u64,
    resource_already_read: Option<bool>,
    already_read: Option<bool>,
    json: bool,
}

impl WatchCountArgs {
    pub fn new(
        user_id: u64,
        resource_already_read: Option<bool>,
        already_read: Option<bool>,
        json: bool,
    ) -> Self {
        Self {
            user_id,
            resource_already_read,
            already_read,
            json,
        }
    }
}

pub fn count(args: &WatchCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &WatchCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(rar) = args.resource_already_read {
        params.push(("resourceAlreadyRead".to_string(), rar.to_string()));
    }
    if let Some(ar) = args.already_read {
        params.push(("alreadyRead".to_string(), ar.to_string()));
    }
    let result = api.count_watchings(args.user_id, &params)?;
    if args.json {
        println!("{{\"count\":{}}}", result.count);
    } else {
        println!("{}", result.count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::watch::WatchingCount;
    use anyhow::anyhow;

    struct MockApi {
        count: Option<WatchingCount>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn count_watchings(
            &self,
            _user_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<WatchingCount> {
            self.count.clone().ok_or_else(|| anyhow!("count failed"))
        }
    }

    fn args(json: bool) -> WatchCountArgs {
        WatchCountArgs::new(1, None, None, json)
    }

    #[test]
    fn count_with_text_output_succeeds() {
        let api = MockApi {
            count: Some(WatchingCount { count: 5 }),
        };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi {
            count: Some(WatchingCount { count: 5 }),
        };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi { count: None };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("count failed"));
    }
}
