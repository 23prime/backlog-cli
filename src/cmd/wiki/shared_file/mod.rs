mod link;
pub mod list;
mod unlink;

pub use link::{WikiSharedFileLinkArgs, link};
pub use list::{WikiSharedFileListArgs, list};
pub use unlink::{WikiSharedFileUnlinkArgs, unlink};
