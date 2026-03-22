use anstream::println;
use anyhow::Result;

use super::format_team_row;
use crate::api::{BacklogApi, BacklogClient};

#[cfg_attr(test, derive(Debug))]
pub struct TeamUpdateArgs {
    id: u64,
    name: Option<String>,
    members: Option<Vec<u64>>,
    json: bool,
}

impl TeamUpdateArgs {
    pub fn try_new(
        id: u64,
        name: Option<String>,
        members: Option<Vec<u64>>,
        json: bool,
    ) -> anyhow::Result<Self> {
        let no_members = members.as_ref().is_none_or(|m| m.is_empty());
        if name.is_none() && no_members {
            anyhow::bail!("at least one of --name or --member must be provided");
        }
        Ok(Self {
            id,
            name,
            members,
            json,
        })
    }
}

pub fn update(args: &TeamUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &TeamUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = vec![];
    if let Some(ref name) = args.name {
        params.push(("name".to_string(), name.clone()));
    }
    if let Some(ref members) = args.members {
        for id in members {
            params.push(("members[]".to_string(), id.to_string()));
        }
    }
    let team = api.update_team(args.id, &params)?;
    if args.json {
        crate::cmd::print_json(&team)?;
    } else {
        println!("Updated: {}", format_team_row(&team));
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
        fn update_team(&self, _team_id: u64, _params: &[(String, String)]) -> anyhow::Result<Team> {
            self.team.clone().ok_or_else(|| anyhow!("update failed"))
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

    fn args(json: bool) -> TeamUpdateArgs {
        TeamUpdateArgs::try_new(1, Some("dev-team".to_string()), None, json).unwrap()
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { team: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }

    #[test]
    fn try_new_fails_when_no_fields_provided() {
        let err = TeamUpdateArgs::try_new(1, None, None, false).unwrap_err();
        assert!(err.to_string().contains("at least one"));
    }

    #[test]
    fn try_new_succeeds_with_name_only() {
        assert!(TeamUpdateArgs::try_new(1, Some("new-name".to_string()), None, false).is_ok());
    }

    #[test]
    fn try_new_succeeds_with_members_only() {
        assert!(TeamUpdateArgs::try_new(1, None, Some(vec![2, 3]), false).is_ok());
    }

    #[test]
    fn try_new_fails_when_members_is_empty_and_name_absent() {
        let err = TeamUpdateArgs::try_new(1, None, Some(vec![]), false).unwrap_err();
        assert!(err.to_string().contains("at least one"));
    }
}
