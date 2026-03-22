mod delete;
mod get;
pub mod list;

pub use delete::{PrAttachmentDeleteArgs, delete};
pub use get::{PrAttachmentGetArgs, get};
pub use list::{PrAttachmentListArgs, list};
