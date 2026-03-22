use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct PriorityListArgs {
    json: bool,
}

impl PriorityListArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn list(args: &PriorityListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &PriorityListArgs, api: &dyn BacklogApi) -> Result<()> {
    let priorities = api.get_priorities()?;
    if args.json {
        crate::cmd::print_json(&priorities)?;
    } else {
        for p in &priorities {
            println!("[{}] {}", p.id, p.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::priority::Priority;
    use anyhow::anyhow;

    struct MockApi {
        priorities: Option<Vec<Priority>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_priorities(&self) -> anyhow::Result<Vec<Priority>> {
            self.priorities
                .clone()
                .ok_or_else(|| anyhow!("no priorities"))
        }
    }

    fn sample_priorities() -> Vec<Priority> {
        vec![
            Priority {
                id: 2,
                name: "High".to_string(),
            },
            Priority {
                id: 3,
                name: "Normal".to_string(),
            },
            Priority {
                id: 4,
                name: "Low".to_string(),
            },
        ]
    }

    fn args(json: bool) -> PriorityListArgs {
        PriorityListArgs::new(json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            priorities: Some(sample_priorities()),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            priorities: Some(sample_priorities()),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { priorities: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no priorities"));
    }
}
