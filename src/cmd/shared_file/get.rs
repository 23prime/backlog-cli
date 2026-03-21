use anstream::println;
use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::api::{BacklogApi, BacklogClient};

pub struct SharedFileGetArgs {
    project_id_or_key: String,
    shared_file_id: u64,
    output: Option<PathBuf>,
}

impl SharedFileGetArgs {
    pub fn new(project_id_or_key: String, shared_file_id: u64, output: Option<PathBuf>) -> Self {
        Self {
            project_id_or_key,
            shared_file_id,
            output,
        }
    }
}

pub fn get(args: &SharedFileGetArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    get_with(args, &client)
}

pub fn get_with(args: &SharedFileGetArgs, api: &dyn BacklogApi) -> Result<()> {
    let (bytes, filename) =
        api.download_shared_file(&args.project_id_or_key, args.shared_file_id)?;
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
        fn download_shared_file(
            &self,
            _project_id_or_key: &str,
            _shared_file_id: u64,
        ) -> anyhow::Result<(Vec<u8>, String)> {
            self.result
                .clone()
                .ok_or_else(|| anyhow!("download failed"))
        }
    }

    fn args(output: Option<PathBuf>) -> SharedFileGetArgs {
        SharedFileGetArgs::new("TEST".to_string(), 1, output)
    }

    #[test]
    fn get_with_saves_file_to_specified_path() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("out.txt");
        let api = MockApi {
            result: Some((b"hello".to_vec(), "file.txt".to_string())),
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
            result: Some((b"world".to_vec(), "response.txt".to_string())),
        };
        assert!(get_with(&args(None), &api).is_ok());
        assert_eq!(
            std::fs::read(dir.path().join("response.txt")).unwrap(),
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
