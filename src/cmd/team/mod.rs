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

#[cfg(test)]
pub(crate) fn sample_member() -> crate::api::team::TeamMember {
    use std::collections::BTreeMap;
    crate::api::team::TeamMember {
        id: 2,
        user_id: Some("dev".to_string()),
        name: "Developer".to_string(),
        role_type: 2,
        lang: None,
        mail_address: None,
        last_login_time: None,
        extra: BTreeMap::new(),
    }
}

#[cfg(test)]
pub(crate) fn sample_team() -> crate::api::team::Team {
    use std::collections::BTreeMap;
    crate::api::team::Team {
        id: 1,
        name: "dev-team".to_string(),
        members: vec![sample_member()],
        display_order: None,
        created: "2024-01-01T00:00:00Z".to_string(),
        updated: "2024-01-01T00:00:00Z".to_string(),
        extra: BTreeMap::new(),
    }
}
