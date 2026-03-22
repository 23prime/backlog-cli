use anstream::println;
use anyhow::Result;

use super::format_team_row;
use crate::api::{BacklogApi, BacklogClient};

pub struct TeamListArgs {
    json: bool,
    pub count: u32,
    pub order: Option<String>,
    pub offset: u64,
}

impl TeamListArgs {
    pub fn try_new(
        json: bool,
        count: u32,
        order: Option<String>,
        offset: u64,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        Ok(Self {
            json,
            count,
            order,
            offset,
        })
    }
}

pub fn list(args: &TeamListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &TeamListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = vec![
        ("count".to_string(), args.count.to_string()),
        ("offset".to_string(), args.offset.to_string()),
    ];
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    let teams = api.get_teams(&params)?;
    if args.json {
        crate::cmd::print_json(&teams)?;
    } else {
        for t in &teams {
            println!("{}", format_team_row(t));
        }
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
        teams: Option<Vec<Team>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_teams(&self, _: &[(String, String)]) -> anyhow::Result<Vec<Team>> {
            self.teams.clone().ok_or_else(|| anyhow!("no teams"))
        }
    }

    #[test]
    fn format_team_row_shows_id_name_member_count() {
        let text = format_team_row(&sample_team());
        assert!(text.contains("[1]"));
        assert!(text.contains("dev-team"));
        assert!(text.contains("1 members"));
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            teams: Some(vec![sample_team()]),
        };
        assert!(list_with(&TeamListArgs::try_new(false, 20, None, 0).unwrap(), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            teams: Some(vec![sample_team()]),
        };
        assert!(list_with(&TeamListArgs::try_new(true, 20, None, 0).unwrap(), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { teams: None };
        let err = list_with(&TeamListArgs::try_new(false, 20, None, 0).unwrap(), &api).unwrap_err();
        assert!(err.to_string().contains("no teams"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(TeamListArgs::try_new(false, 101, None, 0).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(TeamListArgs::try_new(false, 0, None, 0).is_err());
    }

    struct MockApiCapture {
        captured: std::cell::RefCell<Vec<(String, String)>>,
    }

    impl crate::api::BacklogApi for MockApiCapture {
        fn get_teams(&self, params: &[(String, String)]) -> anyhow::Result<Vec<Team>> {
            *self.captured.borrow_mut() = params.to_vec();
            Ok(vec![])
        }
    }

    #[test]
    fn list_with_builds_correct_query_params() {
        let api = MockApiCapture {
            captured: std::cell::RefCell::new(vec![]),
        };
        let args = TeamListArgs::try_new(false, 50, Some("asc".to_string()), 10).unwrap();
        list_with(&args, &api).unwrap();
        let params = api.captured.borrow();
        assert!(params.iter().any(|(k, v)| k == "count" && v == "50"));
        assert!(params.iter().any(|(k, v)| k == "offset" && v == "10"));
        assert!(params.iter().any(|(k, v)| k == "order" && v == "asc"));
    }
}
