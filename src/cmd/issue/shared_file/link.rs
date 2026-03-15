use anstream::println;
use anyhow::{Context, Result, bail};

use crate::api::{BacklogApi, BacklogClient};

#[derive(Debug)]
pub struct IssueSharedFileLinkArgs {
    key: String,
    shared_file_ids: Vec<u64>,
    json: bool,
}

impl IssueSharedFileLinkArgs {
    pub fn try_new(key: String, shared_file_ids: Vec<u64>, json: bool) -> Result<Self> {
        if shared_file_ids.is_empty() {
            bail!("at least one --shared-file-id must be specified");
        }
        Ok(Self {
            key,
            shared_file_ids,
            json,
        })
    }
}

pub fn link(args: &IssueSharedFileLinkArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    link_with(args, &client)
}

pub fn link_with(args: &IssueSharedFileLinkArgs, api: &dyn BacklogApi) -> Result<()> {
    let files = api.link_issue_shared_files(&args.key, &args.shared_file_ids)?;
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
    use crate::api::issue::IssueSharedFile;
    use crate::cmd::issue::shared_file::list::sample_shared_file;
    use anyhow::anyhow;

    struct MockApi {
        files: Option<Vec<IssueSharedFile>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn link_issue_shared_files(
            &self,
            _key: &str,
            _shared_file_ids: &[u64],
        ) -> anyhow::Result<Vec<IssueSharedFile>> {
            self.files.clone().ok_or_else(|| anyhow!("no files"))
        }
    }

    fn args(json: bool) -> IssueSharedFileLinkArgs {
        IssueSharedFileLinkArgs::try_new("TEST-1".to_string(), vec![1], json).unwrap()
    }

    #[test]
    fn link_with_text_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_shared_file()]),
        };
        assert!(link_with(&args(false), &api).is_ok());
    }

    #[test]
    fn link_with_json_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_shared_file()]),
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
        let err =
            IssueSharedFileLinkArgs::try_new("TEST-1".to_string(), vec![], false).unwrap_err();
        assert!(err.to_string().contains("at least one"));
    }
}
