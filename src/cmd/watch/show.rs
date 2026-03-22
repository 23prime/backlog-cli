use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchShowArgs {
    watching_id: u64,
    json: bool,
}

impl WatchShowArgs {
    pub fn new(watching_id: u64, json: bool) -> Self {
        Self { watching_id, json }
    }
}

pub fn show(args: &WatchShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &WatchShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let watching = api.get_watching(args.watching_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&watching).context("Failed to serialize JSON")?
        );
    } else {
        println!(
            "[{}] {} ({})",
            watching.id, watching.issue.summary, watching.issue.issue_key
        );
        if let Some(ref note) = watching.note
            && !note.is_empty()
        {
            println!("note: {note}");
        }
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
        fn get_watching(&self, _watching_id: u64) -> anyhow::Result<Watching> {
            self.watching.clone().ok_or_else(|| anyhow!("not found"))
        }
    }

    fn args(json: bool) -> WatchShowArgs {
        WatchShowArgs::new(1, json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { watching: None };
        let err = show_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }
}
