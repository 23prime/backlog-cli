use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct StarAddArgs {
    issue_id: Option<u64>,
    comment_id: Option<u64>,
    wiki_id: Option<u64>,
    pull_request_id: Option<u64>,
    pull_request_comment_id: Option<u64>,
}

impl StarAddArgs {
    pub fn try_new(
        issue_id: Option<u64>,
        comment_id: Option<u64>,
        wiki_id: Option<u64>,
        pull_request_id: Option<u64>,
        pull_request_comment_id: Option<u64>,
    ) -> anyhow::Result<Self> {
        let count = [
            issue_id.is_some(),
            comment_id.is_some(),
            wiki_id.is_some(),
            pull_request_id.is_some(),
            pull_request_comment_id.is_some(),
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        if count != 1 {
            anyhow::bail!(
                "exactly one of --issue-id, --comment-id, --wiki-id, \
                 --pull-request-id, --pull-request-comment-id must be specified"
            );
        }
        Ok(Self {
            issue_id,
            comment_id,
            wiki_id,
            pull_request_id,
            pull_request_comment_id,
        })
    }
}

pub fn add(args: &StarAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &StarAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(id) = args.issue_id {
        params.push(("issueId".to_string(), id.to_string()));
    }
    if let Some(id) = args.comment_id {
        params.push(("commentId".to_string(), id.to_string()));
    }
    if let Some(id) = args.wiki_id {
        params.push(("wikiId".to_string(), id.to_string()));
    }
    if let Some(id) = args.pull_request_id {
        params.push(("pullRequestId".to_string(), id.to_string()));
    }
    if let Some(id) = args.pull_request_comment_id {
        params.push(("pullRequestCommentId".to_string(), id.to_string()));
    }
    api.add_star(&params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        ok: bool,
    }

    impl crate::api::BacklogApi for MockApi {
        fn add_star(&self, _params: &[(String, String)]) -> anyhow::Result<()> {
            if self.ok {
                Ok(())
            } else {
                Err(anyhow!("api error"))
            }
        }
    }

    fn args_with_issue() -> StarAddArgs {
        StarAddArgs::try_new(Some(1), None, None, None, None).unwrap()
    }

    #[test]
    fn add_with_succeeds() {
        let api = MockApi { ok: true };
        assert!(add_with(&args_with_issue(), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi { ok: false };
        let err = add_with(&args_with_issue(), &api).unwrap_err();
        assert!(err.to_string().contains("api error"));
    }

    #[test]
    fn try_new_rejects_no_target() {
        assert!(StarAddArgs::try_new(None, None, None, None, None).is_err());
    }

    #[test]
    fn try_new_rejects_multiple_targets() {
        assert!(StarAddArgs::try_new(Some(1), Some(2), None, None, None).is_err());
    }

    #[test]
    fn try_new_accepts_each_target() {
        assert!(StarAddArgs::try_new(Some(1), None, None, None, None).is_ok());
        assert!(StarAddArgs::try_new(None, Some(1), None, None, None).is_ok());
        assert!(StarAddArgs::try_new(None, None, Some(1), None, None).is_ok());
        assert!(StarAddArgs::try_new(None, None, None, Some(1), None).is_ok());
        assert!(StarAddArgs::try_new(None, None, None, None, Some(1)).is_ok());
    }
}
