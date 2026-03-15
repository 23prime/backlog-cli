mod activities;
mod disk_usage;
mod licence;
mod notification;
mod show;
mod update_notification;

pub use activities::{SpaceActivitiesArgs, activities};
pub use disk_usage::{SpaceDiskUsageArgs, disk_usage};
pub use licence::{SpaceLicenceArgs, licence};
pub use notification::{SpaceNotificationArgs, notification};
pub use show::{SpaceShowArgs, show};
pub use update_notification::{SpaceUpdateNotificationArgs, update_notification};
