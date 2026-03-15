use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueSharedFileListArgs {
    key: String,
    json: bool,
}

impl IssueSharedFileListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &IssueSharedFileListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &IssueSharedFileListArgs, api: &dyn BacklogApi) -> Result<()> {
    let files = api.get_issue_shared_files(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&files).context("Failed to serialize JSON")?
        );
    } else {
        for f in &files {
            println!(
                "[{}] {}/{} ({} bytes)",
                f.id.to_string().cyan().bold(),
                f.dir,
                f.name,
                f.size
            );
        }
    }
    Ok(())
}

#[cfg(test)]
use crate::api::issue::IssueSharedFile;
#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
pub(crate) fn sample_shared_file() -> IssueSharedFile {
    IssueSharedFile {
        id: 1,
        dir: "/docs".to_string(),
        name: "spec.pdf".to_string(),
        size: 2048,
        extra: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        files: Option<Vec<IssueSharedFile>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_issue_shared_files(&self, _key: &str) -> anyhow::Result<Vec<IssueSharedFile>> {
            self.files.clone().ok_or_else(|| anyhow!("no files"))
        }
    }

    fn args(json: bool) -> IssueSharedFileListArgs {
        IssueSharedFileListArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_shared_file()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_shared_file()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { files: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no files"));
    }
}
