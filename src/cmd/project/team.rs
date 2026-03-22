use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient, team::Team};

pub struct ProjectTeamListArgs {
    key: String,
    json: bool,
}

impl ProjectTeamListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &ProjectTeamListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &ProjectTeamListArgs, api: &dyn BacklogApi) -> Result<()> {
    let teams = api.get_project_teams(&args.key)?;
    if args.json {
        crate::cmd::print_json(&teams)?;
    } else {
        for t in &teams {
            println!("{}", format_project_team_row(t));
        }
    }
    Ok(())
}

pub struct ProjectTeamAddArgs {
    key: String,
    team_id: u64,
    json: bool,
}

impl ProjectTeamAddArgs {
    pub fn new(key: String, team_id: u64, json: bool) -> Self {
        Self { key, team_id, json }
    }
}

pub fn add(args: &ProjectTeamAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &ProjectTeamAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let team = api.add_project_team(&args.key, args.team_id)?;
    if args.json {
        crate::cmd::print_json(&team)?;
    } else {
        println!("Added: {}", format_project_team_row(&team));
    }
    Ok(())
}

pub struct ProjectTeamDeleteArgs {
    key: String,
    team_id: u64,
    json: bool,
}

impl ProjectTeamDeleteArgs {
    pub fn new(key: String, team_id: u64, json: bool) -> Self {
        Self { key, team_id, json }
    }
}

pub fn delete(args: &ProjectTeamDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &ProjectTeamDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let team = api.delete_project_team(&args.key, args.team_id)?;
    if args.json {
        crate::cmd::print_json(&team)?;
    } else {
        println!("Deleted: {}", format_project_team_row(&team));
    }
    Ok(())
}

fn format_project_team_row(t: &Team) -> String {
    format!("[{}] {}", t.id, t.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        teams: Option<Vec<Team>>,
        team: Option<Team>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_teams(&self, _key: &str) -> anyhow::Result<Vec<Team>> {
            self.teams.clone().ok_or_else(|| anyhow!("no teams"))
        }

        fn add_project_team(&self, _key: &str, _team_id: u64) -> anyhow::Result<Team> {
            self.team.clone().ok_or_else(|| anyhow!("add failed"))
        }

        fn delete_project_team(&self, _key: &str, _team_id: u64) -> anyhow::Result<Team> {
            self.team.clone().ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_team() -> Team {
        Team {
            id: 1,
            name: "dev-team".to_string(),
            members: vec![],
            display_order: None,
            created: "2024-01-01T00:00:00Z".to_string(),
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn format_project_team_row_shows_id_and_name() {
        let text = format_project_team_row(&sample_team());
        assert!(text.contains("[1]"));
        assert!(text.contains("dev-team"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            teams: Some(vec![sample_team()]),
            team: None,
        };
        assert!(list_with(&ProjectTeamListArgs::new("TEST".to_string(), false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            teams: Some(vec![sample_team()]),
            team: None,
        };
        assert!(list_with(&ProjectTeamListArgs::new("TEST".to_string(), true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi {
            teams: None,
            team: None,
        };
        let err =
            list_with(&ProjectTeamListArgs::new("TEST".to_string(), false), &api).unwrap_err();
        assert!(err.to_string().contains("no teams"));
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            teams: None,
            team: Some(sample_team()),
        };
        assert!(add_with(&ProjectTeamAddArgs::new("TEST".to_string(), 1, false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            teams: None,
            team: Some(sample_team()),
        };
        assert!(add_with(&ProjectTeamAddArgs::new("TEST".to_string(), 1, true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi {
            teams: None,
            team: None,
        };
        let err =
            add_with(&ProjectTeamAddArgs::new("TEST".to_string(), 1, false), &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            teams: None,
            team: Some(sample_team()),
        };
        assert!(
            delete_with(
                &ProjectTeamDeleteArgs::new("TEST".to_string(), 1, false),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            teams: None,
            team: Some(sample_team()),
        };
        assert!(
            delete_with(
                &ProjectTeamDeleteArgs::new("TEST".to_string(), 1, true),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi {
            teams: None,
            team: None,
        };
        let err = delete_with(
            &ProjectTeamDeleteArgs::new("TEST".to_string(), 1, false),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
