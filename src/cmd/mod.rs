pub mod activity_shared;
pub mod auth;
pub mod banner;
pub mod document;
pub mod git;
pub mod issue;
pub mod notification;
pub mod pr;
pub mod priority;
pub mod project;
pub mod rate_limit;
pub mod resolution;
pub mod shared_file;
pub mod space;
pub mod star;
pub mod team;
pub mod user;
pub mod watch;
pub mod wiki;

pub(crate) fn print_json<T: serde::Serialize>(value: &T) -> anyhow::Result<()> {
    anstream::println!(
        "{}",
        serde_json::to_string_pretty(value).context("Failed to serialize JSON")?
    );
    Ok(())
}

use anyhow::Context;
