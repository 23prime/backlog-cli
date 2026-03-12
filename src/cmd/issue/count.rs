use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient};
use crate::cmd::issue::ParentChild;

pub struct IssueCountArgs {
    project_ids: Vec<u64>,
    status_ids: Vec<u64>,
    assignee_ids: Vec<u64>,
    issue_type_ids: Vec<u64>,
    category_ids: Vec<u64>,
    milestone_ids: Vec<u64>,
    parent_child: Option<ParentChild>,
    keyword: Option<String>,
    json: bool,
}

impl IssueCountArgs {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_ids: Vec<u64>,
        status_ids: Vec<u64>,
        assignee_ids: Vec<u64>,
        issue_type_ids: Vec<u64>,
        category_ids: Vec<u64>,
        milestone_ids: Vec<u64>,
        parent_child: Option<ParentChild>,
        keyword: Option<String>,
        json: bool,
    ) -> Self {
        Self {
            project_ids,
            status_ids,
            assignee_ids,
            issue_type_ids,
            category_ids,
            milestone_ids,
            parent_child,
            keyword,
            json,
        }
    }
}

pub fn count(args: &IssueCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &IssueCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let params = build_params(args);
    let result = api.count_issues(&params)?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", result.count);
    }
    Ok(())
}

fn build_params(args: &IssueCountArgs) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    for id in &args.project_ids {
        params.push(("projectId[]".to_string(), id.to_string()));
    }
    for id in &args.status_ids {
        params.push(("statusId[]".to_string(), id.to_string()));
    }
    for id in &args.assignee_ids {
        params.push(("assigneeId[]".to_string(), id.to_string()));
    }
    for id in &args.issue_type_ids {
        params.push(("issueTypeId[]".to_string(), id.to_string()));
    }
    for id in &args.category_ids {
        params.push(("categoryId[]".to_string(), id.to_string()));
    }
    for id in &args.milestone_ids {
        params.push(("milestoneId[]".to_string(), id.to_string()));
    }
    if let Some(pc) = &args.parent_child {
        params.push(("parentChild".to_string(), pc.to_api_value().to_string()));
    }
    if let Some(kw) = &args.keyword {
        params.push(("keyword".to_string(), kw.clone()));
    }
    params
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::{Issue, IssueAttachment, IssueComment, IssueCount};
    use anyhow::anyhow;

    struct MockApi {
        count: Option<u64>,
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
        fn get_space_activities(&self) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
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
        fn get_issues(&self, _params: &[(String, String)]) -> anyhow::Result<Vec<Issue>> {
            unimplemented!()
        }
        fn count_issues(&self, _params: &[(String, String)]) -> anyhow::Result<IssueCount> {
            self.count
                .map(|c| IssueCount { count: c })
                .ok_or_else(|| anyhow!("no count"))
        }
        fn get_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn create_issue(&self, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn update_issue(&self, _key: &str, _params: &[(String, String)]) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _key: &str) -> anyhow::Result<Issue> {
            unimplemented!()
        }
        fn get_issue_comments(&self, _key: &str) -> anyhow::Result<Vec<IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(&self, _key: &str) -> anyhow::Result<Vec<IssueAttachment>> {
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
        fn get_teams(&self) -> anyhow::Result<Vec<crate::api::team::Team>> {
            unimplemented!()
        }
        fn get_team(&self, _team_id: u64) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn get_user_activities(
            &self,
            _user_id: u64,
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(
            &self,
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedIssue>> {
            unimplemented!()
        }
    }

    fn args(json: bool) -> IssueCountArgs {
        IssueCountArgs::new(
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            None,
            None,
            json,
        )
    }

    #[test]
    fn count_with_text_output_succeeds() {
        let api = MockApi { count: Some(42) };
        assert!(count_with(&args(false), &api).is_ok());
    }

    #[test]
    fn count_with_json_output_succeeds() {
        let api = MockApi { count: Some(0) };
        assert!(count_with(&args(true), &api).is_ok());
    }

    #[test]
    fn count_with_propagates_api_error() {
        let api = MockApi { count: None };
        let err = count_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no count"));
    }
}
