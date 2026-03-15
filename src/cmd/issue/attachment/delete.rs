use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueAttachmentDeleteArgs {
    key: String,
    attachment_id: u64,
    json: bool,
}

impl IssueAttachmentDeleteArgs {
    pub fn new(key: String, attachment_id: u64, json: bool) -> Self {
        Self {
            key,
            attachment_id,
            json,
        }
    }
}

pub fn delete(args: &IssueAttachmentDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &IssueAttachmentDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let attachment = api.delete_issue_attachment(&args.key, args.attachment_id)?;
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
    use crate::api::issue::{IssueAttachment, IssueUser};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    fn sample_attachment() -> IssueAttachment {
        IssueAttachment {
            id: 1,
            name: "file.txt".to_string(),
            size: 1024,
            created_user: IssueUser {
                id: 1,
                user_id: Some("john".to_string()),
                name: "John Doe".to_string(),
                role_type: 1,
                lang: None,
                mail_address: None,
                extra: BTreeMap::new(),
            },
            created: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    struct MockApi {
        attachment: Option<IssueAttachment>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_issue_attachment(
            &self,
            _key: &str,
            _attachment_id: u64,
        ) -> anyhow::Result<IssueAttachment> {
            self.attachment
                .clone()
                .ok_or_else(|| anyhow!("no attachment"))
        }
    }

    fn args(json: bool) -> IssueAttachmentDeleteArgs {
        IssueAttachmentDeleteArgs::new("TEST-1".to_string(), 1, json)
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
