use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchUpdateArgs {
    watching_id: u64,
    note: String,
    json: bool,
}

impl WatchUpdateArgs {
    pub fn new(watching_id: u64, note: String, json: bool) -> Self {
        Self {
            watching_id,
            note,
            json,
        }
    }
}

pub fn update(args: &WatchUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &WatchUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = vec![("note".to_string(), args.note.clone())];
    let watching = api.update_watching(args.watching_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&watching).context("Failed to serialize JSON")?
        );
    } else {
        println!(
            "Updated: [{}] {} ({})",
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
        fn update_watching(
            &self,
            _watching_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<Watching> {
            self.watching
                .clone()
                .ok_or_else(|| anyhow!("update failed"))
        }
    }

    fn sample_watching() -> Watching {
        serde_json::from_value(serde_json::json!({
            "id": 1, "resourceAlreadyRead": false, "note": "updated note", "type": "issue",
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
            "updated": "2024-01-02T00:00:00Z"
        }))
        .unwrap()
    }

    fn args(json: bool) -> WatchUpdateArgs {
        WatchUpdateArgs::new(1, "updated note".to_string(), json)
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(update_with(&args(false), &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(update_with(&args(true), &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { watching: None };
        let err = update_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("update failed"));
    }
}
