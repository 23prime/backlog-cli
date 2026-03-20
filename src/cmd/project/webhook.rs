use anstream::println;
use anyhow::{Context, Result, bail};

use crate::api::{BacklogApi, BacklogClient, project::ProjectWebhook};

pub struct ProjectWebhookListArgs {
    key: String,
    json: bool,
}

impl ProjectWebhookListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &ProjectWebhookListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectWebhookListArgs, api: &dyn BacklogApi) -> Result<()> {
    let hooks = api.get_project_webhooks(&args.key)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&hooks).context("Failed to serialize JSON")?
        );
    } else {
        for h in &hooks {
            println!("{}", format_webhook_row(h));
        }
    }
    Ok(())
}

pub struct ProjectWebhookShowArgs {
    key: String,
    webhook_id: u64,
    json: bool,
}

impl ProjectWebhookShowArgs {
    pub fn new(key: String, webhook_id: u64, json: bool) -> Self {
        Self {
            key,
            webhook_id,
            json,
        }
    }
}

pub fn show(args: &ProjectWebhookShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &ProjectWebhookShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let hook = api.get_project_webhook(&args.key, args.webhook_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&hook).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_webhook_detail(&hook));
    }
    Ok(())
}

pub struct ProjectWebhookAddArgs {
    key: String,
    name: String,
    hook_url: String,
    description: Option<String>,
    all_event: Option<bool>,
    activity_type_ids: Vec<u64>,
    json: bool,
}

impl ProjectWebhookAddArgs {
    pub fn new(
        key: String,
        name: String,
        hook_url: String,
        description: Option<String>,
        all_event: Option<bool>,
        activity_type_ids: Vec<u64>,
        json: bool,
    ) -> Self {
        Self {
            key,
            name,
            hook_url,
            description,
            all_event,
            activity_type_ids,
            json,
        }
    }
}

pub fn add(args: &ProjectWebhookAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectWebhookAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let hook = api.add_project_webhook(
        &args.key,
        &args.name,
        &args.hook_url,
        args.description.as_deref(),
        args.all_event,
        &args.activity_type_ids,
    )?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&hook).context("Failed to serialize JSON")?
        );
    } else {
        println!("Added: {}", format_webhook_row(&hook));
    }
    Ok(())
}

#[cfg_attr(test, derive(Debug))]
pub struct ProjectWebhookUpdateArgs {
    key: String,
    webhook_id: u64,
    name: Option<String>,
    hook_url: Option<String>,
    description: Option<String>,
    all_event: Option<bool>,
    activity_type_ids: Option<Vec<u64>>,
    json: bool,
}

impl ProjectWebhookUpdateArgs {
    pub fn try_new(
        key: String,
        webhook_id: u64,
        name: Option<String>,
        hook_url: Option<String>,
        description: Option<String>,
        all_event: Option<bool>,
        activity_type_ids: Option<Vec<u64>>,
        json: bool,
    ) -> Result<Self> {
        if name.is_none()
            && hook_url.is_none()
            && description.is_none()
            && all_event.is_none()
            && activity_type_ids.is_none()
        {
            bail!(
                "At least one of --name, --hook-url, --description, --all-event, or --activity-type-id must be provided"
            );
        }
        Ok(Self {
            key,
            webhook_id,
            name,
            hook_url,
            description,
            all_event,
            activity_type_ids,
            json,
        })
    }
}

pub fn update(args: &ProjectWebhookUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &ProjectWebhookUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let hook = api.update_project_webhook(
        &args.key,
        args.webhook_id,
        args.name.as_deref(),
        args.hook_url.as_deref(),
        args.description.as_deref(),
        args.all_event,
        args.activity_type_ids.as_deref(),
    )?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&hook).context("Failed to serialize JSON")?
        );
    } else {
        println!("Updated: {}", format_webhook_row(&hook));
    }
    Ok(())
}

pub struct ProjectWebhookDeleteArgs {
    key: String,
    webhook_id: u64,
    json: bool,
}

impl ProjectWebhookDeleteArgs {
    pub fn new(key: String, webhook_id: u64, json: bool) -> Self {
        Self {
            key,
            webhook_id,
            json,
        }
    }
}

pub fn delete(args: &ProjectWebhookDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectWebhookDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let hook = api.delete_project_webhook(&args.key, args.webhook_id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&hook).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", format_webhook_row(&hook));
    }
    Ok(())
}

fn format_webhook_row(h: &ProjectWebhook) -> String {
    format!("[{}] {} ({})", h.id, h.name, h.hook_url)
}

