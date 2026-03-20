use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, project::ProjectCustomField};

pub struct ProjectCustomFieldListArgs {
    key: String,
    json: bool,
}

impl ProjectCustomFieldListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub struct ProjectCustomFieldAddArgs {
    key: String,
    type_id: u64,
    name: String,
    description: Option<String>,
    required: Option<bool>,
    json: bool,
}

impl ProjectCustomFieldAddArgs {
    pub fn new(
        key: String,
        type_id: u64,
        name: String,
        description: Option<String>,
        required: Option<bool>,
        json: bool,
    ) -> Self {
        Self {
            key,
            type_id,
            name,
            description,
            required,
            json,
        }
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectCustomFieldUpdateArgs {
    key: String,
    custom_field_id: u64,
    name: Option<String>,
    description: Option<String>,
    required: Option<bool>,
    json: bool,
}

impl ProjectCustomFieldUpdateArgs {
    pub fn try_new(
        key: String,
        custom_field_id: u64,
        name: Option<String>,
        description: Option<String>,
        required: Option<bool>,
        json: bool,
    ) -> Result<Self> {
        if name.is_none() && description.is_none() && required.is_none() {
            return Err(anyhow::anyhow!(
                "At least one of --name, --description, or --required must be specified"
            ));
        }
        Ok(Self {
            key,
            custom_field_id,
            name,
            description,
            required,
            json,
        })
    }
}

pub struct ProjectCustomFieldDeleteArgs {
    key: String,
    custom_field_id: u64,
    json: bool,
}

impl ProjectCustomFieldDeleteArgs {
    pub fn new(key: String, custom_field_id: u64, json: bool) -> Self {
        Self {
            key,
            custom_field_id,
            json,
        }
    }
}

pub struct ProjectCustomFieldItemAddArgs {
    key: String,
    custom_field_id: u64,
    name: String,
    json: bool,
}

impl ProjectCustomFieldItemAddArgs {
    pub fn new(key: String, custom_field_id: u64, name: String, json: bool) -> Self {
        Self {
            key,
            custom_field_id,
            name,
            json,
        }
    }
}

pub struct ProjectCustomFieldItemUpdateArgs {
    key: String,
    custom_field_id: u64,
    item_id: u64,
    name: String,
    json: bool,
}

impl ProjectCustomFieldItemUpdateArgs {
    pub fn new(key: String, custom_field_id: u64, item_id: u64, name: String, json: bool) -> Self {
        Self {
            key,
            custom_field_id,
            item_id,
            name,
            json,
        }
    }
}

pub struct ProjectCustomFieldItemDeleteArgs {
    key: String,
    custom_field_id: u64,
    item_id: u64,
    json: bool,
}

impl ProjectCustomFieldItemDeleteArgs {
    pub fn new(key: String, custom_field_id: u64, item_id: u64, json: bool) -> Self {
        Self {
            key,
            custom_field_id,
            item_id,
            json,
        }
    }
}

pub fn list(args: &ProjectCustomFieldListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectCustomFieldListArgs, api: &dyn BacklogApi) -> Result<()> {
    let fields = api.get_project_custom_fields(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&fields).context("Failed to serialize JSON")?
        );
    } else {
        for f in &fields {
            println!("{}", format_custom_field_row(f));
        }
    }
    Ok(())
}

pub fn add(args: &ProjectCustomFieldAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectCustomFieldAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let field = api.add_project_custom_field(
        &args.key,
        args.type_id,
        &args.name,
        args.description.as_deref(),
        args.required,
    )?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&field).context("Failed to serialize JSON")?
        );
    } else {
        println!("Added: {}", format_custom_field_row(&field));
    }
    Ok(())
}

pub fn update(args: &ProjectCustomFieldUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &ProjectCustomFieldUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let field = api.update_project_custom_field(
        &args.key,
        args.custom_field_id,
        args.name.as_deref(),
        args.description.as_deref(),
        args.required,
    )?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&field).context("Failed to serialize JSON")?
        );
    } else {
        println!("Updated: {}", format_custom_field_row(&field));
    }
    Ok(())
}

pub fn delete(args: &ProjectCustomFieldDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectCustomFieldDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let field = api.delete_project_custom_field(&args.key, args.custom_field_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&field).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", format_custom_field_row(&field));
    }
    Ok(())
}

