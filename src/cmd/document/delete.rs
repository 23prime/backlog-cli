use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::document::list::format_document_row;

pub struct DocumentDeleteArgs {
    document_id: String,
    json: bool,
}

impl DocumentDeleteArgs {
    pub fn new(document_id: String, json: bool) -> Self {
        Self { document_id, json }
    }
}

pub fn delete(args: &DocumentDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &DocumentDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let doc = api.delete_document(&args.document_id)?;
    if args.json {
        crate::cmd::print_json(&doc)?;
    } else {
        println!("Deleted: {}", format_document_row(&doc));
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
        fn delete_document(&self, _document_id: &str) -> anyhow::Result<Document> {
            self.doc.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn args(json: bool) -> DocumentDeleteArgs {
        DocumentDeleteArgs::new("abc123".to_string(), json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            doc: Some(sample_document()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            doc: Some(sample_document()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { doc: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
