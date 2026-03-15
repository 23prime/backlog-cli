use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::user::show::format_user_text;

pub struct UserUpdateArgs {
    user_id: u64,
    name: Option<String>,
    password: Option<String>,
    mail_address: Option<String>,
    role_type: Option<u8>,
    json: bool,
}

impl UserUpdateArgs {
    pub fn try_new(
        user_id: u64,
        name: Option<String>,
        password: Option<String>,
        mail_address: Option<String>,
        role_type: Option<u8>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if name.is_none() && password.is_none() && mail_address.is_none() && role_type.is_none() {
            anyhow::bail!(
                "at least one of --name, --password, --mail-address, or --role-type must be specified"
            );
        }
        Ok(Self {
            user_id,
            name,
            password,
            mail_address,
            role_type,
            json,
        })
    }
}

pub fn update(args: &UserUpdateArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    update_with(args, &client)
}

pub fn update_with(args: &UserUpdateArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    if let Some(ref name) = args.name {
        params.push(("name".to_string(), name.clone()));
    }
    if let Some(ref password) = args.password {
        params.push(("password".to_string(), password.clone()));
    }
    if let Some(ref mail) = args.mail_address {
        params.push(("mailAddress".to_string(), mail.clone()));
    }
    if let Some(role) = args.role_type {
        params.push(("roleType".to_string(), role.to_string()));
    }
    let user = api.update_user(args.user_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&user).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_user_text(&user));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::User;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        user: Option<User>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn get_users(&self) -> anyhow::Result<Vec<User>> {
            unimplemented!()
        }
        fn get_user(&self, _user_id: u64) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn add_user(&self, _params: &[(String, String)]) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn update_user(&self, _user_id: u64, _params: &[(String, String)]) -> anyhow::Result<User> {
            self.user.clone().ok_or_else(|| anyhow!("no user"))
        }
        fn delete_user(&self, _user_id: u64) -> anyhow::Result<User> {
            unimplemented!()
        }
        fn get_space_activities(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_space_disk_usage(&self) -> anyhow::Result<crate::api::disk_usage::DiskUsage> {
            unimplemented!()
        }
        fn get_space_notification(
            &self,
        ) -> anyhow::Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
        }
        fn get_projects(&self) -> anyhow::Result<Vec<crate::api::project::Project>> {
            unimplemented!()
        }
        fn get_project(&self, _key: &str) -> anyhow::Result<crate::api::project::Project> {
            unimplemented!()
        }
        fn get_project_activities(
            &self,
            _key: &str,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(
            &self,
            _key: &str,
        ) -> anyhow::Result<crate::api::project::ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectCategory>> {
            unimplemented!()
        }
        fn get_project_versions(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectVersion>> {
            unimplemented!()
        }
        fn get_issues(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::issue::Issue>> {
            unimplemented!()
        }
        fn count_issues(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _key: &str) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn create_issue(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn update_issue(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _key: &str) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn get_issue_comments(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueAttachment>> {
            unimplemented!()
        }
        fn get_wikis(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiListItem>> {
            unimplemented!()
        }
        fn get_wiki(&self, _wiki_id: u64) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn create_wiki(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn update_wiki(
            &self,
            _wiki_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn delete_wiki(
            &self,
            _wiki_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn get_wiki_history(
            &self,
            _wiki_id: u64,
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiHistory>> {
            unimplemented!()
        }
        fn get_wiki_attachments(
            &self,
            _wiki_id: u64,
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiAttachment>> {
            unimplemented!()
        }
        fn get_teams(&self, _: &[(String, String)]) -> anyhow::Result<Vec<crate::api::team::Team>> {
            unimplemented!()
        }
        fn get_team(&self, _team_id: u64) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn get_user_activities(
            &self,
            _user_id: u64,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedIssue>> {
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
        fn get_notifications(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::notification::Notification>> {
            unimplemented!()
        }
        fn count_notifications(
            &self,
        ) -> anyhow::Result<crate::api::notification::NotificationCount> {
            unimplemented!()
        }
        fn read_notification(&self, _: u64) -> anyhow::Result<()> {
            unimplemented!()
        }
        fn reset_unread_notifications(
            &self,
        ) -> anyhow::Result<crate::api::notification::NotificationCount> {
            unimplemented!()
        }
    }

    fn sample_user() -> User {
        User {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            mail_address: Some("john@example.com".to_string()),
            role_type: 1,
            lang: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn try_new_rejects_no_fields() {
        assert!(UserUpdateArgs::try_new(1, None, None, None, None, false).is_err());
    }

    #[test]
    fn try_new_accepts_at_least_one_field() {
        assert!(
            UserUpdateArgs::try_new(1, Some("New Name".to_string()), None, None, None, false)
                .is_ok()
        );
    }

    #[test]
    fn update_with_text_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let args =
            UserUpdateArgs::try_new(1, Some("New Name".to_string()), None, None, None, false)
                .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_json_output_succeeds() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let args = UserUpdateArgs::try_new(1, Some("New Name".to_string()), None, None, None, true)
            .unwrap();
        assert!(update_with(&args, &api).is_ok());
    }

    #[test]
    fn update_with_propagates_api_error() {
        let api = MockApi { user: None };
        let args =
            UserUpdateArgs::try_new(1, Some("New Name".to_string()), None, None, None, false)
                .unwrap();
        let err = update_with(&args, &api).unwrap_err();
        assert!(err.to_string().contains("no user"));
    }
}
