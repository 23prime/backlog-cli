use anstream::println;
use anyhow::Result;

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
        crate::cmd::print_json(&rl)?;
    } else {
        println!("{}", format_rate_limit_text(&rl));
    }
    Ok(())
}

fn format_rate_limit_text(rl: &RateLimit) -> String {
    let info = &rl.rate_limit;
    let mut out = format!(
        "Read:    limit={}, remaining={}, reset={}\n\
         Update:  limit={}, remaining={}, reset={}\n\
         Search:  limit={}, remaining={}, reset={}",
        info.read.limit,
        info.read.remaining,
        info.read.reset,
        info.update.limit,
        info.update.remaining,
        info.update.reset,
        info.search.limit,
        info.search.remaining,
        info.search.reset,
    );
    if let Some(icon) = &info.icon {
        out.push_str(&format!(
            "\nIcon:    limit={}, remaining={}, reset={}",
            icon.limit, icon.remaining, icon.reset
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::rate_limit::{RateLimitCategory, RateLimitInfo};
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
                read: RateLimitCategory {
                    limit: 600,
                    remaining: 591,
                    reset: 1774268714,
                },
                update: RateLimitCategory {
                    limit: 150,
                    remaining: 150,
                    reset: 1774268655,
                },
                search: RateLimitCategory {
                    limit: 150,
                    remaining: 150,
                    reset: 1774268655,
                },
                icon: Some(RateLimitCategory {
                    limit: 60,
                    remaining: 60,
                    reset: 1774268655,
                }),
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
    fn format_rate_limit_text_contains_all_categories() {
        let text = format_rate_limit_text(&sample_rate_limit());
        assert!(text.contains("Read:"));
        assert!(text.contains("Update:"));
        assert!(text.contains("Search:"));
        assert!(text.contains("Icon:"));
        assert!(text.contains("600"));
        assert!(text.contains("591"));
    }

    #[test]
    fn format_rate_limit_text_without_icon() {
        let mut rl = sample_rate_limit();
        rl.rate_limit.icon = None;
        let text = format_rate_limit_text(&rl);
        assert!(!text.contains("Icon:"));
    }
}
