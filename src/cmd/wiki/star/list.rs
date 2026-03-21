use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WikiStarListArgs {
    wiki_id: u64,
    json: bool,
}

impl WikiStarListArgs {
    pub fn new(wiki_id: u64, json: bool) -> Self {
        Self { wiki_id, json }
    }
}

pub fn list(args: &WikiStarListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &WikiStarListArgs, api: &dyn BacklogApi) -> Result<()> {
    let stars = api.get_wiki_stars(args.wiki_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&stars).context("Failed to serialize JSON")?
        );
    } else {
        for star in &stars {
            println!("[{}] {} ({})", star.id, star.title, star.presenter.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::{Star, User};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        stars: Option<Vec<Star>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wiki_stars(&self, _wiki_id: u64) -> anyhow::Result<Vec<Star>> {
            self.stars.clone().ok_or_else(|| anyhow!("no stars"))
        }
    }

    fn sample_star() -> Star {
        Star {
            id: 1,
            comment: None,
            url: "https://example.backlog.com/wiki/TEST/Home".to_string(),
            title: "Home".to_string(),
            presenter: User {
                id: 1,
                user_id: Some("john".to_string()),
                name: "John Doe".to_string(),
                mail_address: None,
                role_type: 1,
                lang: None,
                last_login_time: None,
                extra: BTreeMap::new(),
            },
            created: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> WikiStarListArgs {
        WikiStarListArgs::new(1, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            stars: Some(vec![sample_star()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            stars: Some(vec![sample_star()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { stars: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no stars"));
    }
}
