pub mod attachment;
mod create;
mod delete;
mod history;
pub mod list;
mod show;
mod update;

pub use create::{WikiCreateArgs, create};
pub use delete::{WikiDeleteArgs, delete};
pub use history::{WikiHistoryArgs, history};
pub use list::{WikiListArgs, list};
pub use show::{WikiShowArgs, show};
pub use update::{WikiUpdateArgs, update};
