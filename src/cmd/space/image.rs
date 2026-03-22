use anstream::println;
use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::api::{BacklogApi, BacklogClient};

pub struct SpaceImageArgs {
    output: Option<PathBuf>,
}

impl SpaceImageArgs {
    pub fn new(output: Option<PathBuf>) -> Self {
        Self { output }
    }
}

pub fn image(args: &SpaceImageArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    image_with(args, &client)
}

pub fn image_with(args: &SpaceImageArgs, api: &dyn BacklogApi) -> Result<()> {
    let (bytes, filename) = api.download_space_image()?;
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
    let lower = normalized.to_ascii_lowercase();
    let is_generic_attachment = lower == "attachment" || lower.starts_with("attachment.");

    let effective = if normalized.is_empty() || is_generic_attachment {
        "space_image"
    } else {
        normalized
    };
    let base = std::path::Path::new(effective)
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("space_image"));
    PathBuf::from(base)
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
        fn download_space_image(&self) -> anyhow::Result<(Vec<u8>, String)> {
            self.result
                .clone()
                .ok_or_else(|| anyhow!("download failed"))
        }
    }

    fn args(output: Option<PathBuf>) -> SpaceImageArgs {
        SpaceImageArgs::new(output)
    }

    #[test]
    fn image_with_saves_file_to_specified_path() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("out.png");
        let api = MockApi {
            result: Some((b"png-data".to_vec(), "space_image.png".to_string())),
        };
        assert!(image_with(&args(Some(path.clone())), &api).is_ok());
        assert_eq!(std::fs::read(&path).unwrap(), b"png-data");
    }

    #[test]
    fn image_with_saves_file_to_default_filename() {
        let dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        struct DirGuard(std::path::PathBuf);
        impl Drop for DirGuard {
            fn drop(&mut self) {
                let _ = std::env::set_current_dir(&self.0);
            }
        }
        let _guard = DirGuard(original_dir);

        let api = MockApi {
            result: Some((b"png-data".to_vec(), "space_image.png".to_string())),
        };
        assert!(image_with(&args(None), &api).is_ok());
        assert_eq!(
            std::fs::read(dir.path().join("space_image.png")).unwrap(),
            b"png-data"
        );
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
            default_output_path("space_image.png"),
            PathBuf::from("space_image.png")
        );
    }

    #[test]
    fn default_output_path_falls_back_for_attachment() {
        assert_eq!(
            default_output_path("attachment"),
            PathBuf::from("space_image")
        );
    }

    #[test]
    fn default_output_path_falls_back_for_attachment_with_extension() {
        assert_eq!(
            default_output_path("attachment.png"),
            PathBuf::from("space_image")
        );
    }

    #[test]
    fn default_output_path_falls_back_for_empty() {
        assert_eq!(default_output_path(""), PathBuf::from("space_image"));
    }
}
