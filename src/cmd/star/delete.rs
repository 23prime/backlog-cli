use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct StarDeleteArgs {
    star_id: u64,
}

impl StarDeleteArgs {
    pub fn new(star_id: u64) -> Self {
        Self { star_id }
    }
}

pub fn delete(args: &StarDeleteArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    delete_with(args, &client)
}

pub fn delete_with(args: &StarDeleteArgs, api: &dyn BacklogApi) -> Result<()> {
    api.delete_star(args.star_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        ok: bool,
    }

    impl crate::api::BacklogApi for MockApi {
        fn delete_star(&self, _star_id: u64) -> anyhow::Result<()> {
            if self.ok {
                Ok(())
            } else {
                Err(anyhow!("api error"))
            }
        }
    }

    fn args() -> StarDeleteArgs {
        StarDeleteArgs::new(42)
    }

    #[test]
    fn delete_with_succeeds() {
        let api = MockApi { ok: true };
        assert!(delete_with(&args(), &api).is_ok());
    }

    #[test]
    fn delete_with_propagates_api_error() {
        let api = MockApi { ok: false };
        let err = delete_with(&args(), &api).unwrap_err();
        assert!(err.to_string().contains("api error"));
    }
}
