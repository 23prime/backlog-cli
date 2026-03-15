pub mod attachment;
pub mod comment;
mod count;
mod create;
mod delete;
pub mod list;
pub mod participant;
pub mod shared_file;
mod show;
mod update;

pub use count::{IssueCountArgs, count};
pub use create::{IssueCreateArgs, create};
pub use delete::{IssueDeleteArgs, delete};
pub use list::{IssueListArgs, list};
pub use show::{IssueShowArgs, show};
pub use update::{IssueUpdateArgs, update};

/// Parent-child relationship filter for issue list/count.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ParentChild {
    /// All issues (0)
    All,
    /// Issues that are not child issues (1)
    NotChild,
    /// Child issues only (2)
    Child,
    /// Issues that are neither parent nor child (3)
    Standalone,
    /// Parent issues — issues that have child issues (4)
    Parent,
}

impl ParentChild {
    pub fn to_api_value(&self) -> u8 {
        match self {
            Self::All => 0,
            Self::NotChild => 1,
            Self::Child => 2,
            Self::Standalone => 3,
            Self::Parent => 4,
        }
    }
}
