use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::cell::RefCell;
use std::time::Duration;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub mod activity;
pub mod disk_usage;
pub mod issue;
pub mod licence;
pub mod notification;
pub mod priority;
pub mod project;
pub mod rate_limit;
pub mod resolution;
pub mod space;
pub mod space_notification;
pub mod team;
pub mod user;
pub mod watch;
pub mod wiki;

use activity::Activity;
use disk_usage::DiskUsage;
use issue::{
    Issue, IssueAttachment, IssueComment, IssueCommentCount, IssueCommentNotification, IssueCount,
    IssueParticipant, IssueSharedFile,
};
use licence::Licence;
use notification::{Notification, NotificationCount};
use priority::Priority;
use project::{
    Project, ProjectCategory, ProjectCustomField, ProjectDiskUsage, ProjectIssueType,
    ProjectStatus, ProjectUser, ProjectVersion,
};
use rate_limit::RateLimit;
use resolution::Resolution;
use space::Space;
use space_notification::SpaceNotification;
use team::Team;
use user::{RecentlyViewedIssue, RecentlyViewedProject, RecentlyViewedWiki, Star, StarCount, User};
use watch::{Watching, WatchingCount};
use wiki::{Wiki, WikiAttachment, WikiHistory, WikiListItem};

/// Abstraction over the Backlog HTTP API.
///
/// All methods have a default body of `unimplemented!()` so that test
/// `MockApi` structs only need to override the methods actually exercised
/// by the test.  `impl BacklogApi for BacklogClient` overrides every method
/// with a real HTTP call.
///
/// # Adding a new method
///
/// 1. Add the method signature with a default `{ unimplemented!() }` body
///    here (using `_`-prefixed parameter names to suppress unused-variable
///    warnings in the default).
/// 2. Override it in `impl BacklogApi for BacklogClient` below.
/// 3. Test `MockApi` structs **do not** need to be updated — the default
///    `unimplemented!()` fires automatically if an untested method is called.
pub trait BacklogApi {
    fn get_space(&self) -> Result<Space> {
        unimplemented!()
    }
    fn get_rate_limit(&self) -> Result<RateLimit> {
        unimplemented!()
    }
    fn get_myself(&self) -> Result<User> {
        unimplemented!()
    }
    fn get_users(&self) -> Result<Vec<User>> {
        unimplemented!()
    }
    fn get_user(&self, _user_id: u64) -> Result<User> {
        unimplemented!()
    }
    fn get_space_activities(&self, _params: &[(String, String)]) -> Result<Vec<Activity>> {
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
    fn get_project(&self, _key: &str) -> Result<Project> {
        unimplemented!()
    }
    fn get_project_activities(
        &self,
        _key: &str,
        _params: &[(String, String)],
    ) -> Result<Vec<Activity>> {
        unimplemented!()
    }
    fn get_project_disk_usage(&self, _key: &str) -> Result<ProjectDiskUsage> {
        unimplemented!()
    }
    fn get_project_users(&self, _key: &str) -> Result<Vec<ProjectUser>> {
        unimplemented!()
    }
    fn get_project_statuses(&self, _key: &str) -> Result<Vec<ProjectStatus>> {
        unimplemented!()
    }
    fn add_project_status(&self, _key: &str, _name: &str, _color: &str) -> Result<ProjectStatus> {
        unimplemented!()
    }
    fn update_project_status(
        &self,
        _key: &str,
        _status_id: u64,
        _params: &[(String, String)],
    ) -> Result<ProjectStatus> {
        unimplemented!()
    }
    fn delete_project_status(
        &self,
        _key: &str,
        _status_id: u64,
        _substitute_status_id: u64,
    ) -> Result<ProjectStatus> {
        unimplemented!()
    }
    fn reorder_project_statuses(
        &self,
        _key: &str,
        _status_ids: &[u64],
    ) -> Result<Vec<ProjectStatus>> {
        unimplemented!()
    }
    fn get_project_issue_types(&self, _key: &str) -> Result<Vec<ProjectIssueType>> {
        unimplemented!()
    }
    fn add_project_issue_type(
        &self,
        _key: &str,
        _name: &str,
        _color: &str,
    ) -> Result<ProjectIssueType> {
        unimplemented!()
    }
    fn update_project_issue_type(
        &self,
        _key: &str,
        _issue_type_id: u64,
        _params: &[(String, String)],
    ) -> Result<ProjectIssueType> {
        unimplemented!()
    }
    fn delete_project_issue_type(
        &self,
        _key: &str,
        _issue_type_id: u64,
        _substitute_issue_type_id: u64,
    ) -> Result<ProjectIssueType> {
        unimplemented!()
    }
    fn get_project_categories(&self, _key: &str) -> Result<Vec<ProjectCategory>> {
        unimplemented!()
    }
    fn add_project_category(&self, _key: &str, _name: &str) -> Result<ProjectCategory> {
        unimplemented!()
    }
    fn update_project_category(
        &self,
        _key: &str,
        _category_id: u64,
        _name: &str,
    ) -> Result<ProjectCategory> {
        unimplemented!()
    }
    fn delete_project_category(&self, _key: &str, _category_id: u64) -> Result<ProjectCategory> {
        unimplemented!()
    }
    fn get_project_versions(&self, _key: &str) -> Result<Vec<ProjectVersion>> {
        unimplemented!()
    }
    fn add_project_version(
        &self,
        _key: &str,
        _name: &str,
        _description: Option<&str>,
        _start_date: Option<&str>,
        _release_due_date: Option<&str>,
    ) -> Result<ProjectVersion> {
        unimplemented!()
    }
    fn update_project_version(
        &self,
        _key: &str,
        _version_id: u64,
        _name: &str,
        _description: Option<&str>,
        _start_date: Option<&str>,
        _release_due_date: Option<&str>,
        _archived: Option<bool>,
    ) -> Result<ProjectVersion> {
        unimplemented!()
    }
    fn delete_project_version(&self, _key: &str, _version_id: u64) -> Result<ProjectVersion> {
        unimplemented!()
    }
    fn create_project(&self, _params: &[(String, String)]) -> Result<Project> {
        unimplemented!()
    }
    fn update_project(&self, _key: &str, _params: &[(String, String)]) -> Result<Project> {
        unimplemented!()
    }
    fn delete_project(&self, _key: &str) -> Result<Project> {
        unimplemented!()
    }
    fn add_project_user(&self, _key: &str, _user_id: u64) -> Result<ProjectUser> {
        unimplemented!()
    }
    fn delete_project_user(&self, _key: &str, _user_id: u64) -> Result<ProjectUser> {
        unimplemented!()
    }
    fn get_project_administrators(&self, _key: &str) -> Result<Vec<ProjectUser>> {
        unimplemented!()
    }
    fn add_project_administrator(&self, _key: &str, _user_id: u64) -> Result<ProjectUser> {
        unimplemented!()
    }
    fn delete_project_administrator(&self, _key: &str, _user_id: u64) -> Result<ProjectUser> {
        unimplemented!()
    }
    fn get_project_custom_fields(&self, _key: &str) -> Result<Vec<ProjectCustomField>> {
        unimplemented!()
    }
    fn add_project_custom_field(
        &self,
        _key: &str,
        _type_id: u64,
        _name: &str,
        _description: Option<&str>,
        _required: Option<bool>,
    ) -> Result<ProjectCustomField> {
        unimplemented!()
    }
    fn update_project_custom_field(
        &self,
        _key: &str,
        _custom_field_id: u64,
        _name: Option<&str>,
        _description: Option<&str>,
        _required: Option<bool>,
    ) -> Result<ProjectCustomField> {
        unimplemented!()
    }
    fn delete_project_custom_field(
        &self,
        _key: &str,
        _custom_field_id: u64,
    ) -> Result<ProjectCustomField> {
        unimplemented!()
    }
    fn add_project_custom_field_item(
        &self,
        _key: &str,
        _custom_field_id: u64,
        _name: &str,
    ) -> Result<ProjectCustomField> {
        unimplemented!()
    }
    fn update_project_custom_field_item(
        &self,
        _key: &str,
        _custom_field_id: u64,
        _item_id: u64,
        _name: &str,
    ) -> Result<ProjectCustomField> {
        unimplemented!()
    }
    fn delete_project_custom_field_item(
        &self,
        _key: &str,
        _custom_field_id: u64,
        _item_id: u64,
    ) -> Result<ProjectCustomField> {
        unimplemented!()
    }
    fn get_issues(&self, _params: &[(String, String)]) -> Result<Vec<Issue>> {
        unimplemented!()
    }
    fn count_issues(&self, _params: &[(String, String)]) -> Result<IssueCount> {
        unimplemented!()
    }
    fn get_issue(&self, _key: &str) -> Result<Issue> {
        unimplemented!()
    }
    fn create_issue(&self, _params: &[(String, String)]) -> Result<Issue> {
        unimplemented!()
    }
    fn update_issue(&self, _key: &str, _params: &[(String, String)]) -> Result<Issue> {
        unimplemented!()
    }
    fn delete_issue(&self, _key: &str) -> Result<Issue> {
        unimplemented!()
    }
    fn get_issue_comments(&self, _key: &str) -> Result<Vec<IssueComment>> {
        unimplemented!()
    }
    fn add_issue_comment(&self, _key: &str, _params: &[(String, String)]) -> Result<IssueComment> {
        unimplemented!()
    }
    fn update_issue_comment(
        &self,
        _key: &str,
        _comment_id: u64,
        _params: &[(String, String)],
    ) -> Result<IssueComment> {
        unimplemented!()
    }
    fn delete_issue_comment(&self, _key: &str, _comment_id: u64) -> Result<IssueComment> {
        unimplemented!()
    }
    fn get_issue_attachments(&self, _key: &str) -> Result<Vec<IssueAttachment>> {
        unimplemented!()
    }
    fn delete_issue_attachment(&self, _key: &str, _attachment_id: u64) -> Result<IssueAttachment> {
        unimplemented!()
    }
    fn download_issue_attachment(
        &self,
        _key: &str,
        _attachment_id: u64,
    ) -> Result<(Vec<u8>, String)> {
        unimplemented!()
    }
    fn get_issue_participants(&self, _key: &str) -> Result<Vec<IssueParticipant>> {
        unimplemented!()
    }
    fn get_issue_shared_files(&self, _key: &str) -> Result<Vec<IssueSharedFile>> {
        unimplemented!()
    }
    fn link_issue_shared_files(
        &self,
        _key: &str,
        _shared_file_ids: &[u64],
    ) -> Result<Vec<IssueSharedFile>> {
        unimplemented!()
    }
    fn unlink_issue_shared_file(
        &self,
        _key: &str,
        _shared_file_id: u64,
    ) -> Result<IssueSharedFile> {
        unimplemented!()
    }
    fn count_issue_comments(&self, _key: &str) -> Result<IssueCommentCount> {
        unimplemented!()
    }
    fn get_issue_comment(&self, _key: &str, _comment_id: u64) -> Result<IssueComment> {
        unimplemented!()
    }
    fn get_issue_comment_notifications(
        &self,
        _key: &str,
        _comment_id: u64,
    ) -> Result<Vec<IssueCommentNotification>> {
        unimplemented!()
    }
    fn add_issue_comment_notifications(
        &self,
        _key: &str,
        _comment_id: u64,
        _params: &[(String, String)],
    ) -> Result<Vec<IssueCommentNotification>> {
        unimplemented!()
    }
    fn get_wikis(&self, _params: &[(String, String)]) -> Result<Vec<WikiListItem>> {
        unimplemented!()
    }
    fn get_wiki(&self, _wiki_id: u64) -> Result<Wiki> {
        unimplemented!()
    }
    fn create_wiki(&self, _params: &[(String, String)]) -> Result<Wiki> {
        unimplemented!()
    }
    fn update_wiki(&self, _wiki_id: u64, _params: &[(String, String)]) -> Result<Wiki> {
        unimplemented!()
    }
    fn delete_wiki(&self, _wiki_id: u64, _params: &[(String, String)]) -> Result<Wiki> {
        unimplemented!()
    }
    fn get_wiki_history(&self, _wiki_id: u64) -> Result<Vec<WikiHistory>> {
        unimplemented!()
    }
    fn get_wiki_attachments(&self, _wiki_id: u64) -> Result<Vec<WikiAttachment>> {
        unimplemented!()
    }
    fn get_teams(&self, _params: &[(String, String)]) -> Result<Vec<Team>> {
        unimplemented!()
    }
    fn get_team(&self, _team_id: u64) -> Result<Team> {
        unimplemented!()
    }
    fn get_user_activities(
        &self,
        _user_id: u64,
        _params: &[(String, String)],
    ) -> Result<Vec<Activity>> {
        unimplemented!()
    }
    fn get_recently_viewed_issues(
        &self,
        _params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedIssue>> {
        unimplemented!()
    }
    fn add_user(&self, _params: &[(String, String)]) -> Result<User> {
        unimplemented!()
    }
    fn update_user(&self, _user_id: u64, _params: &[(String, String)]) -> Result<User> {
        unimplemented!()
    }
    fn delete_user(&self, _user_id: u64) -> Result<User> {
        unimplemented!()
    }
    fn get_recently_viewed_projects(
        &self,
        _params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedProject>> {
        unimplemented!()
    }
    fn get_recently_viewed_wikis(
        &self,
        _params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedWiki>> {
        unimplemented!()
    }
    fn get_user_stars(&self, _user_id: u64, _params: &[(String, String)]) -> Result<Vec<Star>> {
        unimplemented!()
    }
    fn count_user_stars(&self, _user_id: u64, _params: &[(String, String)]) -> Result<StarCount> {
        unimplemented!()
    }
    fn get_notifications(&self, _params: &[(String, String)]) -> Result<Vec<Notification>> {
        unimplemented!()
    }
    fn count_notifications(&self) -> Result<NotificationCount> {
        unimplemented!()
    }
    fn read_notification(&self, _id: u64) -> Result<()> {
        unimplemented!()
    }
    fn reset_unread_notifications(&self) -> Result<NotificationCount> {
        unimplemented!()
    }
    fn get_space_licence(&self) -> Result<Licence> {
        unimplemented!()
    }
    fn put_space_notification(&self, _content: &str) -> Result<SpaceNotification> {
        unimplemented!()
    }
    fn get_priorities(&self) -> Result<Vec<Priority>> {
        unimplemented!()
    }
    fn get_resolutions(&self) -> Result<Vec<Resolution>> {
        unimplemented!()
    }
    fn get_watchings(&self, _user_id: u64, _params: &[(String, String)]) -> Result<Vec<Watching>> {
        unimplemented!()
    }
    fn count_watchings(
        &self,
        _user_id: u64,
        _params: &[(String, String)],
    ) -> Result<WatchingCount> {
        unimplemented!()
    }
    fn get_watching(&self, _watching_id: u64) -> Result<Watching> {
        unimplemented!()
    }
    fn add_watching(&self, _params: &[(String, String)]) -> Result<Watching> {
        unimplemented!()
    }
    fn update_watching(&self, _watching_id: u64, _params: &[(String, String)]) -> Result<Watching> {
        unimplemented!()
    }
    fn delete_watching(&self, _watching_id: u64) -> Result<Watching> {
        unimplemented!()
    }
    fn read_watching(&self, _watching_id: u64) -> Result<()> {
        unimplemented!()
    }
}

impl BacklogApi for BacklogClient {
    fn get_space(&self) -> Result<Space> {
        self.get_space()
    }

    fn get_rate_limit(&self) -> Result<RateLimit> {
        self.get_rate_limit()
    }

    fn get_myself(&self) -> Result<User> {
        self.get_myself()
    }

    fn get_users(&self) -> Result<Vec<User>> {
        self.get_users()
    }

    fn get_user(&self, user_id: u64) -> Result<User> {
        self.get_user(user_id)
    }

    fn get_space_activities(&self, params: &[(String, String)]) -> Result<Vec<Activity>> {
        self.get_space_activities(params)
    }

    fn get_space_disk_usage(&self) -> Result<DiskUsage> {
        self.get_space_disk_usage()
    }

    fn get_space_notification(&self) -> Result<SpaceNotification> {
        self.get_space_notification()
    }

    fn get_projects(&self) -> Result<Vec<Project>> {
        self.get_projects()
    }

    fn get_project(&self, key: &str) -> Result<Project> {
        self.get_project(key)
    }

    fn get_project_activities(
        &self,
        key: &str,
        params: &[(String, String)],
    ) -> Result<Vec<Activity>> {
        self.get_project_activities(key, params)
    }

    fn get_project_disk_usage(&self, key: &str) -> Result<ProjectDiskUsage> {
        self.get_project_disk_usage(key)
    }

    fn get_project_users(&self, key: &str) -> Result<Vec<ProjectUser>> {
        self.get_project_users(key)
    }

    fn get_project_statuses(&self, key: &str) -> Result<Vec<ProjectStatus>> {
        self.get_project_statuses(key)
    }

    fn add_project_status(&self, key: &str, name: &str, color: &str) -> Result<ProjectStatus> {
        self.add_project_status(key, name, color)
    }

    fn update_project_status(
        &self,
        key: &str,
        status_id: u64,
        params: &[(String, String)],
    ) -> Result<ProjectStatus> {
        self.update_project_status(key, status_id, params)
    }

    fn delete_project_status(
        &self,
        key: &str,
        status_id: u64,
        substitute_status_id: u64,
    ) -> Result<ProjectStatus> {
        self.delete_project_status(key, status_id, substitute_status_id)
    }

    fn reorder_project_statuses(
        &self,
        key: &str,
        status_ids: &[u64],
    ) -> Result<Vec<ProjectStatus>> {
        self.reorder_project_statuses(key, status_ids)
    }

    fn get_project_issue_types(&self, key: &str) -> Result<Vec<ProjectIssueType>> {
        self.get_project_issue_types(key)
    }

    fn add_project_issue_type(
        &self,
        key: &str,
        name: &str,
        color: &str,
    ) -> Result<ProjectIssueType> {
        self.add_project_issue_type(key, name, color)
    }

    fn update_project_issue_type(
        &self,
        key: &str,
        issue_type_id: u64,
        params: &[(String, String)],
    ) -> Result<ProjectIssueType> {
        self.update_project_issue_type(key, issue_type_id, params)
    }

    fn delete_project_issue_type(
        &self,
        key: &str,
        issue_type_id: u64,
        substitute_issue_type_id: u64,
    ) -> Result<ProjectIssueType> {
        self.delete_project_issue_type(key, issue_type_id, substitute_issue_type_id)
    }

    fn get_project_categories(&self, key: &str) -> Result<Vec<ProjectCategory>> {
        self.get_project_categories(key)
    }

    fn add_project_category(&self, key: &str, name: &str) -> Result<ProjectCategory> {
        self.add_project_category(key, name)
    }

    fn update_project_category(
        &self,
        key: &str,
        category_id: u64,
        name: &str,
    ) -> Result<ProjectCategory> {
        self.update_project_category(key, category_id, name)
    }

    fn delete_project_category(&self, key: &str, category_id: u64) -> Result<ProjectCategory> {
        self.delete_project_category(key, category_id)
    }

    fn get_project_versions(&self, key: &str) -> Result<Vec<ProjectVersion>> {
        self.get_project_versions(key)
    }

    fn add_project_version(
        &self,
        key: &str,
        name: &str,
        description: Option<&str>,
        start_date: Option<&str>,
        release_due_date: Option<&str>,
    ) -> Result<ProjectVersion> {
        self.add_project_version(key, name, description, start_date, release_due_date)
    }

    fn update_project_version(
        &self,
        key: &str,
        version_id: u64,
        name: &str,
        description: Option<&str>,
        start_date: Option<&str>,
        release_due_date: Option<&str>,
        archived: Option<bool>,
    ) -> Result<ProjectVersion> {
        self.update_project_version(
            key,
            version_id,
            name,
            description,
            start_date,
            release_due_date,
            archived,
        )
    }

    fn delete_project_version(&self, key: &str, version_id: u64) -> Result<ProjectVersion> {
        self.delete_project_version(key, version_id)
    }

    fn create_project(&self, params: &[(String, String)]) -> Result<Project> {
        self.create_project(params)
    }

    fn update_project(&self, key: &str, params: &[(String, String)]) -> Result<Project> {
        self.update_project(key, params)
    }

    fn delete_project(&self, key: &str) -> Result<Project> {
        self.delete_project(key)
    }

    fn add_project_user(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        self.add_project_user(key, user_id)
    }

    fn delete_project_user(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        self.delete_project_user(key, user_id)
    }

    fn get_project_administrators(&self, key: &str) -> Result<Vec<ProjectUser>> {
        self.get_project_administrators(key)
    }

    fn add_project_administrator(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        self.add_project_administrator(key, user_id)
    }

    fn delete_project_administrator(&self, key: &str, user_id: u64) -> Result<ProjectUser> {
        self.delete_project_administrator(key, user_id)
    }

    fn get_project_custom_fields(&self, key: &str) -> Result<Vec<ProjectCustomField>> {
        self.get_project_custom_fields(key)
    }

    fn add_project_custom_field(
        &self,
        key: &str,
        type_id: u64,
        name: &str,
        description: Option<&str>,
        required: Option<bool>,
    ) -> Result<ProjectCustomField> {
        self.add_project_custom_field(key, type_id, name, description, required)
    }

    fn update_project_custom_field(
        &self,
        key: &str,
        custom_field_id: u64,
        name: Option<&str>,
        description: Option<&str>,
        required: Option<bool>,
    ) -> Result<ProjectCustomField> {
        self.update_project_custom_field(key, custom_field_id, name, description, required)
    }

    fn delete_project_custom_field(
        &self,
        key: &str,
        custom_field_id: u64,
    ) -> Result<ProjectCustomField> {
        self.delete_project_custom_field(key, custom_field_id)
    }

    fn add_project_custom_field_item(
        &self,
        key: &str,
        custom_field_id: u64,
        name: &str,
    ) -> Result<ProjectCustomField> {
        self.add_project_custom_field_item(key, custom_field_id, name)
    }

    fn update_project_custom_field_item(
        &self,
        key: &str,
        custom_field_id: u64,
        item_id: u64,
        name: &str,
    ) -> Result<ProjectCustomField> {
        self.update_project_custom_field_item(key, custom_field_id, item_id, name)
    }

    fn delete_project_custom_field_item(
        &self,
        key: &str,
        custom_field_id: u64,
        item_id: u64,
    ) -> Result<ProjectCustomField> {
        self.delete_project_custom_field_item(key, custom_field_id, item_id)
    }

    fn get_issues(&self, params: &[(String, String)]) -> Result<Vec<Issue>> {
        self.get_issues(params)
    }

    fn count_issues(&self, params: &[(String, String)]) -> Result<IssueCount> {
        self.count_issues(params)
    }

    fn get_issue(&self, key: &str) -> Result<Issue> {
        self.get_issue(key)
    }

    fn create_issue(&self, params: &[(String, String)]) -> Result<Issue> {
        self.create_issue(params)
    }

    fn update_issue(&self, key: &str, params: &[(String, String)]) -> Result<Issue> {
        self.update_issue(key, params)
    }

    fn delete_issue(&self, key: &str) -> Result<Issue> {
        self.delete_issue(key)
    }

    fn get_issue_comments(&self, key: &str) -> Result<Vec<IssueComment>> {
        self.get_issue_comments(key)
    }

    fn add_issue_comment(&self, key: &str, params: &[(String, String)]) -> Result<IssueComment> {
        self.add_issue_comment(key, params)
    }

    fn update_issue_comment(
        &self,
        key: &str,
        comment_id: u64,
        params: &[(String, String)],
    ) -> Result<IssueComment> {
        self.update_issue_comment(key, comment_id, params)
    }

    fn delete_issue_comment(&self, key: &str, comment_id: u64) -> Result<IssueComment> {
        self.delete_issue_comment(key, comment_id)
    }

    fn get_issue_attachments(&self, key: &str) -> Result<Vec<IssueAttachment>> {
        self.get_issue_attachments(key)
    }

    fn delete_issue_attachment(&self, key: &str, attachment_id: u64) -> Result<IssueAttachment> {
        self.delete_issue_attachment(key, attachment_id)
    }

    fn download_issue_attachment(
        &self,
        key: &str,
        attachment_id: u64,
    ) -> Result<(Vec<u8>, String)> {
        self.download_issue_attachment(key, attachment_id)
    }

    fn get_issue_participants(&self, key: &str) -> Result<Vec<IssueParticipant>> {
        self.get_issue_participants(key)
    }

    fn get_issue_shared_files(&self, key: &str) -> Result<Vec<IssueSharedFile>> {
        self.get_issue_shared_files(key)
    }

    fn link_issue_shared_files(
        &self,
        key: &str,
        shared_file_ids: &[u64],
    ) -> Result<Vec<IssueSharedFile>> {
        self.link_issue_shared_files(key, shared_file_ids)
    }

    fn unlink_issue_shared_file(&self, key: &str, shared_file_id: u64) -> Result<IssueSharedFile> {
        self.unlink_issue_shared_file(key, shared_file_id)
    }

    fn count_issue_comments(&self, key: &str) -> Result<IssueCommentCount> {
        self.count_issue_comments(key)
    }

    fn get_issue_comment(&self, key: &str, comment_id: u64) -> Result<IssueComment> {
        self.get_issue_comment(key, comment_id)
    }

    fn get_issue_comment_notifications(
        &self,
        key: &str,
        comment_id: u64,
    ) -> Result<Vec<IssueCommentNotification>> {
        self.get_issue_comment_notifications(key, comment_id)
    }

    fn add_issue_comment_notifications(
        &self,
        key: &str,
        comment_id: u64,
        params: &[(String, String)],
    ) -> Result<Vec<IssueCommentNotification>> {
        self.add_issue_comment_notifications(key, comment_id, params)
    }

    fn get_wikis(&self, params: &[(String, String)]) -> Result<Vec<WikiListItem>> {
        self.get_wikis(params)
    }

    fn get_wiki(&self, wiki_id: u64) -> Result<Wiki> {
        self.get_wiki(wiki_id)
    }

    fn create_wiki(&self, params: &[(String, String)]) -> Result<Wiki> {
        self.create_wiki(params)
    }

    fn update_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki> {
        self.update_wiki(wiki_id, params)
    }

    fn delete_wiki(&self, wiki_id: u64, params: &[(String, String)]) -> Result<Wiki> {
        self.delete_wiki(wiki_id, params)
    }

    fn get_wiki_history(&self, wiki_id: u64) -> Result<Vec<WikiHistory>> {
        self.get_wiki_history(wiki_id)
    }

    fn get_wiki_attachments(&self, wiki_id: u64) -> Result<Vec<WikiAttachment>> {
        self.get_wiki_attachments(wiki_id)
    }

    fn get_teams(&self, params: &[(String, String)]) -> Result<Vec<Team>> {
        self.get_teams(params)
    }

    fn get_team(&self, team_id: u64) -> Result<Team> {
        self.get_team(team_id)
    }

    fn get_user_activities(
        &self,
        user_id: u64,
        params: &[(String, String)],
    ) -> Result<Vec<Activity>> {
        self.get_user_activities(user_id, params)
    }

    fn get_recently_viewed_issues(
        &self,
        params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedIssue>> {
        self.get_recently_viewed_issues(params)
    }

    fn add_user(&self, params: &[(String, String)]) -> Result<User> {
        self.add_user(params)
    }

    fn update_user(&self, user_id: u64, params: &[(String, String)]) -> Result<User> {
        self.update_user(user_id, params)
    }

    fn delete_user(&self, user_id: u64) -> Result<User> {
        self.delete_user(user_id)
    }

    fn get_recently_viewed_projects(
        &self,
        params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedProject>> {
        self.get_recently_viewed_projects(params)
    }

    fn get_recently_viewed_wikis(
        &self,
        params: &[(String, String)],
    ) -> Result<Vec<RecentlyViewedWiki>> {
        self.get_recently_viewed_wikis(params)
    }

    fn get_user_stars(&self, user_id: u64, params: &[(String, String)]) -> Result<Vec<Star>> {
        self.get_user_stars(user_id, params)
    }

    fn count_user_stars(&self, user_id: u64, params: &[(String, String)]) -> Result<StarCount> {
        self.count_user_stars(user_id, params)
    }

    fn get_notifications(&self, params: &[(String, String)]) -> Result<Vec<Notification>> {
        self.get_notifications(params)
    }

    fn count_notifications(&self) -> Result<NotificationCount> {
        self.count_notifications()
    }

    fn read_notification(&self, id: u64) -> Result<()> {
        self.read_notification(id)
    }

    fn reset_unread_notifications(&self) -> Result<NotificationCount> {
        self.reset_unread_notifications()
    }

    fn get_space_licence(&self) -> Result<Licence> {
        self.get_space_licence()
    }

    fn put_space_notification(&self, content: &str) -> Result<SpaceNotification> {
        self.put_space_notification(content)
    }
    fn get_priorities(&self) -> Result<Vec<Priority>> {
        self.get_priorities()
    }
    fn get_resolutions(&self) -> Result<Vec<Resolution>> {
        self.get_resolutions()
    }
    fn get_watchings(&self, user_id: u64, params: &[(String, String)]) -> Result<Vec<Watching>> {
        self.get_watchings(user_id, params)
    }
    fn count_watchings(&self, user_id: u64, params: &[(String, String)]) -> Result<WatchingCount> {
        self.count_watchings(user_id, params)
    }
    fn get_watching(&self, watching_id: u64) -> Result<Watching> {
        self.get_watching(watching_id)
    }
    fn add_watching(&self, params: &[(String, String)]) -> Result<Watching> {
        self.add_watching(params)
    }
    fn update_watching(&self, watching_id: u64, params: &[(String, String)]) -> Result<Watching> {
        self.update_watching(watching_id, params)
    }
    fn delete_watching(&self, watching_id: u64) -> Result<Watching> {
        self.delete_watching(watching_id)
    }
    fn read_watching(&self, watching_id: u64) -> Result<()> {
        self.read_watching(watching_id)
    }
}

/// How the client authenticates with Backlog.
pub(crate) enum AuthMethod {
    /// API key passed as `?apiKey=` query parameter.
    ApiKey(String),
    /// OAuth 2.0 Bearer token.  Both tokens are wrapped in `RefCell` so that
    /// `try_refresh` can update them through a shared `&self` reference.
    Bearer {
        access_token: RefCell<String>,
        refresh_token: RefCell<String>,
        client_id: String,
        client_secret: String,
        space_key: String,
    },
}

pub struct BacklogClient {
    client: Client,
    base_url: String,
    pub(crate) auth: AuthMethod,
}

impl BacklogClient {
    pub fn from_config() -> Result<Self> {
        let space_key = crate::config::current_space_key()?;
        let client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .context("Failed to build HTTP client")?;
        let base_url = format!("https://{space_key}.backlog.com/api/v2");

        // OAuth tokens take priority over API key.
        if let Ok((tokens, backend)) = crate::secret::get_oauth_tokens(&space_key) {
            crate::logger::verbose(&format!("Authenticated via OAuth (Bearer) [{backend}]"));
            return Ok(Self {
                client,
                base_url,
                auth: AuthMethod::Bearer {
                    access_token: RefCell::new(tokens.access_token),
                    refresh_token: RefCell::new(tokens.refresh_token),
                    client_id: tokens.client_id,
                    client_secret: tokens.client_secret,
                    space_key,
                },
            });
        }

        let (api_key, backend) = crate::secret::current_api_key(&space_key)?;
        crate::logger::verbose(&format!("Authenticated via API key [{backend}]"));
        Ok(Self {
            client,
            base_url,
            auth: AuthMethod::ApiKey(api_key),
        })
    }

    /// Apply authentication to a request builder (query param or header).
    fn apply_auth(
        &self,
        builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        match &self.auth {
            AuthMethod::ApiKey(key) => builder.query(&[("apiKey", key.as_str())]),
            AuthMethod::Bearer { access_token, .. } => {
                builder.header("Authorization", format!("Bearer {}", access_token.borrow()))
            }
        }
    }

    /// Parse a successful or error response body.
    fn finish_response(&self, response: reqwest::blocking::Response) -> Result<serde_json::Value> {
        let status = response.status();
        crate::logger::verbose(&format!("← {status}"));
        if status == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::Value::Null);
        }
        let body: serde_json::Value = response.json().context("Failed to parse JSON response")?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }
        Ok(body)
    }

    /// Try to refresh an OAuth access token.  Returns `true` if the token was
    /// refreshed successfully, `false` if auth is API-key-based.
    /// Propagates refresh errors so callers receive an informative error
    /// instead of silently falling back to a stale 401.
    fn try_refresh(&self) -> Result<bool> {
        let AuthMethod::Bearer {
            access_token,
            refresh_token,
            client_id,
            client_secret,
            space_key,
        } = &self.auth
        else {
            return Ok(false);
        };

        let current = crate::oauth::OAuthTokens {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            access_token: access_token.borrow().clone(),
            refresh_token: refresh_token.borrow().clone(),
        };

        let new_tokens = crate::oauth::refresh_access_token(space_key, &current)?;
        *access_token.borrow_mut() = new_tokens.access_token.clone();
        *refresh_token.borrow_mut() = new_tokens.refresh_token.clone();
        crate::secret::set_oauth_tokens(space_key, &new_tokens)?;
        Ok(true)
    }

    /// Send a request (built by `factory`) and retry once on 401 by refreshing
    /// the OAuth token.  `factory` is called at most twice.
    fn execute<F>(&self, factory: F) -> Result<serde_json::Value>
    where
        F: Fn() -> Result<reqwest::blocking::Response>,
    {
        let response = factory()?;
        if response.status() == reqwest::StatusCode::UNAUTHORIZED && self.try_refresh()? {
            return self.finish_response(factory()?);
        }
        self.finish_response(response)
    }

    pub fn get(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        self.execute(|| {
            crate::logger::verbose(&format!("→ GET {url}"));
            self.apply_auth(self.client.get(&url))
                .send()
                .with_context(|| format!("Failed to GET {url}"))
        })
    }

    pub fn get_with_query(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let extra: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        self.execute(|| {
            crate::logger::verbose(&format!("→ GET {url}"));
            self.apply_auth(self.client.get(&url))
                .query(&extra)
                .send()
                .with_context(|| format!("Failed to GET {url}"))
        })
    }

    pub fn post_form(&self, path: &str, params: &[(String, String)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        self.execute(|| {
            crate::logger::verbose(&format!("→ POST {url}"));
            self.apply_auth(self.client.post(&url))
                .form(params)
                .send()
                .with_context(|| format!("Failed to POST {url}"))
        })
    }

    pub fn patch_form(&self, path: &str, params: &[(String, String)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        self.execute(|| {
            crate::logger::verbose(&format!("→ PATCH {url}"));
            self.apply_auth(self.client.patch(&url))
                .form(params)
                .send()
                .with_context(|| format!("Failed to PATCH {url}"))
        })
    }

    pub fn put_form(&self, path: &str, params: &[(String, String)]) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        self.execute(|| {
            crate::logger::verbose(&format!("→ PUT {url}"));
            self.apply_auth(self.client.put(&url))
                .form(params)
                .send()
                .with_context(|| format!("Failed to PUT {url}"))
        })
    }

    pub fn delete_form(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        self.execute(|| {
            crate::logger::verbose(&format!("→ DELETE {url}"));
            self.apply_auth(self.client.delete(&url))
                .form(params)
                .send()
                .with_context(|| format!("Failed to DELETE {url}"))
        })
    }

    /// Send a download request and retry once on 401.
    fn execute_download<F>(&self, factory: F) -> Result<(Vec<u8>, String)>
    where
        F: Fn() -> Result<reqwest::blocking::Response>,
    {
        let response = factory()?;
        if response.status() == reqwest::StatusCode::UNAUTHORIZED && self.try_refresh()? {
            return Self::finish_download(factory()?);
        }
        Self::finish_download(response)
    }

    fn finish_download(response: reqwest::blocking::Response) -> Result<(Vec<u8>, String)> {
        let status = response.status();
        if !status.is_success() {
            let body: serde_json::Value =
                response.json().context("Failed to parse error response")?;
            anyhow::bail!("API error ({}): {}", status, extract_error_message(&body));
        }
        let filename = response
            .headers()
            .get("content-disposition")
            .and_then(|v| v.to_str().ok())
            .and_then(parse_content_disposition_filename)
            .unwrap_or_else(|| "attachment".to_string());
        let bytes = response
            .bytes()
            .context("Failed to read response bytes")?
            .to_vec();
        Ok((bytes, filename))
    }

    pub fn download(&self, path: &str) -> Result<(Vec<u8>, String)> {
        let url = format!("{}{}", self.base_url, path);
        self.execute_download(|| {
            crate::logger::verbose(&format!("→ GET {url}"));
            self.apply_auth(self.client.get(&url))
                .send()
                .with_context(|| format!("Failed to GET {url}"))
        })
    }

    pub fn delete_req(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        self.execute(|| {
            crate::logger::verbose(&format!("→ DELETE {url}"));
            self.apply_auth(self.client.delete(&url))
                .send()
                .with_context(|| format!("Failed to DELETE {url}"))
        })
    }
}

fn parse_content_disposition_filename(header: &str) -> Option<String> {
    let mut ext_filename: Option<String> = None;
    let mut plain_filename: Option<String> = None;
    for part in header.split(';') {
        let part = part.trim();
        if let Some(rest) = part.strip_prefix("filename*=") {
            // RFC 5987: charset'language'percent-encoded-value
            if let Some(encoded) = rest.splitn(3, '\'').nth(2) {
                let decoded = percent_decode(encoded);
                if !decoded.is_empty() {
                    ext_filename = Some(decoded);
                }
            }
        } else if let Some(rest) = part.strip_prefix("filename=") {
            let name = rest.trim_matches('"');
            if !name.is_empty() {
                plain_filename = Some(name.to_string());
            }
        }
    }
    ext_filename.or(plain_filename)
}

fn percent_decode(s: &str) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(h1), Some(h2)) = (h1, h2) {
                if let Ok(b) = u8::from_str_radix(&format!("{h1}{h2}"), 16) {
                    bytes.push(b);
                    continue;
                }
                // Invalid hex — push literal characters
                bytes.push(b'%');
                for byte in h1.to_string().bytes().chain(h2.to_string().bytes()) {
                    bytes.push(byte);
                }
                continue;
            } else {
                // Truncated — push '%' and any consumed char
                bytes.push(b'%');
                if let Some(h1) = h1 {
                    for byte in h1.to_string().bytes() {
                        bytes.push(byte);
                    }
                }
                continue;
            }
        }
        for byte in c.to_string().bytes() {
            bytes.push(byte);
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn extract_error_message(body: &serde_json::Value) -> &str {
    body.get("errors")
        .and_then(|e| e.get(0))
        .and_then(|e| e.get("message"))
        .and_then(|m| m.as_str())
        .unwrap_or("Unknown error")
}

impl BacklogClient {
    pub(crate) fn new_with(base_url: &str, api_key: &str) -> Result<Self> {
        let client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .context("Failed to build HTTP client")?;
        Ok(Self {
            client,
            base_url: base_url.to_string(),
            auth: AuthMethod::ApiKey(api_key.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    const TEST_KEY: &str = "test-api-key";

    #[test]
    fn get_returns_body_on_success() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/space")
                .query_param("apiKey", TEST_KEY);
            then.status(200)
                .json_body(json!({"spaceKey": "mycompany", "name": "My Company"}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let body = client.get("/space").unwrap();
        assert_eq!(body["spaceKey"], "mycompany");
    }

    #[test]
    fn get_returns_error_with_api_message_on_failure() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(401)
                .json_body(json!({"errors": [{"message": "Authentication failure"}]}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get("/space").unwrap_err();
        assert!(err.to_string().contains("Authentication failure"));
    }

    #[test]
    fn get_returns_error_with_fallback_message_on_unknown_error() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/space");
            then.status(500).json_body(json!({}));
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let err = client.get("/space").unwrap_err();
        assert!(err.to_string().contains("Unknown error"));
    }

    #[test]
    fn extract_error_message_from_errors_array() {
        let body = json!({"errors": [{"message": "Authentication failure"}]});
        assert_eq!(extract_error_message(&body), "Authentication failure");
    }

    #[test]
    fn extract_error_message_fallback_when_missing() {
        let body = json!({});
        assert_eq!(extract_error_message(&body), "Unknown error");
    }

    #[test]
    fn extract_error_message_fallback_when_empty_errors() {
        let body = json!({"errors": []});
        assert_eq!(extract_error_message(&body), "Unknown error");
    }

    #[test]
    fn post_form_returns_null_on_204_no_content() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST)
                .path("/notifications/123/markAsRead")
                .query_param("apiKey", TEST_KEY);
            then.status(204);
        });

        let client = BacklogClient::new_with(&server.base_url(), TEST_KEY).unwrap();
        let body = client
            .post_form("/notifications/123/markAsRead", &[])
            .unwrap();
        assert_eq!(body, serde_json::Value::Null);
    }
}
