use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient, project::ProjectDiskUsage};

pub struct ProjectDiskUsageArgs {
    key: String,
    json: bool,
}

impl ProjectDiskUsageArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn disk_usage(args: &ProjectDiskUsageArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    disk_usage_with(args, &client)
}

pub fn disk_usage_with(args: &ProjectDiskUsageArgs, api: &dyn BacklogApi) -> Result<()> {
    let usage = api.get_project_disk_usage(&args.key)?;
    if args.json {
        crate::cmd::print_json(&usage)?;
    } else {
        println!("{}", format_disk_usage_text(&usage));
    }
    Ok(())
}

fn format_disk_usage_text(usage: &ProjectDiskUsage) -> String {
    format!(
        "Issue:      {} bytes\nWiki:       {} bytes\nDocument:   {} bytes\nFile:       {} bytes\nSubversion: {} bytes\nGit:        {} bytes\nGit LFS:    {} bytes",
        usage.issue,
        usage.wiki,
        usage.document,
        usage.file,
        usage.subversion,
        usage.git,
        usage.git_lfs,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        disk_usage: Option<ProjectDiskUsage>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_project_disk_usage(&self, _key: &str) -> anyhow::Result<ProjectDiskUsage> {
            self.disk_usage
                .clone()
                .ok_or_else(|| anyhow!("no disk usage"))
        }
    }

    fn sample_disk_usage() -> ProjectDiskUsage {
        ProjectDiskUsage {
            project_id: 1,
            issue: 2048,
            wiki: 512,
            document: 0,
            file: 1024,
            subversion: 64,
            git: 256,
            git_lfs: 128,
        }
    }

    #[test]
    fn disk_usage_with_text_output_succeeds() {
        let api = MockApi {
            disk_usage: Some(sample_disk_usage()),
        };
        assert!(
            disk_usage_with(&ProjectDiskUsageArgs::new("TEST".to_string(), false), &api).is_ok()
        );
    }

    #[test]
    fn disk_usage_with_json_output_succeeds() {
        let api = MockApi {
            disk_usage: Some(sample_disk_usage()),
        };
        assert!(
            disk_usage_with(&ProjectDiskUsageArgs::new("TEST".to_string(), true), &api).is_ok()
        );
    }

    #[test]
    fn disk_usage_with_propagates_api_error() {
        let api = MockApi { disk_usage: None };
        let err = disk_usage_with(&ProjectDiskUsageArgs::new("TEST".to_string(), false), &api)
            .unwrap_err();
        assert!(err.to_string().contains("no disk usage"));
    }

    #[test]
    fn format_disk_usage_text_contains_fields() {
        let text = format_disk_usage_text(&sample_disk_usage());
        assert!(text.contains("2048"));
        assert!(text.contains("512"));
        assert!(text.contains("128"));
        assert!(text.contains("Issue:"));
        assert!(text.contains("Git LFS:"));
    }
}
