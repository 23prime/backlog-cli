use anstream::println;
use anyhow::Result;

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
        crate::cmd::print_json(&attachment)?;
    } else {
        println!("Deleted: {} ({} bytes)", attachment.name, attachment.size);
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
