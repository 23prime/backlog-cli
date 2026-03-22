use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct PrAttachmentDeleteArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    attachment_id: u64,
    json: bool,
}

impl PrAttachmentDeleteArgs {
    pub fn new(
        project_id_or_key: String,
        repo_id_or_name: String,
        number: u64,
        attachment_id: u64,
        json: bool,
    ) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            number,
            attachment_id,
            json,
        }
    }
}

pub fn delete(args: &PrAttachmentDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &PrAttachmentDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let attachment = api.delete_pull_request_attachment(
        &args.project_id_or_key,
        &args.repo_id_or_name,
        args.number,
        args.attachment_id,
    )?;
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
    use crate::api::pull_request::PullRequestAttachment;
    use crate::cmd::pr::attachment::list::tests_helper::sample_pr_attachment;
    use anyhow::anyhow;

    struct MockApi {
        attachment: Option<PullRequestAttachment>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_pull_request_attachment(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
            _attachment_id: u64,
        ) -> anyhow::Result<PullRequestAttachment> {
            self.attachment
                .clone()
                .ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn args(json: bool) -> PrAttachmentDeleteArgs {
        PrAttachmentDeleteArgs::new("TEST".to_string(), "main".to_string(), 1, 1, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            attachment: Some(sample_pr_attachment()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            attachment: Some(sample_pr_attachment()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { attachment: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
