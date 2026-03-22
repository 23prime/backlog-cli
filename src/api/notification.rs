use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BacklogClient;
use super::deserialize;
use crate::api::issue::{Issue, IssueComment};
use crate::api::project::Project;
use crate::api::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: u64,
    pub already_read: bool,
    pub reason: u64,
    pub resource_already_read: bool,
    pub project: Project,
    pub issue: Option<Issue>,
    pub comment: Option<IssueComment>,
    /// Not yet parsed — no PullRequest struct exists.
    pub pull_request: Option<serde_json::Value>,
    /// Not yet parsed — no PullRequest struct exists.
    pub pull_request_comment: Option<serde_json::Value>,
    pub sender: User,
    pub created: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationCount {
    pub count: u64,
}

impl BacklogClient {
    pub fn get_notifications(&self, params: &[(String, String)]) -> Result<Vec<Notification>> {
        let value = self.get_with_query("/notifications", params)?;
        deserialize(value)
    }

    pub fn count_notifications(&self) -> Result<NotificationCount> {
        let value = self.get("/notifications/count")?;
        deserialize(value)
    }

    pub fn read_notification(&self, id: u64) -> Result<()> {
        self.post_form(&format!("/notifications/{id}/markAsRead"), &[])?;
        Ok(())
    }

    pub fn reset_unread_notifications(&self) -> Result<NotificationCount> {
        let value = self.post_form("/notifications/markAsRead", &[])?;
        deserialize(value)
    }
}
