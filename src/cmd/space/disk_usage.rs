use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient, disk_usage::DiskUsage};

pub struct SpaceDiskUsageArgs {
    json: bool,
}

impl SpaceDiskUsageArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn disk_usage(args: &SpaceDiskUsageArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    disk_usage_with(args, &client)
}

pub fn disk_usage_with(args: &SpaceDiskUsageArgs, api: &dyn BacklogApi) -> Result<()> {
    let usage = api.get_space_disk_usage()?;
    if args.json {
        crate::cmd::print_json(&usage)?;
    } else {
        println!("{}", format_disk_usage_text(&usage));
    }
    Ok(())
}

fn format_disk_usage_text(usage: &DiskUsage) -> String {
    format!(
        "Capacity:   {} bytes\nIssue:      {} bytes\nWiki:       {} bytes\nFile:       {} bytes\nSubversion: {} bytes\nGit:        {} bytes\nGit LFS:    {} bytes\nDetails:    {} project(s) — use --json for breakdown",
        usage.capacity,
        usage.issue,
        usage.wiki,
        usage.file,
        usage.subversion,
        usage.git,
        usage.git_lfs,
        usage.details.len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        disk_usage: Option<DiskUsage>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space_disk_usage(&self) -> Result<DiskUsage> {
            self.disk_usage
                .clone()
                .ok_or_else(|| anyhow!("no disk usage"))
        }
    }

    fn sample_disk_usage() -> DiskUsage {
        use crate::api::disk_usage::DiskUsageDetail;
        DiskUsage {
            capacity: 5242880,
            issue: 2048,
            wiki: 512,
            file: 1024,
            subversion: 64,
            git: 256,
            git_lfs: 128,
            details: vec![DiskUsageDetail {
                project_id: 1,
                issue: 1024,
                wiki: 256,
                document: 0,
                file: 512,
                subversion: 32,
                git: 128,
                git_lfs: 64,
            }],
        }
    }

    #[test]
    fn disk_usage_with_text_output_succeeds() {
        let api = MockApi {
            disk_usage: Some(sample_disk_usage()),
        };
        assert!(disk_usage_with(&SpaceDiskUsageArgs::new(false), &api).is_ok());
    }

    #[test]
    fn disk_usage_with_json_output_succeeds() {
        let api = MockApi {
            disk_usage: Some(sample_disk_usage()),
        };
        assert!(disk_usage_with(&SpaceDiskUsageArgs::new(true), &api).is_ok());
    }

    #[test]
    fn disk_usage_with_propagates_api_error() {
        let api = MockApi { disk_usage: None };
        let err = disk_usage_with(&SpaceDiskUsageArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no disk usage"));
    }

    #[test]
    fn format_disk_usage_text_contains_fields() {
        let text = format_disk_usage_text(&sample_disk_usage());
        assert!(text.contains("5242880"));
        assert!(text.contains("2048"));
        assert!(text.contains("128"));
        assert!(text.contains("1 project(s)"));
        assert!(text.contains("--json"));
    }
}
