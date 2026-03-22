use anstream::println;
use anyhow::{Context, Result};

use super::format_team_row;
use crate::api::{BacklogApi, BacklogClient};

pub struct TeamDeleteArgs {
    id: u64,
    json: bool,
}

impl TeamDeleteArgs {
    pub fn new(id: u64, json: bool) -> Self {
        Self { id, json }
    }
}

pub fn delete(args: &TeamDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &TeamDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let team = api.delete_team(args.id)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&team).context("Failed to serialize JSON")?
        );
    } else {
        println!("Deleted: {}", format_team_row(&team));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    use crate::api::team::Team;
    use crate::cmd::team::sample_team;

    struct MockApi {
        team: Option<Team>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_team(&self, _team_id: u64) -> anyhow::Result<Team> {
            self.team.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn args(json: bool) -> TeamDeleteArgs {
        TeamDeleteArgs::new(1, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { team: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
