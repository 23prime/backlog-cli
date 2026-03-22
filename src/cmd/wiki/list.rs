use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, wiki::WikiListItem};

pub struct WikiListArgs {
    project_id_or_key: String,
    keyword: Option<String>,
    json: bool,
}

impl WikiListArgs {
    pub fn new(project_id_or_key: String, keyword: Option<String>, json: bool) -> Self {
        Self {
            project_id_or_key,
            keyword,
            json,
        }
    }
}

pub fn list(args: &WikiListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &WikiListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> =
        vec![("projectIdOrKey".to_string(), args.project_id_or_key.clone())];
    if let Some(kw) = &args.keyword {
        params.push(("keyword".to_string(), kw.clone()));
    }
    let wikis = api.get_wikis(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&wikis).context("Failed to serialize JSON")?
        );
    } else {
        for wiki in &wikis {
            println!("{}", format_wiki_row(wiki));
        }
    }
    Ok(())
}

pub fn format_wiki_row(wiki: &WikiListItem) -> String {
    let tags = if wiki.tags.is_empty() {
        String::new()
    } else {
        format!(
            " [{}]",
            wiki.tags
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    format!("{}{}", wiki.name.cyan().bold(), tags)
}

#[cfg(test)]
pub(crate) mod tests_helper {
    use std::collections::BTreeMap;

    use crate::api::wiki::{Wiki, WikiListItem, WikiTag, WikiUser};

    pub fn sample_wiki_user() -> WikiUser {
        WikiUser {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    pub fn sample_wiki() -> Wiki {
        use std::collections::BTreeMap;
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

    pub fn sample_wiki_list_item() -> WikiListItem {
        WikiListItem {
            id: 1,
            project_id: 1,
            name: "Home".to_string(),
            tags: vec![WikiTag {
                id: 1,
                name: "guide".to_string(),
            }],
            created_user: sample_wiki_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_wiki_user(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::WikiListItem;
    use anyhow::anyhow;
    use tests_helper::{sample_wiki_list_item, sample_wiki_user};

    struct MockApi {
        wikis: Option<Vec<WikiListItem>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wikis(&self, _params: &[(String, String)]) -> anyhow::Result<Vec<WikiListItem>> {
            self.wikis.clone().ok_or_else(|| anyhow!("no wikis"))
        }
    }

    fn args(json: bool) -> WikiListArgs {
        WikiListArgs::new("TEST".to_string(), None, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            wikis: Some(vec![sample_wiki_list_item()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            wikis: Some(vec![sample_wiki_list_item()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { wikis: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no wikis"));
    }

    #[test]
    fn format_wiki_row_with_tags() {
        let wiki = sample_wiki_list_item();
        let row = format_wiki_row(&wiki);
        assert!(row.contains("Home"));
        assert!(row.contains("guide"));
    }

    #[test]
    fn format_wiki_row_without_tags() {
        let mut wiki = sample_wiki_list_item();
        wiki.tags.clear();
        let row = format_wiki_row(&wiki);
        assert!(row.contains("Home"));
        assert!(!row.contains("guide"));
    }

    #[test]
    fn list_with_keyword_builds_params() {
        let api = MockApi {
            wikis: Some(vec![]),
        };
        let args = WikiListArgs::new("TEST".to_string(), Some("guide".to_string()), false);
        assert!(list_with(&args, &api).is_ok());
    }

    #[test]
    fn sample_wiki_user_has_expected_fields() {
        let u = sample_wiki_user();
        assert_eq!(u.name, "John Doe");
        assert_eq!(u.user_id.as_deref(), Some("john"));
    }
}
