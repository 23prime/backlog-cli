use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct UserStarListArgs {
    user_id: u64,
    min_id: Option<u64>,
    max_id: Option<u64>,
    count: u32,
    order: Option<String>,
    json: bool,
}

impl UserStarListArgs {
    pub fn try_new(
        user_id: u64,
        min_id: Option<u64>,
        max_id: Option<u64>,
        count: u32,
        order: Option<String>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        if let (Some(min), Some(max)) = (min_id, max_id)
            && min > max
        {
            anyhow::bail!("min-id must be less than or equal to max-id");
        }
        Ok(Self {
            user_id,
            min_id,
            max_id,
            count,
            order,
            json,
        })
    }
}

pub fn list(args: &UserStarListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &UserStarListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("count".to_string(), args.count.to_string()));
    if let Some(min_id) = args.min_id {
        params.push(("minId".to_string(), min_id.to_string()));
    }
    if let Some(max_id) = args.max_id {
        params.push(("maxId".to_string(), max_id.to_string()));
    }
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    let stars = api.get_user_stars(args.user_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&stars).context("Failed to serialize JSON")?
        );
    } else {
        for star in &stars {
            println!("[{}] {}", star.id, star.title);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::{Star, User};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        stars: Option<Vec<Star>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_user_stars(
            &self,
            _user_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<Star>> {
            self.stars.clone().ok_or_else(|| anyhow!("no stars"))
        }
    }

    fn sample_star() -> Star {
        Star {
            id: 1,
            comment: None,
            url: "https://example.com/issue/1".to_string(),
            title: "Issue title".to_string(),
            presenter: User {
                id: 2,
                user_id: Some("alice".to_string()),
                name: "Alice".to_string(),
                mail_address: None,
                role_type: 1,
                lang: None,
                last_login_time: None,
                extra: BTreeMap::new(),
            },
            created: "2024-01-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> UserStarListArgs {
        UserStarListArgs::try_new(1, None, None, 20, None, json).unwrap()
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            stars: Some(vec![sample_star()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            stars: Some(vec![sample_star()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { stars: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no stars"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(UserStarListArgs::try_new(1, None, None, 101, None, false).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(UserStarListArgs::try_new(1, None, None, 0, None, false).is_err());
    }

    #[test]
    fn try_new_rejects_min_id_greater_than_max_id() {
        assert!(UserStarListArgs::try_new(1, Some(200), Some(100), 20, None, false).is_err());
    }
}
