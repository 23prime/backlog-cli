use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WikiCountArgs {
    project_id_or_key: Option<String>,
    json: bool,
}

impl WikiCountArgs {
    pub fn new(project_id_or_key: Option<String>, json: bool) -> Self {
        Self {
            project_id_or_key,
            json,
        }
    }
}

pub fn count(args: &WikiCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &WikiCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(ref key) = args.project_id_or_key {
        params.push(("projectIdOrKey".to_string(), key.clone()));
    }
    let result = api.get_wiki_count(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", result.count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::WikiCount;
    use anyhow::anyhow;

    struct MockApi {
        result: Option<WikiCount>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wiki_count(&self, _params: &[(String, String)]) -> anyhow::Result<WikiCount> {
            self.result.clone().ok_or_else(|| anyhow!("no count"))
        }
    }

    fn args(json: bool) -> WikiCountArgs {
        WikiCountArgs::new(Some("TEST".to_string()), json)
    }

    #[test]
    fn count_with_text_output_succeeds() {
        let api = MockApi {
            result: Some(WikiCount { count: 5 }),
        };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi {
            result: Some(WikiCount { count: 5 }),
        };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi { result: None };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no count"));
    }
}
