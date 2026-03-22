mod add;
mod delete;
mod icon;
mod list;
mod show;
mod update;

pub use add::{TeamAddArgs, add};
pub use delete::{TeamDeleteArgs, delete};
pub use icon::{TeamIconArgs, icon};
pub use list::{TeamListArgs, list};
pub use show::{TeamShowArgs, show};
pub use update::{TeamUpdateArgs, update};

pub(crate) fn format_team_row(t: &crate::api::team::Team) -> String {
    format!("[{}] {} ({} members)", t.id, t.name, t.members.len())
}
