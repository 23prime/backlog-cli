use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct ResolutionListArgs {
    json: bool,
}

impl ResolutionListArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn list(args: &ResolutionListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ResolutionListArgs, api: &dyn BacklogApi) -> Result<()> {
    let resolutions = api.get_resolutions()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&resolutions).context("Failed to serialize JSON")?
        );
    } else {
        for r in &resolutions {
            println!("[{}] {}", r.id, r.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::resolution::Resolution;
    use anyhow::anyhow;

    struct MockApi {
        resolutions: Option<Vec<Resolution>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_resolutions(&self) -> anyhow::Result<Vec<Resolution>> {
            self.resolutions
                .clone()
                .ok_or_else(|| anyhow!("no resolutions"))
        }
    }

    fn sample_resolutions() -> Vec<Resolution> {
        vec![
            Resolution {
                id: 0,
                name: "Fixed".to_string(),
            },
            Resolution {
                id: 1,
                name: "Won't Fix".to_string(),
            },
            Resolution {
                id: 2,
                name: "Invalid".to_string(),
            },
        ]
    }

    fn args(json: bool) -> ResolutionListArgs {
        ResolutionListArgs::new(json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            resolutions: Some(sample_resolutions()),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            resolutions: Some(sample_resolutions()),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { resolutions: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no resolutions"));
    }
}
