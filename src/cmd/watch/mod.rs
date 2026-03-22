mod add;
mod count;
mod delete;
mod list;
mod read;
mod show;
mod update;

pub use add::{WatchAddArgs, add};

#[cfg(test)]
pub(crate) fn sample_watching() -> crate::api::watch::Watching {
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
pub use count::{WatchCountArgs, count};
pub use delete::{WatchDeleteArgs, delete};
pub use list::{WatchListArgs, list};
pub use read::{WatchReadArgs, read};
pub use show::{WatchShowArgs, show};
pub use update::{WatchUpdateArgs, update};
