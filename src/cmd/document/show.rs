use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, document::Document};

pub struct DocumentShowArgs {
    document_id: String,
    json: bool,
}

impl DocumentShowArgs {
    pub fn new(document_id: String, json: bool) -> Self {
        Self { document_id, json }
    }
}

pub fn show(args: &DocumentShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &DocumentShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let doc = api.get_document(&args.document_id)?;
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

pub fn print_document_detail(doc: &Document) {
    println!("ID: {}", doc.id);
    println!("Title: {}", doc.title);
    println!("Project ID: {}", doc.project_id);
    if let Some(plain) = &doc.plain {
        println!("Content:\n{plain}");
    }
    println!("Created: {} ({})", doc.created_user.name, doc.created);
    println!("Updated: {} ({})", doc.updated_user.name, doc.updated);
    if !doc.tags.is_empty() {
        let tag_names: Vec<&str> = doc.tags.iter().map(|t| t.name.as_str()).collect();
        println!("Tags: {}", tag_names.join(", "));
    }
    if !doc.attachments.is_empty() {
        println!("Attachments: {}", doc.attachments.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::document::tests_helper::sample_document;
    use anyhow::anyhow;

    struct MockApi {
        doc: Option<Document>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_document(&self, _document_id: &str) -> anyhow::Result<Document> {
            self.doc.clone().ok_or_else(|| anyhow!("not found"))
        }
    }

    fn args(json: bool) -> DocumentShowArgs {
        DocumentShowArgs::new("abc123".to_string(), json)
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            doc: Some(sample_document()),
        };
        assert!(show_with(&args(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            doc: Some(sample_document()),
        };
        assert!(show_with(&args(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { doc: None };
        let err = show_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("not found"));
    }
}
