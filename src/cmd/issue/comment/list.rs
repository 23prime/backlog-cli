use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, issue::IssueComment};

pub struct IssueCommentListArgs {
    key: String,
    json: bool,
}

impl IssueCommentListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &IssueCommentListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &IssueCommentListArgs, api: &dyn BacklogApi) -> Result<()> {
    let comments = api.get_issue_comments(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&comments).context("Failed to serialize JSON")?
        );
    } else {
        for c in &comments {
            println!("{}", format_comment_row(c));
        }
    }
    Ok(())
}

pub fn format_comment_row(c: &IssueComment) -> String {
    let content = c.content.as_deref().unwrap_or("(no content)");
    format!(
        "[{}] {} ({}): {}",
        c.id.to_string().cyan().bold(),
        c.created_user.name,
        c.created,
        content
    )
}

#[cfg(test)]
use crate::api::issue::IssueUser;
#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
fn sample_user() -> IssueUser {
    IssueUser {
        id: 1,
        user_id: Some("john".to_string()),
        name: "John Doe".to_string(),
        role_type: 1,
        lang: None,
        mail_address: None,
        extra: BTreeMap::new(),
    }
}

#[cfg(test)]
pub(crate) fn sample_comment() -> crate::api::issue::IssueComment {
    use crate::api::issue::IssueComment;
    IssueComment {
        id: 1,
        content: Some("A comment".to_string()),
        created_user: sample_user(),
        created: "2024-01-01T00:00:00Z".to_string(),
        updated: "2024-01-01T00:00:00Z".to_string(),
        extra: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::IssueComment;
    use anyhow::anyhow;

    struct MockApi {
        comments: Option<Vec<IssueComment>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_issue_comments(&self, _key: &str) -> anyhow::Result<Vec<IssueComment>> {
            self.comments.clone().ok_or_else(|| anyhow!("no comments"))
        }
    }

    fn args(json: bool) -> IssueCommentListArgs {
        IssueCommentListArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            comments: Some(vec![sample_comment()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            comments: Some(vec![sample_comment()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { comments: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no comments"));
    }

    #[test]
    fn format_comment_row_with_content() {
        let row = format_comment_row(&sample_comment());
        assert!(row.contains('1'));
        assert!(row.contains("John Doe"));
        assert!(row.contains("A comment"));
    }

    #[test]
    fn format_comment_row_null_content() {
        let mut c = sample_comment();
        c.content = None;
        let row = format_comment_row(&c);
        assert!(row.contains("(no content)"));
    }
}
