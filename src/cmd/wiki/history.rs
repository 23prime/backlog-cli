use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, wiki::WikiHistory};

pub struct WikiHistoryArgs {
    wiki_id: u64,
    json: bool,
}

impl WikiHistoryArgs {
    pub fn new(wiki_id: u64, json: bool) -> Self {
        Self { wiki_id, json }
    }
}

pub fn history(args: &WikiHistoryArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    history_with(args, &client)
}

pub fn history_with(args: &WikiHistoryArgs, api: &dyn BacklogApi) -> Result<()> {
    let entries = api.get_wiki_history(args.wiki_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&entries).context("Failed to serialize JSON")?
        );
    } else {
        for entry in &entries {
            println!("{}", format_history_row(entry));
        }
    }
    Ok(())
}

pub fn format_history_row(entry: &WikiHistory) -> String {
    format!(
        "{} {} — {}",
        format!("v{}", entry.version).cyan().bold(),
        entry.name,
        entry.created
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::{WikiHistory, WikiUser};
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use anyhow::anyhow;

    struct MockApi {
        history: Option<Vec<WikiHistory>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wiki_history(&self, _wiki_id: u64) -> anyhow::Result<Vec<WikiHistory>> {
            self.history.clone().ok_or_else(|| anyhow!("no history"))
        }
    }

    fn sample_history(user: WikiUser) -> WikiHistory {
        WikiHistory {
            page_id: 1,
            version: 3,
            name: "Home".to_string(),
            content: "# Home v3".to_string(),
            created_user: user,
            created: "2024-03-01T00:00:00Z".to_string(),
        }
    }

    fn args(json: bool) -> WikiHistoryArgs {
        WikiHistoryArgs::new(1, json)
    }

    #[test]
    fn history_with_text_output_succeeds() {
        let api = MockApi {
            history: Some(vec![sample_history(sample_wiki_user())]),
        };
        assert!(history_with(&args(false), &api).is_ok());
    }

    #[test]
    fn history_with_json_output_succeeds() {
        let api = MockApi {
            history: Some(vec![sample_history(sample_wiki_user())]),
        };
        assert!(history_with(&args(true), &api).is_ok());
    }

    #[test]
    fn history_with_propagates_api_error() {
        let api = MockApi { history: None };
        let err = history_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no history"));
    }

    #[test]
    fn format_history_row_contains_version_and_name() {
        let entry = sample_history(sample_wiki_user());
        let row = format_history_row(&entry);
        assert!(row.contains("v3"));
        assert!(row.contains("Home"));
        assert!(row.contains("2024-03-01"));
    }
}
