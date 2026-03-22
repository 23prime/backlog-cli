use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchDeleteArgs {
    watching_id: u64,
    json: bool,
}

impl WatchDeleteArgs {
    pub fn new(watching_id: u64, json: bool) -> Self {
        Self { watching_id, json }
    }
}

pub fn delete(args: &WatchDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &WatchDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    let watching = api.delete_watching(args.watching_id)?;
    if args.json {
        crate::cmd::print_json(&watching)?;
    } else {
        println!(
            "Deleted: [{}] {} ({})",
            watching.id, watching.issue.summary, watching.issue.issue_key
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::watch::Watching;
    use anyhow::anyhow;

    struct MockApi {
        watching: Option<Watching>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_watching(&self, _watching_id: u64) -> anyhow::Result<Watching> {
            self.watching
                .clone()
                .ok_or_else(|| anyhow!("delete failed"))
        }
    }

    fn sample_watching() -> Watching {
        serde_json::from_value(serde_json::json!({
            "id": 1, "resourceAlreadyRead": false, "note": null, "type": "issue",
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
            "lastContentUpdated": null,
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z"
        }))
        .unwrap()
    }

    fn args(json: bool) -> WatchDeleteArgs {
        WatchDeleteArgs::new(1, json)
    }

    #[test]
    fn delete_with_text_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(delete_with(&args(false), &api).is_ok());
    }

    #[test]
    fn delete_with_json_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(delete_with(&args(true), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { watching: None };
        let err = delete_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("delete failed"));
    }
}
