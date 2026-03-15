use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub fn reset_unread() -> Result<()> {
    let client = BacklogClient::from_config()?;
    reset_unread_with(&client)
}

pub fn reset_unread_with(api: &dyn BacklogApi) -> Result<()> {
    api.reset_unread_notifications()?;
    anstream::println!("Unread count reset.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::notification::NotificationCount;

    struct MockApi;

    impl BacklogApi for MockApi {
        fn reset_unread_notifications(&self) -> Result<NotificationCount> {
            Ok(NotificationCount { count: 0 })
        }
    }

    #[test]
    fn reset_unread_succeeds() {
        reset_unread_with(&MockApi).unwrap();
    }
}
