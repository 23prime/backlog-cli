use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, user::RecentlyViewedIssue};

pub struct UserRecentlyViewedArgs {
    json: bool,
    pub count: u32,
    pub offset: u64,
    pub order: Option<String>,
}

impl UserRecentlyViewedArgs {
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

pub fn recently_viewed(args: &UserRecentlyViewedArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    recently_viewed_with(args, &client)
}

pub fn recently_viewed_with(args: &UserRecentlyViewedArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("count".to_string(), args.count.to_string()));
    params.push(("offset".to_string(), args.offset.to_string()));
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    let items = api.get_recently_viewed_issues(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&items).context("Failed to serialize JSON")?
        );
    } else {
        for item in &items {
            println!("{}", format_row(item));
        }
    }
    Ok(())
}

fn format_row(item: &RecentlyViewedIssue) -> String {
    let status = &item.issue.status.name;
    let assignee = item
        .issue
        .assignee
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("-");
    format!(
        "[{}] {} ({}, {})",
        item.issue.issue_key, item.issue.summary, status, assignee,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::issue::{Issue, IssuePriority, IssueStatus, IssueType, IssueUser};
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        items: Option<Vec<RecentlyViewedIssue>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<RecentlyViewedIssue>> {
            self.items.clone().ok_or_else(|| anyhow!("no items"))
        }
    }

    fn sample_issue_user() -> IssueUser {
        IssueUser {
            id: 1,
            user_id: Some("admin".to_string()),
            name: "admin".to_string(),
            role_type: 1,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    fn sample_issue() -> Issue {
        Issue {
            id: 1,
            project_id: 1,
            issue_key: "BLG-1".to_string(),
            key_id: 1,
            summary: "Fix login".to_string(),
            description: None,
            resolution: None,
            status: IssueStatus {
                id: 1,
                project_id: 1,
                name: "Open".to_string(),
                color: "#ed8077".to_string(),
                display_order: 1000,
            },
            priority: IssuePriority {
                id: 2,
                name: "Normal".to_string(),
            },
            issue_type: IssueType {
                id: 1,
                project_id: 1,
                name: "Bug".to_string(),
                color: "#990000".to_string(),
                display_order: 0,
            },
            assignee: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            actual_hours: None,
            parent_issue_id: None,
            created_user: sample_issue_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: sample_issue_user(),
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn sample_item() -> RecentlyViewedIssue {
        RecentlyViewedIssue {
            issue: sample_issue(),
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn format_row_contains_fields() {
        let text = format_row(&sample_item());
        assert!(text.contains("[BLG-1]"));
        assert!(text.contains("Fix login"));
        assert!(text.contains("(Open, -)"));
    }

    #[test]
    fn recently_viewed_with_text_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(
            recently_viewed_with(
                &UserRecentlyViewedArgs::try_new(false, 20, 0, None).unwrap(),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn recently_viewed_with_json_output_succeeds() {
        let api = MockApi {
            items: Some(vec![sample_item()]),
        };
        assert!(
            recently_viewed_with(
                &UserRecentlyViewedArgs::try_new(true, 20, 0, None).unwrap(),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn recently_viewed_with_propagates_api_error() {
        let api = MockApi { items: None };
        let err = recently_viewed_with(
            &UserRecentlyViewedArgs::try_new(false, 20, 0, None).unwrap(),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no items"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(UserRecentlyViewedArgs::try_new(false, 101, 0, None).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(UserRecentlyViewedArgs::try_new(false, 0, 0, None).is_err());
    }

    struct MockApiCapture {
        captured: std::cell::RefCell<Vec<(String, String)>>,
    }

    impl crate::api::BacklogApi for MockApiCapture {
        fn get_recently_viewed_issues(
            &self,
            params: &[(String, String)],
        ) -> anyhow::Result<Vec<RecentlyViewedIssue>> {
            *self.captured.borrow_mut() = params.to_vec();
            Ok(vec![])
        }
    }

    #[test]
    fn recently_viewed_with_builds_correct_query_params() {
        let api = MockApiCapture {
            captured: std::cell::RefCell::new(vec![]),
        };
        let args = UserRecentlyViewedArgs::try_new(false, 50, 10, Some("asc".to_string())).unwrap();
        recently_viewed_with(&args, &api).unwrap();
        let params = api.captured.borrow();
        assert!(params.iter().any(|(k, v)| k == "count" && v == "50"));
        assert!(params.iter().any(|(k, v)| k == "offset" && v == "10"));
        assert!(params.iter().any(|(k, v)| k == "order" && v == "asc"));
    }
}
