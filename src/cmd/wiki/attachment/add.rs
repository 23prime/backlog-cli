use anstream::println;
use anyhow::{Result, bail};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::wiki::attachment::list::format_attachment_row;

#[derive(Debug)]
pub struct WikiAttachmentAddArgs {
    wiki_id: u64,
    attachment_ids: Vec<u64>,
    json: bool,
}

impl WikiAttachmentAddArgs {
    pub fn try_new(wiki_id: u64, attachment_ids: Vec<u64>, json: bool) -> Result<Self> {
        if attachment_ids.is_empty() {
            bail!("at least one --attachment-id must be specified");
        }
        Ok(Self {
            wiki_id,
            attachment_ids,
            json,
        })
    }
}

pub fn add(args: &WikiAttachmentAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &WikiAttachmentAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let attachments = api.add_wiki_attachments(args.wiki_id, &args.attachment_ids)?;
    if args.json {
        crate::cmd::print_json(&attachments)?;
    } else {
        for a in &attachments {
            println!("{}", format_attachment_row(a));
        }
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
        attachments: Option<Vec<WikiAttachment>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn add_wiki_attachments(
            &self,
            _wiki_id: u64,
            _attachment_ids: &[u64],
        ) -> anyhow::Result<Vec<WikiAttachment>> {
            self.attachments
                .clone()
                .ok_or_else(|| anyhow!("no attachments"))
        }
    }

    fn args(json: bool) -> WikiAttachmentAddArgs {
        WikiAttachmentAddArgs::try_new(1, vec![1], json).unwrap()
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_attachment()]),
        };
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_attachment()]),
        };
        assert!(add_with(&args(true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi { attachments: None };
        let err = add_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no attachments"));
    }

    #[test]
    fn try_new_rejects_empty_ids() {
        let err = WikiAttachmentAddArgs::try_new(1, vec![], false).unwrap_err();
        assert!(err.to_string().contains("at least one"));
    }
}
