use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchAddArgs {
    issue_id_or_key: String,
    note: Option<String>,
    json: bool,
}

impl WatchAddArgs {
    pub fn new(issue_id_or_key: String, note: Option<String>, json: bool) -> Self {
        Self {
            issue_id_or_key,
            note,
            json,
        }
    }
}

pub fn add(args: &WatchAddArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    add_with(args, &client)
}

pub fn add_with(args: &WatchAddArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("issueIdOrKey".to_string(), args.issue_id_or_key.clone()));
    if let Some(ref note) = args.note {
        params.push(("note".to_string(), note.clone()));
    }
    let watching = api.add_watching(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&watching).context("Failed to serialize JSON")?
        );
    } else {
        println!(
            "Added: [{}] {} ({})",
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
        fn add_watching(&self, _params: &[(String, String)]) -> anyhow::Result<Watching> {
            self.watching.clone().ok_or_else(|| anyhow!("add failed"))
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

    fn args(json: bool) -> WatchAddArgs {
        WatchAddArgs::new("TEST-1".to_string(), None, json)
    }

    #[test]
    fn add_with_text_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(add_with(&args(false), &api).is_ok());
    }

    #[test]
    fn add_with_json_output_succeeds() {
        let api = MockApi {
            watching: Some(sample_watching()),
        };
        assert!(add_with(&args(true), &api).is_ok());
    }

    #[test]
    fn add_with_propagates_api_error() {
        let api = MockApi { watching: None };
        let err = add_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("add failed"));
    }
}
