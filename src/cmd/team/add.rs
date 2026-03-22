use anstream::println;
use anyhow::Result;

use super::format_team_row;
use crate::api::{BacklogApi, BacklogClient};

pub struct TeamAddArgs {
    name: String,
    members: Vec<u64>,
    json: bool,
}

impl TeamAddArgs {
    pub fn new(name: String, members: Vec<u64>, json: bool) -> Self {
        Self {
            name,
            members,
            json,
        }
    }
}

pub fn add(args: &TeamAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &TeamAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params = vec![("name".to_string(), args.name.clone())];
    for id in &args.members {
        params.push(("members[]".to_string(), id.to_string()));
    }
    let team = api.create_team(&params)?;
    if args.json {
        crate::cmd::print_json(&team)?;
    } else {
        println!("Created: {}", format_team_row(&team));
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
        fn create_team(&self, _params: &[(String, String)]) -> anyhow::Result<Team> {
            self.team.clone().ok_or_else(|| anyhow!("create failed"))
        }
    }

    fn args(json: bool) -> TeamAddArgs {
        TeamAddArgs::new("dev-team".to_string(), vec![2], json)
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(add_with(&args(true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi { team: None };
        let err = add_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("create failed"));
    }
}
