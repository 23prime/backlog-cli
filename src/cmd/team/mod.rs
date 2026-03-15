mod add;
mod delete;
mod list;
mod show;
mod update;

pub use add::{TeamAddArgs, add};
pub use delete::{TeamDeleteArgs, delete};
pub use list::{TeamListArgs, list};
pub use show::{TeamShowArgs, show};
pub use update::{TeamUpdateArgs, update};
