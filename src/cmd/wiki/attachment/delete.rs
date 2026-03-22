use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WikiAttachmentDeleteArgs {
    wiki_id: u64,
    attachment_id: u64,
    json: bool,
}

impl WikiAttachmentDeleteArgs {
    pub fn new(wiki_id: u64, attachment_id: u64, json: bool) -> Self {
        Self {
            wiki_id,
            attachment_id,
            json,
        }
    }
}

pub fn delete(args: &WikiAttachmentDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &WikiAttachmentDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let attachment = api.delete_wiki_attachment(args.wiki_id, args.attachment_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&attachment).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {} ({} bytes)", attachment.name, attachment.size);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::WikiAttachment;
    use crate::cmd::wiki::attachment::sample_attachment;

    use anyhow::anyhow;

    struct MockApi {
        attachment: Option<WikiAttachment>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_wiki_attachment(
            &self,
            _wiki_id: u64,
            _attachment_id: u64,
        ) -> anyhow::Result<WikiAttachment> {
            self.attachment
                .clone()
                .ok_or_else(|| anyhow!("no attachment"))
        }
    }

    fn args(json: bool) -> WikiAttachmentDeleteArgs {
        WikiAttachmentDeleteArgs::new(1, 1, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            attachment: Some(sample_attachment()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            attachment: Some(sample_attachment()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { attachment: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no attachment"));
    }
}
