use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchUpdateArgs {
    watching_id: u64,
    note: String,
    json: bool,
}

impl WatchUpdateArgs {
    pub fn new(watching_id: u64, note: String, json: bool) -> Self {
        Self {
            watching_id,
            note,
            json,
        }
    }
}

pub fn update(args: &WatchUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &WatchUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![("note".to_string(), args.note.clone())];
    let watching = api.update_watching(args.watching_id, &params)?;
    if args.json {
        crate::cmd::print_json(&watching)?;
    } else {
        println!(
            "Updated: [{}] {} ({})",
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
        fn update_watching(
            &self,
            _watching_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<Watching> {
            self.watching
                .clone()
                .ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn args(json: bool) -> WatchUpdateArgs {
        WatchUpdateArgs::new(1, "updated note".to_string(), json)
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { watching: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }
}
