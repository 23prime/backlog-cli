use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct WikiSharedFileUnlinkArgs {
    wiki_id: u64,
    shared_file_id: u64,
    json: bool,
}

impl WikiSharedFileUnlinkArgs {
    pub fn new(wiki_id: u64, shared_file_id: u64, json: bool) -> Self {
        Self {
            wiki_id,
            shared_file_id,
            json,
        }
    }
}

pub fn unlink(args: &WikiSharedFileUnlinkArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    unlink_with(args, &client)
}

pub fn unlink_with(args: &WikiSharedFileUnlinkArgs, api: &dyn BacklogApi) -> Result<()> {
    let file = api.unlink_wiki_shared_file(args.wiki_id, args.shared_file_id)?;
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
    use crate::api::wiki::WikiSharedFile;
    use crate::cmd::wiki::shared_file::list::sample_wiki_shared_file;
    use anyhow::anyhow;

    struct MockApi {
        file: Option<WikiSharedFile>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn unlink_wiki_shared_file(
            &self,
            _wiki_id: u64,
            _shared_file_id: u64,
        ) -> anyhow::Result<WikiSharedFile> {
            self.file.clone().ok_or_else(|| anyhow!("no file"))
        }
    }

    fn args(json: bool) -> WikiSharedFileUnlinkArgs {
        WikiSharedFileUnlinkArgs::new(1, 1, json)
    }

    #[test]
    fn unlink_with_text_output_succeeds() {
        let api = MockApi {
            file: Some(sample_wiki_shared_file()),
        };
        assert!(unlink_with(&args(false), &api).is_ok());
    }

    #[test]
    fn unlink_with_json_output_succeeds() {
        let api = MockApi {
            file: Some(sample_wiki_shared_file()),
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
