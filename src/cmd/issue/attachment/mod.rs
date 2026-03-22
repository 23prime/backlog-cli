pub mod delete;
mod get;
mod list;

pub use delete::{IssueAttachmentDeleteArgs, delete};
pub use get::{IssueAttachmentGetArgs, get};
pub use list::{IssueAttachmentListArgs, list};

#[cfg(test)]
pub(crate) fn sample_attachment() -> crate::api::issue::IssueAttachment {
    use std::collections::BTreeMap;
    let user = crate::api::issue::IssueUser {
        id: 1,
        user_id: Some("john".to_string()),
        name: "John Doe".to_string(),
        role_type: 1,
        lang: None,
        mail_address: None,
        extra: BTreeMap::new(),
    };
    crate::api::issue::IssueAttachment {
        id: 1,
        name: "file.txt".to_string(),
        size: 1024,
        created_user: user,
        created: "2024-01-01T00:00:00Z".to_string(),
    }
}
