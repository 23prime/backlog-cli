use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, wiki::Wiki};

pub struct WikiShowArgs {
    wiki_id: u64,
    json: bool,
}

impl WikiShowArgs {
    pub fn new(wiki_id: u64, json: bool) -> Self {
        Self { wiki_id, json }
    }
}

pub fn show(args: &WikiShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &WikiShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let wiki = api.get_wiki(args.wiki_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&wiki).context("Failed to serialize JSON")?
        );
    } else {
        print_wiki(&wiki);
    }
    Ok(())
}

pub fn print_wiki(wiki: &Wiki) {
    println!("{}", wiki.name.cyan().bold());
    if !wiki.tags.is_empty() {
        let tag_names: Vec<&str> = wiki.tags.iter().map(|t| t.name.as_str()).collect();
        println!("  Tags:    {}", tag_names.join(", "));
    }
    println!("  Created: {}", wiki.created);
    println!("  Updated: {}", wiki.updated);
    if !wiki.content.is_empty() {
        println!("\n{}", wiki.content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::Wiki;
    use crate::cmd::wiki::list::tests_helper::sample_wiki;
    use anyhow::anyhow;

    struct MockApi {
        wiki: Option<Wiki>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wiki(&self, _wiki_id: u64) -> anyhow::Result<Wiki> {
            self.wiki.clone().ok_or_else(|| anyhow!("no wiki"))
        }
    }

    fn args(json: bool) -> WikiShowArgs {
        WikiShowArgs::new(1, json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { wiki: None };
        let err = show_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no wiki"));
    }
}
