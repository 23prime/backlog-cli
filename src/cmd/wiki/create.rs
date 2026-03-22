use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::wiki::show::print_wiki;

pub struct WikiCreateArgs {
    project_id: u64,
    name: String,
    content: String,
    mail_notify: bool,
    json: bool,
}

impl WikiCreateArgs {
    pub fn new(
        project_id: u64,
        name: String,
        content: String,
        mail_notify: bool,
        json: bool,
    ) -> Self {
        Self {
            project_id,
            name,
            content,
            mail_notify,
            json,
        }
    }
}

pub fn create(args: &WikiCreateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    create_with(args, &client)
}

pub fn create_with(args: &WikiCreateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = vec![
        ("projectId".to_string(), args.project_id.to_string()),
        ("name".to_string(), args.name.clone()),
        ("content".to_string(), args.content.clone()),
    ];
    if args.mail_notify {
        params.push(("mailNotify".to_string(), "true".to_string()));
    }
    let wiki = api.create_wiki(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&wiki).context("Failed to serialize JSON")?
        );
    } else {
        print_wiki(&wiki);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::wiki::Wiki;
    use crate::cmd::wiki::sample_wiki;
    use anyhow::anyhow;

    struct MockApi {
        wiki: Option<Wiki>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn create_wiki(&self, _params: &[(String, String)]) -> anyhow::Result<Wiki> {
            self.wiki.clone().ok_or_else(|| anyhow!("create failed"))
        }
    }

    fn args(json: bool) -> WikiCreateArgs {
        WikiCreateArgs::new(1, "Home".to_string(), "# Home".to_string(), false, json)
    }

    #[test]
    fn create_with_text_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(create_with(&args(false), &api).is_ok());
    }

    #[test]
    fn create_with_json_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(create_with(&args(true), &api).is_ok());
    }

    #[test]
    fn create_with_propagates_api_error() {
        let api = MockApi { wiki: None };
        let err = create_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("create failed"));
    }
}
