mod activities;
pub mod admin;
pub mod category;
mod create;
pub mod custom_field;
mod delete;
mod disk_usage;
mod image;
pub mod issue_type;
mod list;
mod show;
pub mod status;
pub mod team;
mod update;
pub mod user;
pub mod version;
pub mod webhook;

pub use activities::{ProjectActivitiesArgs, activities};
pub use create::{ProjectCreateArgs, create};
pub use delete::{ProjectDeleteArgs, delete};
pub use disk_usage::{ProjectDiskUsageArgs, disk_usage};
pub use image::{ProjectImageArgs, image};
pub use list::{ProjectListArgs, list};
pub use show::{ProjectShowArgs, show};
pub use update::{ProjectUpdateArgs, update};

pub(crate) fn format_project_user_row(u: &crate::api::project::ProjectUser) -> String {
    match u.user_id.as_deref() {
        Some(user_id) if !user_id.is_empty() => format!("[{}] {}", user_id, u.name),
        _ => format!("[{}] {}", u.id, u.name),
    }
}

#[cfg(test)]
pub(crate) fn sample_project() -> crate::api::project::Project {
    use std::collections::BTreeMap;
    crate::api::project::Project {
        id: 1,
        project_key: "TEST".to_string(),
        name: "Test Project".to_string(),
        chart_enabled: false,
        subtasking_enabled: false,
        project_leader_can_edit_project_leader: false,
        text_formatting_rule: "markdown".to_string(),
        archived: false,
        extra: BTreeMap::new(),
    }
}

#[cfg(test)]
pub(crate) fn sample_project_user() -> crate::api::project::ProjectUser {
    use std::collections::BTreeMap;
    crate::api::project::ProjectUser {
        id: 1,
        user_id: Some("john".to_string()),
        name: "John Doe".to_string(),
        role_type: 1,
        lang: Some("ja".to_string()),
        mail_address: Some("john@example.com".to_string()),
        last_login_time: None,
        extra: BTreeMap::new(),
    }
}
