use anstream::println;
use anyhow::Result;

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
        crate::cmd::print_json(&team)?;
    } else {
        println!("Deleted: {}", format_team_row(&team));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    use crate::api::team::{Team, TeamMember};

    struct MockApi {
        team: Option<Team>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_team(&self, _team_id: u64) -> anyhow::Result<Team> {
            self.team.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_member() -> TeamMember {
        TeamMember {
            id: 2,
            user_id: Some("dev".to_string()),
            name: "Developer".to_string(),
            role_type: 2,
            lang: None,
            mail_address: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    fn sample_team() -> Team {
        Team {
            id: 1,
            name: "dev-team".to_string(),
            members: vec![sample_member()],
            display_order: None,
            created: "2024-01-01T00:00:00Z".to_string(),
            updated: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
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
