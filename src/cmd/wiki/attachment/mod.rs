mod add;
mod delete;
mod get;
pub mod list;

pub use add::{WikiAttachmentAddArgs, add};
pub use delete::{WikiAttachmentDeleteArgs, delete};
pub use get::{WikiAttachmentGetArgs, get};
pub use list::{WikiAttachmentListArgs, list};

#[cfg(test)]
pub(crate) fn sample_attachment() -> crate::api::wiki::WikiAttachment {
    use crate::cmd::wiki::list::tests_helper::sample_wiki_user;
    crate::api::wiki::WikiAttachment {
        id: 1,
        name: "image.png".to_string(),
        size: 2048,
        created_user: sample_wiki_user(),
        created: "2024-01-01T00:00:00Z".to_string(),
    }
}
