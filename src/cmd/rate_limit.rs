use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, rate_limit::RateLimit};

pub struct RateLimitArgs {
    json: bool,
}

impl RateLimitArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn show(args: &RateLimitArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &RateLimitArgs, api: &dyn BacklogApi) -> Result<()> {
    let rl = api.get_rate_limit()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&rl).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_rate_limit_text(&rl));
    }
    Ok(())
}

fn format_rate_limit_text(rl: &RateLimit) -> String {
    format!(
        "Limit:     {}\nRemaining: {}\nReset:     {}",
        rl.rate_limit.limit, rl.rate_limit.remaining, rl.rate_limit.reset,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::rate_limit::RateLimitInfo;
    use anyhow::anyhow;

    struct MockApi {
        rate_limit: Option<RateLimit>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_rate_limit(&self) -> anyhow::Result<RateLimit> {
            self.rate_limit
                .clone()
                .ok_or_else(|| anyhow!("no rate limit"))
        }
    }

    fn sample_rate_limit() -> RateLimit {
        RateLimit {
            rate_limit: RateLimitInfo {
                limit: 600,
                remaining: 599,
                reset: 1698230400,
            },
        }
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            rate_limit: Some(sample_rate_limit()),
        };
        assert!(show_with(&RateLimitArgs::new(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            rate_limit: Some(sample_rate_limit()),
        };
        assert!(show_with(&RateLimitArgs::new(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { rate_limit: None };
        let err = show_with(&RateLimitArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no rate limit"));
    }

    #[test]
    fn format_rate_limit_text_contains_all_fields() {
        let text = format_rate_limit_text(&sample_rate_limit());
        assert!(text.contains("600"));
        assert!(text.contains("599"));
        assert!(text.contains("1698230400"));
    }
}
