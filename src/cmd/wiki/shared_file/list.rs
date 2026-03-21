use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WikiSharedFileListArgs {
    wiki_id: u64,
    json: bool,
}

impl WikiSharedFileListArgs {
    pub fn new(wiki_id: u64, json: bool) -> Self {
        Self { wiki_id, json }
    }
}

pub fn list(args: &WikiSharedFileListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &WikiSharedFileListArgs, api: &dyn BacklogApi) -> Result<()> {
    let files = api.get_wiki_shared_files(args.wiki_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&files).context("Failed to serialize JSON")?
        );
    } else {
        for f in &files {
            let sep = if f.dir.ends_with('/') { "" } else { "/" };
            println!("[{}] {}{}{} ({} bytes)", f.id, f.dir, sep, f.name, f.size);
        }
    }
    Ok(())
}

#[cfg(test)]
use crate::api::wiki::WikiSharedFile;
#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
pub(crate) fn sample_wiki_shared_file() -> WikiSharedFile {
    WikiSharedFile {
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
        files: Option<Vec<WikiSharedFile>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_wiki_shared_files(&self, _wiki_id: u64) -> anyhow::Result<Vec<WikiSharedFile>> {
            self.files.clone().ok_or_else(|| anyhow!("no files"))
        }
    }

    fn args(json: bool) -> WikiSharedFileListArgs {
        WikiSharedFileListArgs::new(1, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_wiki_shared_file()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_wiki_shared_file()]),
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