fn format_webhook_detail(h: &ProjectWebhook) -> String {
    let events = if h.all_event {
        "all".to_string()
    } else {
        h.activity_type_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "ID: {}\nName: {}\nURL: {}\nDescription: {}\nEvents: {}",
        h.id, h.name, h.hook_url, h.description, events
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    use crate::api::project::WebhookUser;

    struct MockApi {
        hooks: Option<Vec<ProjectWebhook>>,
        hook: Option<ProjectWebhook>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_webhooks(&self, _key: &str) -> anyhow::Result<Vec<ProjectWebhook>> {
            self.hooks.clone().ok_or_else(|| anyhow!("no hooks"))
        }

        fn get_project_webhook(
            &self,
            _key: &str,
            _webhook_id: u64,
        ) -> anyhow::Result<ProjectWebhook> {
            self.hook.clone().ok_or_else(|| anyhow!("no hook"))
        }

        fn add_project_webhook(
            &self,
            _key: &str,
            _name: &str,
            _hook_url: &str,
            _description: Option<&str>,
            _all_event: Option<bool>,
            _activity_type_ids: &[u64],
        ) -> anyhow::Result<ProjectWebhook> {
            self.hook.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn update_project_webhook(
            &self,
            _key: &str,
            _webhook_id: u64,
            _name: Option<&str>,
            _hook_url: Option<&str>,
            _description: Option<&str>,
            _all_event: Option<bool>,
            _activity_type_ids: Option<&[u64]>,
        ) -> anyhow::Result<ProjectWebhook> {
            self.hook.clone().ok_or_else(|| anyhow!("update failed"))
        }

        fn delete_project_webhook(
            &self,
            _key: &str,
            _webhook_id: u64,
        ) -> anyhow::Result<ProjectWebhook> {
            self.hook.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_user() -> WebhookUser {
        WebhookUser {
            id: 1,
            user_id: Some("admin".to_string()),
            name: "Admin".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    fn sample_hook() -> ProjectWebhook {
        ProjectWebhook {
            id: 1,
            name: "My Webhook".to_string(),
            description: "desc".to_string(),
            hook_url: "https://example.com/hook".to_string(),
            all_event: false,
            activity_type_ids: vec![1, 2],
            created_user: sample_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_user(),
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn format_webhook_row_shows_id_name_url() {
        let text = format_webhook_row(&sample_hook());
        assert!(text.contains("[1]"));
        assert!(text.contains("My Webhook"));
        assert!(text.contains("https://example.com/hook"));
    }

    #[test]
    fn format_webhook_detail_shows_full_info() {
        let text = format_webhook_detail(&sample_hook());
        assert!(text.contains("ID: 1"));
        assert!(text.contains("My Webhook"));
        assert!(text.contains("https://example.com/hook"));
        assert!(text.contains("desc"));
        assert!(text.contains("1, 2"));
    }

    #[test]
    fn format_webhook_detail_all_event_shows_all() {
        let mut hook = sample_hook();
        hook.all_event = true;
        let text = format_webhook_detail(&hook);
        assert!(text.contains("Events: all"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            hooks: Some(vec![sample_hook()]),
            hook: None,
        };
        assert!(
            list_with(
                &ProjectWebhookListArgs::new("TEST".to_string(), false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            hooks: Some(vec![sample_hook()]),
            hook: None,
        };
        assert!(list_with(&ProjectWebhookListArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi {
            hooks: None,
            hook: None,
        };
        let err = list_with(
            &ProjectWebhookListArgs::new("TEST".to_string(), false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no hooks"));
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        assert!(
            show_with(
                &ProjectWebhookShowArgs::new("TEST".to_string(), 1, false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        assert!(
            show_with(
                &ProjectWebhookShowArgs::new("TEST".to_string(), 1, true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi {
            hooks: None,
            hook: None,
        };
        let err = show_with(
            &ProjectWebhookShowArgs::new("TEST".to_string(), 1, false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no hook"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        assert!(
            add_with(
                &ProjectWebhookAddArgs::new(
                    "TEST".to_string(),
                    "My Webhook".to_string(),
                    "https://example.com/hook".to_string(),
                    None,
                    None,
                    vec![],
                    false,
                ),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        assert!(
            add_with(
                &ProjectWebhookAddArgs::new(
                    "TEST".to_string(),
                    "My Webhook".to_string(),
                    "https://example.com/hook".to_string(),
                    None,
                    None,
                    vec![],
                    true,
                ),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi {
            hooks: None,
            hook: None,
        };
        let err = add_with(
            &ProjectWebhookAddArgs::new(
                "TEST".to_string(),
                "My Webhook".to_string(),
                "https://example.com/hook".to_string(),
                None,
                None,
                vec![],
                false,
            ),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn update_try_new_rejects_no_fields() {
        let err = ProjectWebhookUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap_err();
        assert!(err.to_string().contains("At least one"));
    }

    #[test]
    fn update_try_new_accepts_name_only() {
        let args = ProjectWebhookUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("New Name".to_string()),
            None,
            None,
            None,
            None,
            false,
        );
        assert!(args.is_ok());
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        let args = ProjectWebhookUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("New Name".to_string()),
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        let args = ProjectWebhookUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("New Name".to_string()),
            None,
            None,
            None,
            None,
            true,
        )
        .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi {
            hooks: None,
            hook: None,
        };
        let args = ProjectWebhookUpdateArgs::try_new(
            "TEST".to_string(),
            1,
            Some("New Name".to_string()),
            None,
            None,
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
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        assert!(
            delete_with(
                &ProjectWebhookDeleteArgs::new("TEST".to_string(), 1, false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            hooks: None,
            hook: Some(sample_hook()),
        };
        assert!(
            delete_with(
                &ProjectWebhookDeleteArgs::new("TEST".to_string(), 1, true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi {
            hooks: None,
            hook: None,
        };
        let err = delete_with(
            &ProjectWebhookDeleteArgs::new("TEST".to_string(), 1, false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
