use anstream::println;
use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::api::{BacklogApi, BacklogClient};

pub struct TeamIconArgs {
    id: u64,
    output: Option<PathBuf>,
}

impl TeamIconArgs {
    pub fn new(id: u64, output: Option<PathBuf>) -> Self {
        Self { id, output }
    }
}

pub fn icon(args: &TeamIconArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    icon_with(args, &client)
}

pub fn icon_with(args: &TeamIconArgs, api: &dyn BacklogApi) -> Result<()> {
    let (bytes, filename) = api.download_team_icon(args.id)?;
    let path = args.output.clone().unwrap_or_else(|| {
        let base = std::path::Path::new(&filename)
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("icon"));
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
        fn download_team_icon(&self, _team_id: u64) -> anyhow::Result<(Vec<u8>, String)> {
            self.result
                .clone()
                .ok_or_else(|| anyhow!("download failed"))
        }
    }

    fn args(output: Option<PathBuf>) -> TeamIconArgs {
        TeamIconArgs::new(1, output)
    }

    #[test]
    fn icon_with_saves_file_to_specified_path() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("out.png");
        let api = MockApi {
            result: Some((b"png-data".to_vec(), "icon.png".to_string())),
        };
        assert!(icon_with(&args(Some(path.clone())), &api).is_ok());
        assert_eq!(std::fs::read(&path).unwrap(), b"png-data");
    }

    #[test]
    fn icon_with_saves_file_to_default_filename() {
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
            result: Some((b"png-data".to_vec(), "icon.png".to_string())),
        };
        assert!(icon_with(&args(None), &api).is_ok());
        assert_eq!(
            std::fs::read(dir.path().join("icon.png")).unwrap(),
            b"png-data"
        );
    }

    #[test]
    fn icon_with_propagates_api_error() {
        let api = MockApi { result: None };
        let err = icon_with(&args(None), &api).unwrap_err();
        assert!(err.to_string().contains("download failed"));
    }
}
