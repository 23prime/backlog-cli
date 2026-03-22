use anstream::println;
use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::api::{BacklogApi, BacklogClient};

pub struct ProjectImageArgs {
    key: String,
    output: Option<PathBuf>,
}

impl ProjectImageArgs {
    pub fn new(key: String, output: Option<PathBuf>) -> Self {
        Self { key, output }
    }
}

pub fn image(args: &ProjectImageArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    image_with(args, &client)
}

pub fn image_with(args: &ProjectImageArgs, api: &dyn BacklogApi) -> Result<()> {
    let (bytes, filename) = api.download_project_image(&args.key)?;
    let path = args
        .output
        .clone()
        .unwrap_or_else(|| default_output_path(&filename));
    std::fs::write(&path, &bytes).with_context(|| format!("Failed to write {}", path.display()))?;
    println!("Saved: {} ({} bytes)", path.display(), bytes.len());
    Ok(())
}

fn default_output_path(filename: &str) -> PathBuf {
    let normalized = filename.trim();
    let base = std::path::Path::new(normalized)
        .file_name()
        .unwrap_or(std::ffi::OsStr::new(""));
    let base_lower = base.to_string_lossy().to_ascii_lowercase();
    let is_generic_attachment = base_lower == "attachment" || base_lower.starts_with("attachment.");

    if base.is_empty() || is_generic_attachment {
        PathBuf::from("project_image")
    } else {
        PathBuf::from(base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use tempfile::tempdir;

    struct MockApi {
        result: Option<(Vec<u8>, String)>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn download_project_image(&self, _key: &str) -> anyhow::Result<(Vec<u8>, String)> {
            self.result
                .clone()
                .ok_or_else(|| anyhow!("download failed"))
        }
    }

    fn args(output: Option<PathBuf>) -> ProjectImageArgs {
        ProjectImageArgs::new("TEST".to_string(), output)
    }

    #[test]
    fn image_with_saves_file_to_specified_path() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("out.png");
        let api = MockApi {
            result: Some((b"png-data".to_vec(), "project_image.png".to_string())),
        };
        assert!(image_with(&args(Some(path.clone())), &api).is_ok());
        assert_eq!(std::fs::read(&path).unwrap(), b"png-data");
    }

    #[test]
    fn image_with_propagates_api_error() {
        let api = MockApi { result: None };
        let err = image_with(&args(None), &api).unwrap_err();
        assert!(err.to_string().contains("download failed"));
    }

    #[test]
    fn default_output_path_uses_server_filename() {
        assert_eq!(
            default_output_path("project_image.png"),
            PathBuf::from("project_image.png")
        );
    }

    #[test]
    fn default_output_path_falls_back_for_attachment() {
        assert_eq!(
            default_output_path("attachment"),
            PathBuf::from("project_image")
        );
    }

    #[test]
    fn default_output_path_falls_back_for_attachment_with_extension() {
        assert_eq!(
            default_output_path("attachment.png"),
            PathBuf::from("project_image")
        );
    }

    #[test]
    fn default_output_path_falls_back_for_path_with_attachment_basename() {
        assert_eq!(
            default_output_path("foo/attachment.png"),
            PathBuf::from("project_image")
        );
    }

    #[test]
    fn default_output_path_falls_back_for_empty() {
        assert_eq!(default_output_path(""), PathBuf::from("project_image"));
    }
}
