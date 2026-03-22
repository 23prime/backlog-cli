mod activities;
mod disk_usage;
mod licence;
mod notification;
mod show;
mod update_notification;

pub use activities::{SpaceActivitiesArgs, activities};

#[cfg(test)]
pub(crate) fn sample_notification() -> crate::api::space_notification::SpaceNotification {
    crate::api::space_notification::SpaceNotification {
        content: "Scheduled maintenance on 2024-07-01.".to_string(),
        updated: Some("2024-06-18T07:55:37Z".to_string()),
    }
}
pub use disk_usage::{SpaceDiskUsageArgs, disk_usage};
pub use licence::{SpaceLicenceArgs, licence};
pub use notification::{SpaceNotificationArgs, notification};
pub use show::{SpaceShowArgs, show};
pub use update_notification::{SpaceUpdateNotificationArgs, update_notification};
