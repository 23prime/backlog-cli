mod activities;
mod add;
mod delete;
mod icon;
mod list;
mod recently_viewed;
mod recently_viewed_projects;
mod recently_viewed_wikis;
mod show;
pub mod star;
mod update;

pub use activities::{UserActivitiesArgs, activities};
pub use icon::{UserIconArgs, icon};

#[cfg(test)]
pub(crate) fn sample_user() -> crate::api::user::User {
    use std::collections::BTreeMap;
    crate::api::user::User {
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
pub use add::{UserAddArgs, add};
pub use delete::{UserDeleteArgs, delete};
pub use list::{UserListArgs, list};
pub use recently_viewed::{UserRecentlyViewedArgs, recently_viewed};
pub use recently_viewed_projects::{UserRecentlyViewedProjectsArgs, recently_viewed_projects};
pub use recently_viewed_wikis::{UserRecentlyViewedWikisArgs, recently_viewed_wikis};
pub use show::{UserShowArgs, show};
pub use update::{UserUpdateArgs, update};
