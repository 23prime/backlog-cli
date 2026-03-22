use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct WikiTagListArgs {
    project_id_or_key: Option<String>,
    json: bool,
}

impl WikiTagListArgs {
    pub fn new(project_id_or_key: Option<String>, json: bool) -> Self {
        Self {
            project_id_or_key,
            json,
        }
    }
}

pub fn list(args: &WikiTagListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &WikiTagListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(ref key) = args.project_id_or_key {
        params.push(("projectIdOrKey".to_string(), key.clone()));
    }
    let tags = api.get_wiki_tags(&params)?;
    if args.json {
        crate::cmd::print_json(&tags)?;
    } else {
        for tag in &tags {
            println!("[{}] {}", tag.id, tag.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::WikiTag;
    use anyhow::anyhow;

    struct MockApi {
        tags: Option<Vec<WikiTag>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wiki_tags(&self, _params: &[(String, String)]) -> anyhow::Result<Vec<WikiTag>> {
            self.tags.clone().ok_or_else(|| anyhow!("no tags"))
        }
    }

    fn args(json: bool) -> WikiTagListArgs {
        WikiTagListArgs::new(Some("TEST".to_string()), json)
    }

    fn sample_tag() -> WikiTag {
        WikiTag {
            id: 1,
            name: "guide".to_string(),
        }
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            tags: Some(vec![sample_tag()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            tags: Some(vec![sample_tag()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { tags: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no tags"));
    }
}
