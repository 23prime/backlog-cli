use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient, document::Document};

pub struct DocumentListArgs {
    project_ids: Vec<u64>,
    keyword: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    count: Option<u32>,
    offset: Option<u64>,
    json: bool,
}

impl DocumentListArgs {
    pub fn new(
        project_ids: Vec<u64>,
        keyword: Option<String>,
        sort: Option<String>,
        order: Option<String>,
        count: Option<u32>,
        offset: Option<u64>,
        json: bool,
    ) -> Self {
        Self {
            project_ids,
            keyword,
            sort,
            order,
            count,
            offset,
            json,
        }
    }
}

pub fn list(args: &DocumentListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &DocumentListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = vec![];
    for id in &args.project_ids {
        params.push(("projectId[]".to_string(), id.to_string()));
    }
    if let Some(k) = &args.keyword {
        params.push(("keyword".to_string(), k.clone()));
    }
    if let Some(s) = &args.sort {
        params.push(("sort".to_string(), s.clone()));
    }
    if let Some(o) = &args.order {
        params.push(("order".to_string(), o.clone()));
    }
    params.push(("count".to_string(), args.count.unwrap_or(20).to_string()));
    params.push(("offset".to_string(), args.offset.unwrap_or(0).to_string()));
    let docs = api.get_documents(&params)?;
    if args.json {
        crate::cmd::print_json(&docs)?;
    } else {
        for doc in &docs {
            println!("{}", format_document_row(doc));
        }
    }
    Ok(())
}

pub fn format_document_row(doc: &Document) -> String {
    format!("[{}] {}", doc.id, doc.title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::document::tests_helper::sample_document;
    use anyhow::anyhow;

    struct MockApi {
        docs: Option<Vec<Document>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_documents(&self, _params: &[(String, String)]) -> anyhow::Result<Vec<Document>> {
            self.docs.clone().ok_or_else(|| anyhow!("list failed"))
        }
    }

    fn args(json: bool) -> DocumentListArgs {
        DocumentListArgs::new(vec![1], None, None, None, None, None, json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            docs: Some(vec![sample_document()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            docs: Some(vec![sample_document()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { docs: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("list failed"));
    }
}
