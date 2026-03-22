use anstream::println;
use anyhow::Result;

use crate::api::{BacklogApi, BacklogClient};

pub struct SharedFileListArgs {
    project_id_or_key: String,
    path: String,
    count: u32,
    order: Option<String>,
    offset: Option<u64>,
    json: bool,
}

impl SharedFileListArgs {
    pub fn try_new(
        project_id_or_key: String,
        path: Option<String>,
        count: u32,
        order: Option<String>,
        offset: Option<u64>,
        json: bool,
    ) -> anyhow::Result<Self> {
        if !(1..=100).contains(&count) {
            anyhow::bail!("count must be between 1 and 100");
        }
        Ok(Self {
            project_id_or_key,
            path: path.unwrap_or_default(),
            count,
            order,
            offset,
            json,
        })
    }
}

pub fn list(args: &SharedFileListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &SharedFileListArgs, api: &dyn BacklogApi) -> Result<()> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("count".to_string(), args.count.to_string()));
    if let Some(ref order) = args.order {
        params.push(("order".to_string(), order.clone()));
    }
    if let Some(offset) = args.offset {
        params.push(("offset".to_string(), offset.to_string()));
    }
    let files = api.list_shared_files(&args.project_id_or_key, &args.path, &params)?;
    if args.json {
        crate::cmd::print_json(&files)?;
    } else {
        for f in &files {
            let sep = if f.dir.ends_with('/') { "" } else { "/" };
            match f.size {
                Some(size) => {
                    println!("[{}] {}{}{} ({} bytes)", f.id, f.dir, sep, f.name, size)
                }
                None => println!("[{}] {}{}{}", f.id, f.dir, sep, f.name),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::shared_file::SharedFile;
    use crate::api::user::User;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    struct MockApi {
        files: Option<Vec<SharedFile>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn list_shared_files(
            &self,
            _project_id_or_key: &str,
            _path: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<SharedFile>> {
            self.files.clone().ok_or_else(|| anyhow!("no files"))
        }
    }

    fn sample_user() -> User {
        User {
            id: 1,
            user_id: Some("admin".to_string()),
            name: "Admin".to_string(),
            mail_address: None,
            role_type: 1,
            lang: None,
            last_login_time: None,
            extra: BTreeMap::new(),
        }
    }

    fn sample_file() -> SharedFile {
        SharedFile {
            id: 1,
            project_id: 10,
            file_type: "file".to_string(),
            dir: "/".to_string(),
            name: "test.txt".to_string(),
            size: Some(1024),
            created_user: sample_user(),
            created: "2024-01-01T00:00:00Z".to_string(),
            updated_user: None,
            updated: "2024-06-01T00:00:00Z".to_string(),
            extra: BTreeMap::new(),
        }
    }

    fn args(json: bool) -> SharedFileListArgs {
        SharedFileListArgs::try_new("TEST".to_string(), None, 20, None, None, json).unwrap()
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_file()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            files: Some(vec![sample_file()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { files: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no files"));
    }

    #[test]
    fn try_new_rejects_count_zero() {
        assert!(
            SharedFileListArgs::try_new("TEST".to_string(), None, 0, None, None, false).is_err()
        );
    }

    #[test]
    fn try_new_rejects_count_over_100() {
        assert!(
            SharedFileListArgs::try_new("TEST".to_string(), None, 101, None, None, false).is_err()
        );
    }
}
