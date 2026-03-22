pub mod attachment;
mod count;
mod create;
mod delete;
mod history;
pub mod list;
pub mod shared_file;
mod show;
pub mod star;
pub mod tag;
mod update;

pub use count::{WikiCountArgs, count};
pub use create::{WikiCreateArgs, create};
pub use delete::{WikiDeleteArgs, delete};
pub use history::{WikiHistoryArgs, history};
pub use list::{WikiListArgs, list};
pub use show::{WikiShowArgs, show};
pub use update::{WikiUpdateArgs, update};

#[cfg(test)]
pub(crate) fn sample_wiki() -> crate::api::wiki::Wiki {
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    use std::collections::BTreeMap;
    crate::api::wiki::Wiki {
        id: 1,
        project_id: 1,
        name: "Home".to_string(),
        content: "# Home\nWelcome!".to_string(),
        tags: vec![crate::api::wiki::WikiTag {
            id: 1,
            name: "guide".to_string(),
        }],
        created_user: sample_wiki_user(),
        created: "2024-01-01T00:00:00Z".to_string(),
        updated_user: sample_wiki_user(),
        updated: "2024-01-02T00:00:00Z".to_string(),
        extra: BTreeMap::new(),
    }
}
