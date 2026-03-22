use crate::api::activity::Activity;

pub(crate) fn validate_activity_query(
    count: u32,
    min_id: Option<u64>,
    max_id: Option<u64>,
) -> anyhow::Result<()> {
    if !(1..=100).contains(&count) {
        anyhow::bail!("count must be between 1 and 100");
    }
    if let (Some(min), Some(max)) = (min_id, max_id)
        && min > max
    {
        anyhow::bail!("min-id must be less than or equal to max-id");
    }
    Ok(())
}

pub(crate) fn build_activity_params(
    activity_type_ids: &[u32],
    min_id: Option<u64>,
    max_id: Option<u64>,
    count: u32,
    order: Option<&str>,
) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    for id in activity_type_ids {
        params.push(("activityTypeId[]".to_string(), id.to_string()));
    }
    if let Some(min) = min_id {
        params.push(("minId".to_string(), min.to_string()));
    }
    if let Some(max) = max_id {
        params.push(("maxId".to_string(), max.to_string()));
    }
    params.push(("count".to_string(), count.to_string()));
    if let Some(order) = order {
        params.push(("order".to_string(), order.to_string()));
    }
    params
}

pub(crate) fn format_activity_row(a: &Activity) -> String {
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
