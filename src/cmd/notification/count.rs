use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct NotificationCountArgs {
    pub json: bool,
}

impl NotificationCountArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn count(args: &NotificationCountArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    count_with(args, &client)
}

pub fn count_with(args: &NotificationCountArgs, api: &dyn BacklogApi) -> Result<()> {
    let result = api.count_notifications()?;
    if args.json {
        crate::cmd::print_json(&result)?;
    } else {
        anstream::println!("{}", result.count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::notification::NotificationCount;

    struct MockApi {
        count: u64,
    }

    impl BacklogApi for MockApi {
        fn count_notifications(&self) -> Result<NotificationCount> {
            Ok(NotificationCount { count: self.count })
        }
    }

    #[test]
    fn count_displays_number() {
        let api = MockApi { count: 5 };
        count_with(&NotificationCountArgs::new(false), &api).unwrap();
    }
}
