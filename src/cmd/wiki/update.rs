use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::wiki::show::print_wiki;

#[cfg_attr(test, derive(Debug))]
pub struct WikiUpdateArgs {
    wiki_id: u64,
    name: Option<String>,
    content: Option<String>,
    mail_notify: bool,
    json: bool,
}

impl WikiUpdateArgs {
    pub fn try_new(
        wiki_id: u64,
        name: Option<String>,
        content: Option<String>,
        mail_notify: bool,
        json: bool,
    ) -> Result<Self> {
        if name.is_none() && content.is_none() {
            return Err(anyhow::anyhow!(
                "at least one of --name or --content must be specified"
            ));
        }
        Ok(Self {
            wiki_id,
            name,
            content,
            mail_notify,
            json,
        })
    }
}

pub fn update(args: &WikiUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &WikiUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(name) = &args.name {
        params.push(("name".to_string(), name.clone()));
    }
    if let Some(content) = &args.content {
        params.push(("content".to_string(), content.clone()));
    }
    if args.mail_notify {
        params.push(("mailNotify".to_string(), "true".to_string()));
    }
    let wiki = api.update_wiki(args.wiki_id, &params)?;
    if args.json {
        crate::cmd::print_json(&wiki)?;
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
        fn update_wiki(&self, _wiki_id: u64, _params: &[(String, String)]) -> anyhow::Result<Wiki> {
            self.wiki.clone().ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn args(json: bool) -> WikiUpdateArgs {
        WikiUpdateArgs::try_new(1, Some("Updated".to_string()), None, false, json).unwrap()
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            wiki: Some(sample_wiki()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { wiki: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn update_rejects_no_fields() {
        let err = WikiUpdateArgs::try_new(1, None, None, false, false).unwrap_err();
        assert!(
            err.to_string()
                .contains("at least one of --name or --content")
        );
    }
}
