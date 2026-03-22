use anstream::println;
use anyhow::Result;
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
        crate::cmd::print_json(&wiki)?;
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
    use crate::api::wiki::{Wiki, WikiTag};
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        wiki: Option<Wiki>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wiki(&self, _wiki_id: u64) -> anyhow::Result<Wiki> {
            self.wiki.clone().ok_or_else(|| anyhow!("no wiki"))
        }
    }

    fn sample_wiki() -> Wiki {
        Wiki {
            id: 1,
            project_id: 1,
            name: "Home".to_string(),
            content: "# Home\nWelcome!".to_string(),
            tags: vec![WikiTag {
                id: 1,
                name: "guide".to_string(),
            }],
            created_user: sample_wiki_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_wiki_user(),
            updated: "2024-01-02T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
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
