use anstream::println;
use anyhow::Result;
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, team::Team};

pub struct TeamShowArgs {
    id: u64,
    json: bool,
}

impl TeamShowArgs {
    pub fn new(id: u64, json: bool) -> Self {
        Self { id, json }
    }
}

pub fn show(args: &TeamShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &TeamShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let team = api.get_team(args.id)?;
    if args.json {
        crate::cmd::print_json(&team)?;
    } else {
        println!("{}", format_team_text(&team));
    }
    Ok(())
}

fn format_team_text(t: &Team) -> String {
    let members = t
        .members
        .iter()
        .map(|m| format!("    [{}] {}", m.id, m.name))
        .collect::<Vec<_>>()
        .join("\n");
    let members_section = if members.is_empty() {
        "    (none)".to_string()
    } else {
        members
    };
    format!(
        "ID:      {}\nName:    {}\nCreated: {}\nUpdated: {}\nMembers:\n{}",
        t.id.to_string().bold(),
        t.name,
        t.created,
        t.updated,
        members_section,
    )
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
        fn get_team(&self, _team_id: u64) -> anyhow::Result<Team> {
            self.team.clone().ok_or_else(|| anyhow!("no team"))
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
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(show_with(&TeamShowArgs::new(1, false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            team: Some(sample_team()),
        };
        assert!(show_with(&TeamShowArgs::new(1, true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { team: None };
        let err = show_with(&TeamShowArgs::new(999, false), &api).unwrap_err();
        assert!(err.to_string().contains("no team"));
    }

    #[test]
    fn format_team_text_contains_fields() {
        let text = format_team_text(&sample_team());
        assert!(text.contains("1"));
        assert!(text.contains("dev-team"));
        assert!(text.contains("Developer"));
        assert!(text.contains("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn format_team_text_empty_members() {
        let mut t = sample_team();
        t.members.clear();
        let text = format_team_text(&t);
        assert!(text.contains("(none)"));
    }
}
