use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::document::show::print_document_detail;

pub struct DocumentCreateArgs {
    project_id: u64,
    title: Option<String>,
    content: Option<String>,
    emoji: Option<String>,
    parent_id: Option<String>,
    add_last: bool,
    json: bool,
}

impl DocumentCreateArgs {
    pub fn new(
        project_id: u64,
        title: Option<String>,
        content: Option<String>,
        emoji: Option<String>,
        parent_id: Option<String>,
        add_last: bool,
        json: bool,
    ) -> Self {
        Self {
            project_id,
            title,
            content,
            emoji,
            parent_id,
            add_last,
            json,
        }
    }
}

pub fn create(args: &DocumentCreateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    create_with(args, &client)
}

pub fn create_with(args: &DocumentCreateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> =
        vec![("projectId".to_string(), args.project_id.to_string())];
    if let Some(t) = &args.title {
        params.push(("title".to_string(), t.clone()));
    }
    if let Some(c) = &args.content {
        params.push(("content".to_string(), c.clone()));
    }
    if let Some(e) = &args.emoji {
        params.push(("emoji".to_string(), e.clone()));
    }
    if let Some(p) = &args.parent_id {
        params.push(("parentId".to_string(), p.clone()));
    }
    if args.add_last {
        params.push(("addLast".to_string(), "true".to_string()));
    }
    let doc = api.create_document(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&doc).context("Failed to serialize JSON")?
        );
    } else {
        print_document_detail(&doc);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::document::{Document, tests_helper::sample_document};
    use anyhow::anyhow;

    struct MockApi {
        doc: Option<Document>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn create_document(&self, _params: &[(String, String)]) -> anyhow::Result<Document> {
            self.doc.clone().ok_or_else(|| anyhow!("create failed"))
        }
    }

    fn args(json: bool) -> DocumentCreateArgs {
        DocumentCreateArgs::new(
            1,
            Some("New Doc".to_string()),
            None,
            None,
            None,
            false,
            json,
        )
    }

    #[test]
    fn create_with_text_output_succeeds() {
        let api = MockApi {
            doc: Some(sample_document()),
        };
        assert!(create_with(&args(false), &api).is_ok());
    }

    #[test]
    fn create_with_json_output_succeeds() {
        let api = MockApi {
            doc: Some(sample_document()),
        };
        assert!(create_with(&args(true), &api).is_ok());
    }

    #[test]
    fn create_with_propagates_api_error() {
        let api = MockApi { doc: None };
        let err = create_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("create failed"));
    }
}
