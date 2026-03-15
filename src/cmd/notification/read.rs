use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct NotificationReadArgs {
    pub id: u64,
}

impl NotificationReadArgs {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

pub fn read(args: &NotificationReadArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    read_with(args, &client)
}

pub fn read_with(args: &NotificationReadArgs, api: &dyn BacklogApi) -> Result<()> {
    api.read_notification(args.id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;

    struct MockApi {
        called_with: Cell<Option<u64>>,
    }

    impl BacklogApi for MockApi {
        fn read_notification(&self, id: u64) -> Result<()> {
            self.called_with.set(Some(id));
            Ok(())
        }
    }

    #[test]
    fn read_calls_api_with_correct_id() {
        let api = MockApi {
            called_with: Cell::new(None),
        };
        read_with(&NotificationReadArgs::new(42), &api).unwrap();
        assert_eq!(api.called_with.get(), Some(42));
    }
}