pub fn item_add(args: &ProjectCustomFieldItemAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    item_add_with(args, &client)
}

pub fn item_add_with(args: &ProjectCustomFieldItemAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let field = api.add_project_custom_field_item(&args.key, args.custom_field_id, &args.name)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&field).context("Failed to serialize JSON")?
        );
    } else {
        println!("Added item to: {}", format_custom_field_row(&field));
    }
    Ok(())
}

pub fn item_update(args: &ProjectCustomFieldItemUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    item_update_with(args, &client)
}

pub fn item_update_with(
    args: &ProjectCustomFieldItemUpdateArgs,
    api: &dyn BacklogApi,
) -> Result<()> {
    let field = api.update_project_custom_field_item(
        &args.key,
        args.custom_field_id,
        args.item_id,
        &args.name,
    )?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&field).context("Failed to serialize JSON")?
        );
    } else {
        println!("Updated item in: {}", format_custom_field_row(&field));
    }
    Ok(())
}

pub fn item_delete(args: &ProjectCustomFieldItemDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    item_delete_with(args, &client)
}

pub fn item_delete_with(
    args: &ProjectCustomFieldItemDeleteArgs,
    api: &dyn BacklogApi,
) -> Result<()> {
    let field =
        api.delete_project_custom_field_item(&args.key, args.custom_field_id, args.item_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&field).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted item from: {}", format_custom_field_row(&field));
    }
    Ok(())
}

