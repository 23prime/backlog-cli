use anstream::println;
use anyhow::{Context, Result};

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
    if let Some(ar) = args.already_read {
        params.push(("alreadyRead".to_string(), ar.to_string()));
    } else if let Some(rar) = args.resource_already_read {
        params.push(("resourceAlreadyRead".to_string(), rar.to_string()));
    }
    let result = api.count_watchings(args.user_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).context("Failed to serialize JSON")?
        );
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
    use std::cell::RefCell;

    struct MockApi {
        count: Option<WatchingCount>,
        captured: RefCell<Vec<(String, String)>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn count_watchings(
            &self,
            _user_id: u64,
            params: &[(String, String)],
        ) -> anyhow::Result<WatchingCount> {
            *self.captured.borrow_mut() = params.to_vec();
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
            captured: RefCell::new(vec![]),
        };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi {
            count: Some(WatchingCount { count: 5 }),
            captured: RefCell::new(vec![]),
        };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi {
            count: None,
            captured: RefCell::new(vec![]),
        };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("count failed"));
    }

    #[test]
    fn already_read_takes_precedence_over_resource_already_read() {
        let api = MockApi {
            count: Some(WatchingCount { count: 3 }),
            captured: RefCell::new(vec![]),
        };
        let args = WatchCountArgs::new(1, Some(true), Some(false), false);
        count_with(&args, &api).unwrap();
        let params = api.captured.borrow();
        assert!(
            params
                .iter()
                .any(|(k, v)| k == "alreadyRead" && v == "false")
        );
        assert!(!params.iter().any(|(k, _)| k == "resourceAlreadyRead"));
    }
}
