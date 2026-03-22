use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct DocumentTreeArgs {
    project_id_or_key: String,
    json: bool,
}

impl DocumentTreeArgs {
    pub fn new(project_id_or_key: String, json: bool) -> Self {
        Self {
            project_id_or_key,
            json,
        }
    }
}

pub fn tree(args: &DocumentTreeArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    tree_with(args, &client)
}

pub fn tree_with(args: &DocumentTreeArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![("projectIdOrKey".to_string(), args.project_id_or_key.clone())];
    let t = api.get_document_tree(&params)?;
    if args.json {
        crate::cmd::print_json(&t)?;
    } else {
        println!("Project: {}", t.project_id);
        println!("Active:");
        print_tree_node(&t.active_tree, 0);
        println!("Trash:");
        print_tree_node(&t.trash_tree, 0);
    }
    Ok(())
}

fn print_tree_node(node: &crate::api::document::DocumentTreeNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let name = node.name.as_deref().unwrap_or("(root)");
    let emoji = node
        .emoji
        .as_deref()
        .map(|e| format!("{e} "))
        .unwrap_or_default();
    println!("{indent}{emoji}{name} [{}]", node.id);
    for child in &node.children {
        print_tree_node(child, depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::document::{DocumentTree, DocumentTreeNode};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        tree: Option<DocumentTree>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_document_tree(&self, _params: &[(String, String)]) -> anyhow::Result<DocumentTree> {
            self.tree.clone().ok_or_else(|| anyhow!("tree failed"))
        }
    }

    fn sample_tree() -> DocumentTree {
        DocumentTree {
            project_id: 1,
            active_tree: DocumentTreeNode {
                id: "root".to_string(),
                name: None,
                emoji: None,
                children: vec![DocumentTreeNode {
                    id: "child1".to_string(),
                    name: Some("Chapter 1".to_string()),
                    emoji: Some("📖".to_string()),
                    children: vec![],
                    extra: BTreeMap::new(),
                }],
                extra: BTreeMap::new(),
            },
            trash_tree: DocumentTreeNode {
                id: "trash".to_string(),
                name: None,
                emoji: None,
                children: vec![],
                extra: BTreeMap::new(),
            },
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> DocumentTreeArgs {
        DocumentTreeArgs::new("TEST".to_string(), json)
    }

    #[test]
    fn tree_with_text_output_succeeds() {
        let api = MockApi {
            tree: Some(sample_tree()),
        };
        assert!(tree_with(&args(false), &api).is_ok());
    }

    #[test]
    fn tree_with_json_output_succeeds() {
        let api = MockApi {
            tree: Some(sample_tree()),
        };
        assert!(tree_with(&args(true), &api).is_ok());
    }

    #[test]
    fn tree_with_propagates_api_error() {
        let api = MockApi { tree: None };
        let err = tree_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("tree failed"));
    }
}