fn format_custom_field_row(f: &ProjectCustomField) -> String {
    let required = if f.required { " [required]" } else { "" };
    format!("[{}] {} (type:{}){}", f.id, f.name, f.type_id, required)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        list: Option<Vec<ProjectCustomField>>,
        single: Option<ProjectCustomField>,
    }

    fn mock(list: Option<Vec<ProjectCustomField>>, single: Option<ProjectCustomField>) -> MockApi {
        MockApi { list, single }
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_custom_fields(&self, _key: &str) -> anyhow::Result<Vec<ProjectCustomField>> {
            self.list.clone().ok_or_else(|| anyhow!("list failed"))
        }

        fn add_project_custom_field(
            &self,
            _key: &str,
            _type_id: u64,
            _name: &str,
            _description: Option<&str>,
            _required: Option<bool>,
        ) -> anyhow::Result<ProjectCustomField> {
            self.single.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn update_project_custom_field(
            &self,
            _key: &str,
            _custom_field_id: u64,
            _name: Option<&str>,
            _description: Option<&str>,
            _required: Option<bool>,
        ) -> anyhow::Result<ProjectCustomField> {
            self.single.clone().ok_or_else(|| anyhow!("update failed"))
        }

        fn delete_project_custom_field(
            &self,
            _key: &str,
            _custom_field_id: u64,
        ) -> anyhow::Result<ProjectCustomField> {
            self.single.clone().ok_or_else(|| anyhow!("delete failed"))
        }

        fn add_project_custom_field_item(
            &self,
            _key: &str,
            _custom_field_id: u64,
            _name: &str,
        ) -> anyhow::Result<ProjectCustomField> {
            self.single
                .clone()
                .ok_or_else(|| anyhow!("item add failed"))
        }

        fn update_project_custom_field_item(
            &self,
            _key: &str,
            _custom_field_id: u64,
            _item_id: u64,
            _name: &str,
        ) -> anyhow::Result<ProjectCustomField> {
            self.single
                .clone()
                .ok_or_else(|| anyhow!("item update failed"))
        }

        fn delete_project_custom_field_item(
            &self,
            _key: &str,
            _custom_field_id: u64,
            _item_id: u64,
        ) -> anyhow::Result<ProjectCustomField> {
            self.single
                .clone()
                .ok_or_else(|| anyhow!("item delete failed"))
        }
    }

    fn sample_field() -> ProjectCustomField {
        ProjectCustomField {
            id: 1,
            type_id: 6,
            name: "Priority".to_string(),
            description: None,
            required: false,
            applicable_issue_types: vec![],
            allow_add_item: Some(true),
            items: Some(vec![]),
        }
    }

    #[test]
    fn format_custom_field_row_contains_fields() {
        let text = format_custom_field_row(&sample_field());
        assert!(text.contains("[1]"));
        assert!(text.contains("Priority"));
        assert!(text.contains("type:6"));
        assert!(!text.contains("[required]"));
    }

    #[test]
    fn format_custom_field_row_required() {
        let f = ProjectCustomField {
            required: true,
            ..sample_field()
        };
        let text = format_custom_field_row(&f);
        assert!(text.contains("[required]"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = mock(Some(vec![sample_field()]), None);
        assert!(
            list_with(
                &ProjectCustomFieldListArgs::new("TEST".to_string(), false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = mock(Some(vec![sample_field()]), None);
        assert!(
            list_with(
                &ProjectCustomFieldListArgs::new("TEST".to_string(), true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = mock(None, None);
        let err = list_with(
            &ProjectCustomFieldListArgs::new("TEST".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("list failed"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldAddArgs::new(
            "TEST".to_string(),
            6,
            "Priority".to_string(),
            None,
            None,
            false,
        );
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldAddArgs::new(
            "TEST".to_string(),
            6,
            "Priority".to_string(),
            None,
            None,
            true,
        );
        assert!(add_with(&args, &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectCustomFieldAddArgs::new(
            "TEST".to_string(),
            6,
            "Priority".to_string(),
            None,
            None,
            false,
        );
        let err = add_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn update_try_new_requires_at_least_one_field() {
        let err =
            ProjectCustomFieldUpdateArgs::try_new("TEST".to_string(), 1, None, None, None, false)
                .unwrap_err();
        assert!(err.to_string().contains("At least one"));
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("NewName".to_string()),
            None,
            None,
            false,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("NewName".to_string()),
            None,
            None,
            true,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectCustomFieldUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("NewName".to_string()),
            None,
            None,
            false,
        )
        .unwrap();
        let err = update_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldDeleteArgs::new("TEST".to_string(), 1, false);
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldDeleteArgs::new("TEST".to_string(), 1, true);
        assert!(delete_with(&args, &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectCustomFieldDeleteArgs::new("TEST".to_string(), 1, false);
        let err = delete_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }

    #[test]
    fn item_add_with_text_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args =
            ProjectCustomFieldItemAddArgs::new("TEST".to_string(), 1, "item1".to_string(), false);
        assert!(item_add_with(&args, &api).is_ok());
    }

    #[test]
    fn item_add_with_json_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args =
            ProjectCustomFieldItemAddArgs::new("TEST".to_string(), 1, "item1".to_string(), true);
        assert!(item_add_with(&args, &api).is_ok());
    }

    #[test]
    fn item_add_with_propagates_api_error() {
        let api = mock(None, None);
        let args =
            ProjectCustomFieldItemAddArgs::new("TEST".to_string(), 1, "item1".to_string(), false);
        let err = item_add_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("item add failed"));
    }

    #[test]
    fn item_update_with_text_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldItemUpdateArgs::new(
            "TEST".to_string(),
            1,
            10,
            "item1-new".to_string(),
            false,
        );
        assert!(item_update_with(&args, &api).is_ok());
    }

    #[test]
    fn item_update_with_json_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldItemUpdateArgs::new(
            "TEST".to_string(),
            1,
            10,
            "item1-new".to_string(),
            true,
        );
        assert!(item_update_with(&args, &api).is_ok());
    }

    #[test]
    fn item_update_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectCustomFieldItemUpdateArgs::new(
            "TEST".to_string(),
            1,
            10,
            "item1-new".to_string(),
            false,
        );
        let err = item_update_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("item update failed"));
    }

    #[test]
    fn item_delete_with_text_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldItemDeleteArgs::new("TEST".to_string(), 1, 10, false);
        assert!(item_delete_with(&args, &api).is_ok());
    }

    #[test]
    fn item_delete_with_json_output_succeeds() {
        let api = mock(None, Some(sample_field()));
        let args = ProjectCustomFieldItemDeleteArgs::new("TEST".to_string(), 1, 10, true);
        assert!(item_delete_with(&args, &api).is_ok());
    }

    #[test]
    fn item_delete_with_propagates_api_error() {
        let api = mock(None, None);
        let args = ProjectCustomFieldItemDeleteArgs::new("TEST".to_string(), 1, 10, false);
        let err = item_delete_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("item delete failed"));
    }
}
