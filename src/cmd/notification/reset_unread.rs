use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub fn reset_unread() -> Result<()> {
    let client = BacklogClient::from_config()?;
    reset_unread_with(&client)
}

pub fn reset_unread_with(api: &dyn BacklogApi) -> Result<()> {
    api.reset_unread_notifications()?;
    anstream::println!("Unread count reset.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::activity::Activity;
    use crate::api::disk_usage::DiskUsage;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount};
    use crate::api::notification::{Notification, NotificationCount};
    use crate::api::project::{
        Project, ProjectCategory, ProjectDiskUsage, ProjectIssueType, ProjectStatus, ProjectUser,
        ProjectVersion,
    };
    use crate::api::space::Space;
    use crate::api::space_notification::SpaceNotification;
    use crate::api::team::Team;
    use crate::api::user::{RecentlyViewedIssue, User};
    use crate::api::wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem};

    struct MockApi;

    impl BacklogApi for MockApi {
        fn get_space(&self) -> Result<Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> Result<User> {
            unimplemented!()
        }
        fn get_users(&self) -> Result<Vec<User>> {
            unimplemented!()
        }
        fn get_user(&self, _: u64) -> Result<User> {
            unimplemented!()
        }
        fn get_space_activities(&self) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_space_disk_usage(&self) -> Result<DiskUsage> {
            unimplemented!()
        }
        fn get_space_notification(&self) -> Result<SpaceNotification> {
            unimplemented!()
        }
        fn get_projects(&self) -> Result<Vec<Project>> {
            unimplemented!()
        }
        fn get_project(&self, _: &str) -> Result<Project> {
            unimplemented!()
        }
        fn get_project_activities(&self, _: &str) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(&self, _: &str) -> Result<ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(&self, _: &str) -> Result<Vec<ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(&self, _: &str) -> Result<Vec<ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(&self, _: &str) -> Result<Vec<ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(&self, _: &str) -> Result<Vec<ProjectCategory>> {
            unimplemented!()
        }
        fn get_project_versions(&self, _: &str) -> Result<Vec<ProjectVersion>> {
            unimplemented!()
        }
        fn get_issues(&self, _: &[(String, String)]) -> Result<Vec<Issue>> {
            unimplemented!()
        }
        fn count_issues(&self, _: &[(String, String)]) -> Result<IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _: &str) -> Result<Issue> {
            unimplemented!()
        }
        fn create_issue(&self, _: &[(String, String)]) -> Result<Issue> {
            unimplemented!()
        }
        fn update_issue(&self, _: &str, _: &[(String, String)]) -> Result<Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _: &str) -> Result<Issue> {
            unimplemented!()
        }
        fn get_issue_comments(&self, _: &str) -> Result<Vec<IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(&self, _: &str, _: &[(String, String)]) -> Result<IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _: &str,
            _: u64,
            _: &[(String, String)],
        ) -> Result<IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(&self, _: &str, _: u64) -> Result<IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(&self, _: &str) -> Result<Vec<IssueAttachment>> {
            unimplemented!()
        }
        fn get_wikis(&self, _: &[(String, String)]) -> Result<Vec<WikiListItem>> {
            unimplemented!()
        }
        fn get_wiki(&self, _: u64) -> Result<Wiki> {
            unimplemented!()
        }
        fn create_wiki(&self, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn update_wiki(&self, _: u64, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn delete_wiki(&self, _: u64, _: &[(String, String)]) -> Result<Wiki> {
            unimplemented!()
        }
        fn get_wiki_history(&self, _: u64) -> Result<Vec<WikiHistory>> {
            unimplemented!()
        }
        fn get_wiki_attachments(&self, _: u64) -> Result<Vec<WikiAttachment>> {
            unimplemented!()
        }
        fn get_teams(&self) -> Result<Vec<Team>> {
            unimplemented!()
        }
        fn get_team(&self, _: u64) -> Result<Team> {
            unimplemented!()
        }
        fn get_user_activities(&self, _: u64) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(&self) -> Result<Vec<RecentlyViewedIssue>> {
            unimplemented!()
        }
        fn get_notifications(&self) -> Result<Vec<Notification>> {
            unimplemented!()
        }
        fn count_notifications(&self) -> Result<NotificationCount> {
            unimplemented!()
        }
        fn read_notification(&self, _: u64) -> Result<()> {
            unimplemented!()
        }
        fn reset_unread_notifications(&self) -> Result<NotificationCount> {
            Ok(NotificationCount { count: 0 })
        }
    }

    #[test]
    fn reset_unread_succeeds() {
        reset_unread_with(&MockApi).unwrap();
    }
}
