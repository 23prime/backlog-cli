use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct PrAttachmentListArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    json: bool,
}

impl PrAttachmentListArgs {
    pub fn new(
        project_id_or_key: String,
        repo_id_or_name: String,
        number: u64,
        json: bool,
    ) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            number,
            json,
        }
    }
}

pub fn list(args: &PrAttachmentListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &PrAttachmentListArgs, api: &dyn BacklogApi) -> Result<()> {
    let attachments = api.get_pull_request_attachments(
        &args.project_id_or_key,
        &args.repo_id_or_name,
        args.number,
    )?;
    if args.json {
        crate::cmd::print_json(&attachments)?;
    } else {
        for a in &attachments {
            println!("[{}] {} ({} bytes)", a.id, a.name, a.size);
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests_helper {
    use crate::api::pull_request::PullRequestAttachment;
    use crate::cmd::pr::list::tests_helper::sample_pr_user;

    pub fn sample_pr_attachment() -> PullRequestAttachment {
        PullRequestAttachment {
            id: 1,
            name: "screenshot.png".to_string(),
            size: 1024,
            created_user: sample_pr_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::pull_request::PullRequestAttachment;
    use anyhow::anyhow;
    use tests_helper::sample_pr_attachment;

    struct MockApi {
        attachments: Option<Vec<PullRequestAttachment>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_pull_request_attachments(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
        ) -> anyhow::Result<Vec<PullRequestAttachment>> {
            self.attachments
                .clone()
                .ok_or_else(|| anyhow!("no attachments"))
        }
    }

    fn args(json: bool) -> PrAttachmentListArgs {
        PrAttachmentListArgs::new("TEST".to_string(), "main".to_string(), 1, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_pr_attachment()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            attachments: Some(vec![sample_pr_attachment()]),
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
