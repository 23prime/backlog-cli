use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct WatchReadArgs {
    watching_id: u64,
}

impl WatchReadArgs {
    pub fn new(watching_id: u64) -> Self {
        Self { watching_id }
    }
}

pub fn read(args: &WatchReadArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    read_with(args, &client)
}

pub fn read_with(args: &WatchReadArgs, api: &dyn BacklogApi) -> Result<()> {
    api.read_watching(args.watching_id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct MockApi {
        called_with: Cell<Option<u64>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn read_watching(&self, watching_id: u64) -> anyhow::Result<()> {
            self.called_with.set(Some(watching_id));
            Ok(())
        }
    }

    #[test]
    fn read_calls_api_with_correct_id() {
        let api = MockApi {
            called_with: Cell::new(None),
        };
        read_with(&WatchReadArgs::new(42), &api).unwrap();
        assert_eq!(api.called_with.get(), Some(42));
    }
}
