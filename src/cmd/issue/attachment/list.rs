use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueAttachmentListArgs {
    key: String,
    json: bool,
}

impl IssueAttachmentListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &IssueAttachmentListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &IssueAttachmentListArgs, api: &dyn BacklogApi) -> Result<()> {
    let attachments = api.get_issue_attachments(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&attachments).context("Failed to serialize JSON")?
        );
    } else {
        for a in &attachments {
            println!(
                "[{}] {} ({} bytes)",
                a.id.to_string().cyan().bold(),
                a.name,
                a.size
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::IssueAttachment;
    use crate::cmd::issue::attachment::sample_attachment;
    use anyhow::anyhow;

    struct MockApi {
        attachments: Option<Vec<IssueAttachment>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_issue_attachments(&self, _key: &str) -> anyhow::Result<Vec<IssueAttachment>> {
            self.attachments
                .clone()
                .ok_or_else(|| anyhow!("no attachments"))
        }
    }

    fn args(json: bool) -> IssueAttachmentListArgs {
        IssueAttachmentListArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_attachment()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_attachment()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { attachments: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no attachments"));
    }
}
