pub mod delete;
mod list;

pub use delete::{IssueAttachmentDeleteArgs, delete};
pub use list::{IssueAttachmentListArgs, list};
