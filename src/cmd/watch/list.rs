use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchListArgs {
    user_id: u64,
    order: Option<String>,
    sort: Option<String>,
    count: u32,
    offset: Option<u64>,
    resource_already_read: Option<bool>,
    issue_ids: Vec<u64>,
    json: bool,
}

impl WatchListArgs {
    pub fn try_new(
        user_id: u64,
        order: Option<String>,
        sort: Option<String>,
        count: u32,
        offset: Option<u64>,
        resource_already_read: Option<bool>,
        issue_ids: Vec<u64>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        Ok(Self {
            user_id,
            order,
            sort,
            count,
            offset,
            resource_already_read,
            issue_ids,
            json,
        })
    }
}

pub fn list(args: &WatchListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &WatchListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    if let Some(ref sort) = args.sort {
        params.push(("sort".to_string(), sort.clone()));
    }
    params.push(("count".to_string(), args.count.to_string()));
    if let Some(offset) = args.offset {
        params.push(("offset".to_string(), offset.to_string()));
    }
    if let Some(rar) = args.resource_already_read {
        params.push(("resourceAlreadyRead".to_string(), rar.to_string()));
    }
    for id in &args.issue_ids {
        params.push(("issueId[]".to_string(), id.to_string()));
    }
    let watchings = api.get_watchings(args.user_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&watchings).context("Failed to serialize JSON")?
        );
    } else {
        for w in &watchings {
            println!("[{}] {} ({})", w.id, w.issue.summary, w.issue.issue_key);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::watch::Watching;
    use anyhow::anyhow;

    struct MockApi {
        watchings: Option<Vec<Watching>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_watchings(
            &self,
            _user_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<Watching>> {
            self.watchings
                .clone()
                .ok_or_else(|| anyhow!("no watchings"))
        }
    }

    fn sample_watching() -> Watching {
        serde_json::from_value(sample_watching_json()).unwrap()
    }

    fn sample_watching_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "resourceAlreadyRead": false,
            "note": "test note",
            "type": "issue",
            "issue": {
                "id": 10, "projectId": 1, "issueKey": "TEST-1", "keyId": 1,
                "issueType": {"id":1,"projectId":1,"name":"Bug","color":"#990000","displayOrder":0},
                "summary": "Test issue", "description": null, "resolution": null,
                "priority": {"id":3,"name":"Normal"},
                "status": {"id":1,"projectId":1,"name":"Open","color":"#ed8077","displayOrder":1000},
                "assignee": null, "startDate": null, "dueDate": null,
                "estimatedHours": null, "actualHours": null, "parentIssueId": null,
                "createdUser": {"id":1,"userId":"alice","name":"Alice","roleType":1,"lang":null,"mailAddress":null,"lastLoginTime":null},
                "created": "2024-01-01T00:00:00Z",
                "updatedUser": {"id":1,"userId":"alice","name":"Alice","roleType":1,"lang":null,"mailAddress":null,"lastLoginTime":null},
                "updated": "2024-01-01T00:00:00Z"
            },
            "lastContentUpdated": "2024-01-01T00:00:00Z",
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z"
        })
    }

    fn args(json: bool) -> WatchListArgs {
        WatchListArgs::try_new(1, None, None, 20, None, None, vec![], json).unwrap()
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            watchings: Some(vec![sample_watching()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            watchings: Some(vec![sample_watching()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { watchings: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no watchings"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(WatchListArgs::try_new(1, None, None, 101, None, None, vec![], false).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(WatchListArgs::try_new(1, None, None, 0, None, None, vec![], false).is_err());
    }
}
