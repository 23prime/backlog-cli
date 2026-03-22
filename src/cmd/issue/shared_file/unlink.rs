use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueSharedFileUnlinkArgs {
    key: String,
    shared_file_id: u64,
    json: bool,
}

impl IssueSharedFileUnlinkArgs {
    pub fn new(key: String, shared_file_id: u64, json: bool) -> Self {
        Self {
            key,
            shared_file_id,
            json,
        }
    }
}

pub fn unlink(args: &IssueSharedFileUnlinkArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    unlink_with(args, &client)
}

pub fn unlink_with(args: &IssueSharedFileUnlinkArgs, api: &dyn BacklogApi) -> Result<()> {
    let file = api.unlink_issue_shared_file(&args.key, args.shared_file_id)?;
    if args.json {
        crate::cmd::print_json(&file)?;
    } else {
        println!("Unlinked: {}", file.name);
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
        file: Option<IssueSharedFile>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn unlink_issue_shared_file(
            &self,
            _key: &str,
            _shared_file_id: u64,
        ) -> anyhow::Result<IssueSharedFile> {
            self.file.clone().ok_or_else(|| anyhow!("no file"))
        }
    }

    fn args(json: bool) -> IssueSharedFileUnlinkArgs {
        IssueSharedFileUnlinkArgs::new("TEST-1".to_string(), 1, json)
    }

    #[test]
    fn unlink_with_text_output_succeeds() {
        let api = MockApi {
            file: Some(sample_shared_file()),
        };
        assert!(unlink_with(&args(false), &api).is_ok());
    }

    #[test]
    fn unlink_with_json_output_succeeds() {
        let api = MockApi {
            file: Some(sample_shared_file()),
        };
        assert!(unlink_with(&args(true), &api).is_ok());
    }

    #[test]
    fn unlink_with_propagates_api_error() {
        let api = MockApi { file: None };
        let err = unlink_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no file"));
    }
}
