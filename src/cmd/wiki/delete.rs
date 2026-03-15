use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::wiki::show::print_wiki;

pub struct WikiDeleteArgs {
    wiki_id: u64,
    mail_notify: bool,
    json: bool,
}

impl WikiDeleteArgs {
    pub fn new(wiki_id: u64, mail_notify: bool, json: bool) -> Self {
        Self {
            wiki_id,
            mail_notify,
            json,
        }
    }
}

pub fn delete(args: &WikiDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &WikiDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if args.mail_notify {
        params.push(("mailNotify".to_string(), "true".to_string()));
    }
    let wiki = api.delete_wiki(args.wiki_id, &params)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::Wiki;
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        wiki: Option<Wiki>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_wiki(&self, _wiki_id: u64, _params: &[(String, String)]) -> anyhow::Result<Wiki> {
            self.wiki.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_wiki() -> Wiki {
        Wiki {
            id: 1,
            project_id: 1,
            name: "Home".to_string(),
            content: "# Home".to_string(),
            tags: vec![],
            created_user: sample_wiki_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_wiki_user(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> WikiDeleteArgs {
        WikiDeleteArgs::new(1, false, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { wiki: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
