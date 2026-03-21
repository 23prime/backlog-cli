mod add;
mod delete;
mod get;
pub mod list;

pub use add::{WikiAttachmentAddArgs, add};
pub use delete::{WikiAttachmentDeleteArgs, delete};
pub use get::{WikiAttachmentGetArgs, get};
pub use list::{WikiAttachmentListArgs, list};
