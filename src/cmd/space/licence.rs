use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, licence::Licence};

pub struct SpaceLicenceArgs {
    json: bool,
}

impl SpaceLicenceArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn licence(args: &SpaceLicenceArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    licence_with(args, &client)
}

pub fn licence_with(args: &SpaceLicenceArgs, api: &dyn BacklogApi) -> Result<()> {
    let l = api
        .get_space_licence()
        .context("Failed to fetch space licence")?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&l).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_licence_text(&l));
    }
    Ok(())
}

fn format_licence_text(l: &Licence) -> String {
    let contract = l.contract_type.as_deref().unwrap_or("(not set)");
    format!(
        "Contract:  {}\nStorage:   {} / {} bytes\nStart:     {}",
        contract, l.storage_usage, l.storage_limit, l.start_date
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        licence: Option<Licence>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space_licence(&self) -> Result<Licence> {
            self.licence.clone().ok_or_else(|| anyhow!("no licence"))
        }
    }

    fn sample_licence() -> Licence {
        Licence {
            start_date: "2020-01-01".to_string(),
            contract_type: Some("premium".to_string()),
            storage_limit: 1073741824,
            storage_usage: 5242880,
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn licence_with_text_output_succeeds() {
        let api = MockApi {
            licence: Some(sample_licence()),
        };
        assert!(licence_with(&SpaceLicenceArgs::new(false), &api).is_ok());
    }

    #[test]
    fn licence_with_json_output_succeeds() {
        let api = MockApi {
            licence: Some(sample_licence()),
        };
        assert!(licence_with(&SpaceLicenceArgs::new(true), &api).is_ok());
    }

    #[test]
    fn licence_with_propagates_api_error() {
        let api = MockApi { licence: None };
        let err = licence_with(&SpaceLicenceArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("Failed to fetch space licence"));
    }

    #[test]
    fn format_licence_text_contains_fields() {
        let text = format_licence_text(&sample_licence());
        assert!(text.contains("premium"));
        assert!(text.contains("5242880"));
        assert!(text.contains("1073741824"));
        assert!(text.contains("2020-01-01"));
    }

    #[test]
    fn format_licence_text_with_null_contract_type() {
        let l = Licence {
            start_date: "2020-01-01".to_string(),
            contract_type: None,
            storage_limit: 1073741824,
            storage_usage: 0,
            extra: BTreeMap::new(),
        };
        let text = format_licence_text(&l);
        assert!(text.contains("(not set)"));
    }
}
