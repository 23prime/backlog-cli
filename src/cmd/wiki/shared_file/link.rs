use anstream::println;
use anyhow::{Context, Result, bail};

use crate::api::{BacklogApi, BacklogClient};

#[derive(Debug)]
pub struct WikiSharedFileLinkArgs {
    wiki_id: u64,
    shared_file_ids: Vec<u64>,
    json: bool,
}

impl WikiSharedFileLinkArgs {
    pub fn try_new(wiki_id: u64, shared_file_ids: Vec<u64>, json: bool) -> Result<Self> {
        if shared_file_ids.is_empty() {
            bail!("at least one --shared-file-id must be specified");
        }
        Ok(Self {
            wiki_id,
            shared_file_ids,
            json,
        })
    }
}

pub fn link(args: &WikiSharedFileLinkArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    link_with(args, &client)
}

pub fn link_with(args: &WikiSharedFileLinkArgs, api: &dyn BacklogApi) -> Result<()> {
    let files = api.link_wiki_shared_files(args.wiki_id, &args.shared_file_ids)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&files).context("Failed to serialize JSON")?
        );
    } else {
        println!("Linked {} file(s).", files.len());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::WikiSharedFile;
    use crate::cmd::wiki::shared_file::list::sample_wiki_shared_file;
    use anyhow::anyhow;

    struct MockApi {
        files: Option<Vec<WikiSharedFile>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn link_wiki_shared_files(
            &self,
            _wiki_id: u64,
            _shared_file_ids: &[u64],
        ) -> anyhow::Result<Vec<WikiSharedFile>> {
            self.files.clone().ok_or_else(|| anyhow!("no files"))
        }
    }

    fn args(json: bool) -> WikiSharedFileLinkArgs {
        WikiSharedFileLinkArgs::try_new(1, vec![1], json).unwrap()
    }

    #[test]
    fn link_with_text_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_wiki_shared_file()]),
        };
        assert!(link_with(&args(false), &api).is_ok());
    }

    #[test]
    fn link_with_json_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_wiki_shared_file()]),
        };
        assert!(link_with(&args(true), &api).is_ok());
    }

    #[test]
    fn link_with_propagates_api_error() {
        let api = MockApi { files: None };
        let err = link_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no files"));
    }

    #[test]
    fn try_new_rejects_empty_ids() {
        let err = WikiSharedFileLinkArgs::try_new(1, vec![], false).unwrap_err();
        assert!(err.to_string().contains("at least one"));
    }
}
