use anstream::println;
use anyhow::{Context, Result};

use crate::api::{
    BacklogApi, BacklogClient,
    activity::{build_activity_params, format_activity_row},
};

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
    let params = build_activity_params(
        &args.activity_type_ids,
        args.min_id,
        args.max_id,
        args.count,
        args.order.as_deref(),
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::activity::{Activity, ActivityUser};
    use anyhow::anyhow;

    struct MockApi {
        activities: Option<Vec<Activity>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_user_activities(
            &self,
            _user_id: u64,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<Activity>> {
            self.activities
                .clone()
                .ok_or_else(|| anyhow!("no activities"))
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
        fn get_user_activities(
            &self,
            _user_id: u64,
            params: &[(String, String)],
        ) -> anyhow::Result<Vec<Activity>> {
            *self.captured.borrow_mut() = params.to_vec();
            Ok(vec![])
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
