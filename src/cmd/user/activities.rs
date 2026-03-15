use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, activity::Activity};

pub struct UserActivitiesArgs {
    user_id: u64,
    json: bool,
    pub activity_type_ids: Vec<u32>,
    pub min_id: Option<u64>,
    pub max_id: Option<u64>,
    pub count: u32,
    pub order: Option<String>,
}

impl UserActivitiesArgs {
    pub fn try_new(
        user_id: u64,
        json: bool,
        activity_type_ids: Vec<u32>,
        min_id: Option<u64>,
        max_id: Option<u64>,
        count: u32,
        order: Option<String>,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        if let (Some(min), Some(max)) = (min_id, max_id)
            && min > max
        {
            anyhow::bail!("min-id must be less than or equal to max-id");
        }
        Ok(Self {
            user_id,
            json,
            activity_type_ids,
            min_id,
            max_id,
            count,
            order,
        })
    }
}

pub fn activities(args: &UserActivitiesArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    activities_with(args, &client)
}

pub fn activities_with(args: &UserActivitiesArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    for id in &args.activity_type_ids {
        params.push(("activityTypeId[]".to_string(), id.to_string()));
    }
    if let Some(min) = args.min_id {
        params.push(("minId".to_string(), min.to_string()));
    }
    if let Some(max) = args.max_id {
        params.push(("maxId".to_string(), max.to_string()));
    }
    params.push(("count".to_string(), args.count.to_string()));
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    let activities = api.get_user_activities(args.user_id, &params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&activities).context("Failed to serialize JSON")?
        );
    } else {
        for a in &activities {
            println!("{}", format_activity_row(a));
        }
    }
    Ok(())
}

fn format_activity_row(a: &Activity) -> String {
    let project = a
        .project
        .as_ref()
        .map(|p| p.project_key.as_str())
        .unwrap_or("-");
    format!(
        "[{}] type={} project={} user={} created={}",
        a.id, a.activity_type, project, a.created_user.name, a.created,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::activity::ActivityUser;
    use anyhow::anyhow;

    struct MockApi {
        activities: Option<Vec<Activity>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_users(&self) -> anyhow::Result<Vec<crate::api::user::User>> {
            unimplemented!()
        }
        fn get_user(&self, _user_id: u64) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(&self, _: &[(String, String)]) -> anyhow::Result<Vec<Activity>> {
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
        ) -> anyhow::Result<Vec<Activity>> {
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
        ) -> anyhow::Result<Vec<Activity>> {
            self.activities
                .clone()
                .ok_or_else(|| anyhow!("no activities"))
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedIssue>> {
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
        fn get_space_licence(&self) -> anyhow::Result<crate::api::licence::Licence> {
            unimplemented!()
        }
        fn put_space_notification(&self, _content: &str) -> anyhow::Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
        }
    }

    fn sample_activity() -> Activity {
        Activity {
            id: 10,
            project: None,
            activity_type: 2,
            content: serde_json::Value::Null,
            created_user: ActivityUser {
                id: 1,
                user_id: Some("john".to_string()),
                name: "John Doe".to_string(),
                extra: Default::default(),
            },
            created: "2024-06-01T00:00:00Z".to_string(),
            extra: Default::default(),
        }
    }

    #[test]
    fn format_activity_row_contains_fields() {
        let text = format_activity_row(&sample_activity());
        assert!(text.contains("[10]"));
        assert!(text.contains("type=2"));
        assert!(text.contains("project=-"));
        assert!(text.contains("John Doe"));
        assert!(text.contains("2024-06-01T00:00:00Z"));
    }

    #[test]
    fn activities_with_text_output_succeeds() {
        let api = MockApi {
            activities: Some(vec![sample_activity()]),
        };
        assert!(
            activities_with(
                &UserActivitiesArgs::try_new(1, false, vec![], None, None, 20, None).unwrap(),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn activities_with_json_output_succeeds() {
        let api = MockApi {
            activities: Some(vec![sample_activity()]),
        };
        assert!(
            activities_with(
                &UserActivitiesArgs::try_new(1, true, vec![], None, None, 20, None).unwrap(),
                &api
            )
            .is_ok()
        );
    }

    #[test]
    fn activities_with_propagates_api_error() {
        let api = MockApi { activities: None };
        let err = activities_with(
            &UserActivitiesArgs::try_new(1, false, vec![], None, None, 20, None).unwrap(),
            &api,
        )
        .unwrap_err();
        assert!(err.to_string().contains("no activities"));
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(UserActivitiesArgs::try_new(1, false, vec![], None, None, 101, None).is_err());
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(UserActivitiesArgs::try_new(1, false, vec![], None, None, 0, None).is_err());
    }

    #[test]
    fn try_new_rejects_min_id_greater_than_max_id() {
        assert!(
            UserActivitiesArgs::try_new(1, false, vec![], Some(20), Some(10), 20, None).is_err()
        );
    }

    struct MockApiCapture {
        captured: std::cell::RefCell<Vec<(String, String)>>,
    }

    impl crate::api::BacklogApi for MockApiCapture {
        fn get_space(&self) -> anyhow::Result<crate::api::space::Space> {
            unimplemented!()
        }
        fn get_myself(&self) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_users(&self) -> anyhow::Result<Vec<crate::api::user::User>> {
            unimplemented!()
        }
        fn get_user(&self, _user_id: u64) -> anyhow::Result<crate::api::user::User> {
            unimplemented!()
        }
        fn get_space_activities(&self, _: &[(String, String)]) -> anyhow::Result<Vec<Activity>> {
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
        ) -> anyhow::Result<Vec<Activity>> {
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
            params: &[(String, String)],
        ) -> anyhow::Result<Vec<Activity>> {
            *self.captured.borrow_mut() = params.to_vec();
            Ok(vec![])
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedIssue>> {
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

    #[test]
    fn activities_with_builds_correct_query_params() {
        let api = MockApiCapture {
            captured: std::cell::RefCell::new(vec![]),
        };
        let args = UserActivitiesArgs::try_new(
            1,
            false,
            vec![1, 2],
            Some(10),
            Some(20),
            50,
            Some("asc".to_string()),
        )
        .unwrap();
        activities_with(&args, &api).unwrap();
        let params = api.captured.borrow();
        assert!(
            params
                .iter()
                .any(|(k, v)| k == "activityTypeId[]" && v == "1")
        );
        assert!(
            params
                .iter()
                .any(|(k, v)| k == "activityTypeId[]" && v == "2")
        );
        assert!(params.iter().any(|(k, v)| k == "minId" && v == "10"));
        assert!(params.iter().any(|(k, v)| k == "maxId" && v == "20"));
        assert!(params.iter().any(|(k, v)| k == "count" && v == "50"));
        assert!(params.iter().any(|(k, v)| k == "order" && v == "asc"));
    }
}
