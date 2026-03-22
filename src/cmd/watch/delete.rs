use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchDeleteArgs {
    watching_id: u64,
    json: bool,
}

impl WatchDeleteArgs {
    pub fn new(watching_id: u64, json: bool) -> Self {
        Self { watching_id, json }
    }
}

pub fn delete(args: &WatchDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &WatchDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let watching = api.delete_watching(args.watching_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&watching).context("Failed to serialize JSON")?
        );
    } else {
        println!(
            "Deleted: [{}] {} ({})",
            watching.id, watching.issue.summary, watching.issue.issue_key
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::watch::Watching;
    use crate::cmd::watch::sample_watching;
    use anyhow::anyhow;

    struct MockApi {
        watching: Option<Watching>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_watching(&self, _watching_id: u64) -> anyhow::Result<Watching> {
            self.watching
                .clone()
                .ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn args(json: bool) -> WatchDeleteArgs {
        WatchDeleteArgs::new(1, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { watching: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
