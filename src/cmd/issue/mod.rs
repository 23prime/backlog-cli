pub mod attachment;
pub mod comment;
mod count;
mod create;
mod delete;
pub mod list;
mod show;
mod update;

pub use count::count;
pub use create::create;
pub use delete::delete;
pub use list::list;
pub use show::show;
pub use update::update;

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
