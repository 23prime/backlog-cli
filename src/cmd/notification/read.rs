use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct NotificationReadArgs {
    pub id: u64,
}

impl NotificationReadArgs {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

pub fn read(args: &NotificationReadArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    read_with(args, &client)
}

pub fn read_with(args: &NotificationReadArgs, api: &dyn BacklogApi) -> Result<()> {
    api.read_notification(args.id)?;
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
    use std::cell::Cell;

    struct MockApi {
        called_with: Cell<Option<u64>>,
    }

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
        fn add_user(&self, _params: &[(String, String)]) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn update_user(
            &self,
            _user_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn delete_user(&self, _user_id: u64) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(&self, _: &[(String, String)]) -> Result<Vec<Activity>> {
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
        fn get_project_activities(&self, _: &str, _: &[(String, String)]) -> Result<Vec<Activity>> {
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
        fn get_teams(&self, _: &[(String, String)]) -> Result<Vec<Team>> {
            unimplemented!()
        }
        fn get_team(&self, _: u64) -> Result<Team> {
            unimplemented!()
        }
        fn get_user_activities(&self, _: u64, _: &[(String, String)]) -> Result<Vec<Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> Result<Vec<RecentlyViewedIssue>> {
            unimplemented!()
        }
        fn get_recently_viewed_projects(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedProject>> {
            unimplemented!()
        }
        fn get_recently_viewed_wikis(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedWiki>> {
            unimplemented!()
        }
        fn get_user_stars(
            &self,
            _user_id: u64,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::Star>> {
            unimplemented!()
        }
        fn count_user_stars(&self, _user_id: u64) -> anyhow::Result<crate::api::user::StarCount> {
            unimplemented!()
        }
        fn get_notifications(&self, _: &[(String, String)]) -> Result<Vec<Notification>> {
            unimplemented!()
        }
        fn count_notifications(&self) -> Result<NotificationCount> {
            unimplemented!()
        }
        fn read_notification(&self, id: u64) -> Result<()> {
            self.called_with.set(Some(id));
            Ok(())
        }
        fn reset_unread_notifications(&self) -> Result<NotificationCount> {
            unimplemented!()
        }
    }

    #[test]
    fn read_calls_api_with_correct_id() {
        let api = MockApi {
            called_with: Cell::new(None),
        };
        read_with(&NotificationReadArgs::new(42), &api).unwrap();
        assert_eq!(api.called_with.get(), Some(42));
    }
}
