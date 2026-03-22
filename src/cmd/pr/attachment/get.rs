use anstream::println;
use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::api::{BacklogApi, BacklogClient};

pub struct PrAttachmentGetArgs {
    project_id_or_key: String,
    repo_id_or_name: String,
    number: u64,
    attachment_id: u64,
    output: Option<PathBuf>,
}

impl PrAttachmentGetArgs {
    pub fn new(
        project_id_or_key: String,
        repo_id_or_name: String,
        number: u64,
        attachment_id: u64,
        output: Option<PathBuf>,
    ) -> Self {
        Self {
            project_id_or_key,
            repo_id_or_name,
            number,
            attachment_id,
            output,
        }
    }
}

pub fn get(args: &PrAttachmentGetArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    get_with(args, &client)
}

pub fn get_with(args: &PrAttachmentGetArgs, api: &dyn BacklogApi) -> Result<()> {
    let (bytes, filename) = api.download_pull_request_attachment(
        &args.project_id_or_key,
        &args.repo_id_or_name,
        args.number,
        args.attachment_id,
    )?;
    let path = args.output.clone().unwrap_or_else(|| {
        let base = std::path::Path::new(&filename)
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("attachment"));
        PathBuf::from(base)
    });
    std::fs::write(&path, &bytes).with_context(|| format!("Failed to write {}", path.display()))?;
    println!("Saved: {} ({} bytes)", path.display(), bytes.len());
    Ok(())
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
        fn download_pull_request_attachment(
            &self,
            _project_id_or_key: &str,
            _repo_id_or_name: &str,
            _number: u64,
            _attachment_id: u64,
        ) -> anyhow::Result<(Vec<u8>, String)> {
            self.result
                .clone()
                .ok_or_else(|| anyhow!("download failed"))
        }
    }

    fn args(output: Option<PathBuf>) -> PrAttachmentGetArgs {
        PrAttachmentGetArgs::new("TEST".to_string(), "main".to_string(), 1, 1, output)
    }

    #[test]
    fn get_with_saves_file_to_specified_path() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("out.png");
        let api = MockApi {
            result: Some((b"hello".to_vec(), "file.png".to_string())),
        };
        assert!(get_with(&args(Some(path.clone())), &api).is_ok());
        assert_eq!(std::fs::read(&path).unwrap(), b"hello");
    }

    #[test]
    fn get_with_saves_file_to_default_filename() {
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
            result: Some((b"world".to_vec(), "response.png".to_string())),
        };
        assert!(get_with(&args(None), &api).is_ok());
        assert_eq!(
            std::fs::read(dir.path().join("response.png")).unwrap(),
            b"world"
        );
    }

    #[test]
    fn get_with_propagates_api_error() {
        let api = MockApi { result: None };
        let err = get_with(&args(None), &api).unwrap_err();
        assert!(err.to_string().contains("download failed"));
    }
}
