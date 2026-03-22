mod activities;
pub mod admin;
pub mod category;
mod create;
pub mod custom_field;
mod delete;
mod disk_usage;
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
pub use list::{ProjectListArgs, list};
pub use show::{ProjectShowArgs, show};
pub use update::{ProjectUpdateArgs, update};

pub(crate) fn format_project_user_row(u: &crate::api::project::ProjectUser) -> String {
    match u.user_id.as_deref() {
        Some(user_id) if !user_id.is_empty() => format!("[{}] {}", user_id, u.name),
        _ => format!("[{}] {}", u.id, u.name),
    }
}
