use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct UserRecentlyViewedWikisArgs {
    json: bool,
    pub count: u32,
    pub offset: u64,
    pub order: Option<String>,
}

impl UserRecentlyViewedWikisArgs {
    pub fn try_new(
        json: bool,
        count: u32,
        offset: u64,
        order: Option<String>,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        Ok(Self {
            json,
            count,
            offset,
            order,
        })
    }
}

pub fn recently_viewed_wikis(args: &UserRecentlyViewedWikisArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    recently_viewed_wikis_with(args, &client)
}

pub fn recently_viewed_wikis_with(
    args: &UserRecentlyViewedWikisArgs,
    api: &dyn BacklogApi,
) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("count".to_string(), args.count.to_string()));
    params.push(("offset".to_string(), args.offset.to_string()));
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    let items = api.get_recently_viewed_wikis(&params)?;
    if args.json {
        crate::cmd::print_json(&items)?;
    } else {
        for item in &items {
            println!(
                "[{}] {} (project: {})",
                item.page.id, item.page.name, item.page.project_id
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::RecentlyViewedWiki;
    use crate::api::wiki::{WikiListItem, WikiUser};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        items: Option<Vec<RecentlyViewedWiki>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_recently_viewed_wikis(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<RecentlyViewedWiki>> {
            self.items.clone().ok_or_else(|| anyhow!("no items"))
        }
    }

    fn sample_wiki_user() -> WikiUser {
        WikiUser {
            id: 1,
            user_id: Some("admin".to_string()),
            name: "Admin".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    fn sample_item() -> RecentlyViewedWiki {
        RecentlyViewedWiki {
            page: WikiListItem {
                id: 1,
                project_id: 1,
                name: "Home".to_string(),
                tags: vec![],
                created_user: sample_wiki_user(),
                created: "2024-01-01T00:00:00Z".to_string(),
                updated_user: sample_wiki_user(),
                updated: "2024-06-01T00:00:00Z".to_string(),
                extra: BTreeMap::new(),
            },
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> UserRecentlyViewedWikisArgs {
        UserRecentlyViewedWikisArgs::try_new(json, 20, 0, None).unwrap()
    }

    #[test]
    fn recently_viewed_wikis_with_text_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(recently_viewed_wikis_with(&args(false), &api).is_ok());
    }

    #[test]
    fn recently_viewed_wikis_with_json_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(recently_viewed_wikis_with(&args(true), &api).is_ok());
    }

    #[test]
    fn recently_viewed_wikis_with_propagates_api_error() {
        let api = MockApi { items: None };
        let err = recently_viewed_wikis_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no items"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(UserRecentlyViewedWikisArgs::try_new(false, 101, 0, None).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(UserRecentlyViewedWikisArgs::try_new(false, 0, 0, None).is_err());
    }
}
