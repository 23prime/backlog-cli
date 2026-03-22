use anstream::println;
use anyhow::Result;
use std::path::PathBuf;

use crate::api::{BacklogApi, BacklogClient, space::SpaceAttachment};

pub struct SpaceUploadAttachmentArgs {
    file: PathBuf,
    json: bool,
}

impl SpaceUploadAttachmentArgs {
    pub fn new(file: PathBuf, json: bool) -> Self {
        Self { file, json }
    }
}

pub fn upload_attachment(args: &SpaceUploadAttachmentArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    upload_attachment_with(args, &client)
}

pub fn upload_attachment_with(
    args: &SpaceUploadAttachmentArgs,
    api: &dyn BacklogApi,
) -> Result<()> {
    let attachment = api.upload_space_attachment(&args.file)?;
    if args.json {
        crate::cmd::print_json(&attachment)?;
    } else {
        println!("{}", format_attachment_text(&attachment));
    }
    Ok(())
}

fn format_attachment_text(a: &SpaceAttachment) -> String {
    format!(
        "ID:      {}\nName:    {}\nSize:    {} bytes\nCreated: {}",
        a.id, a.name, a.size, a.created
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::collections::BTreeMap;
    use tempfile::NamedTempFile;

    use crate::api::space::{SpaceAttachment, SpaceAttachmentUser};

    struct MockApi {
        result: Option<SpaceAttachment>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn upload_space_attachment(
            &self,
            _file_path: &std::path::Path,
        ) -> anyhow::Result<SpaceAttachment> {
            self.result.clone().ok_or_else(|| anyhow!("upload failed"))
        }
    }

    fn sample_attachment() -> SpaceAttachment {
        SpaceAttachment {
            id: 1,
            name: "test.txt".to_string(),
            size: 100,
            created_user: SpaceAttachmentUser {
                id: 1,
                user_id: Some("alice".to_string()),
                name: "Alice".to_string(),
                extra: BTreeMap::new(),
            },
            created: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    fn args(json: bool) -> SpaceUploadAttachmentArgs {
        let tmp = NamedTempFile::new().unwrap();
        SpaceUploadAttachmentArgs::new(tmp.path().to_path_buf(), json)
    }

    #[test]
    fn upload_attachment_with_text_output_succeeds() {
        let api = MockApi {
            result: Some(sample_attachment()),
        };
        assert!(upload_attachment_with(&args(false), &api).is_ok());
    }

    #[test]
    fn upload_attachment_with_json_output_succeeds() {
        let api = MockApi {
            result: Some(sample_attachment()),
        };
        assert!(upload_attachment_with(&args(true), &api).is_ok());
    }

    #[test]
    fn upload_attachment_with_propagates_api_error() {
        let api = MockApi { result: None };
        let err = upload_attachment_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("upload failed"));
    }

    #[test]
    fn format_attachment_text_contains_all_fields() {
        let text = format_attachment_text(&sample_attachment());
        assert!(text.contains("1"));
        assert!(text.contains("test.txt"));
        assert!(text.contains("100"));
        assert!(text.contains("2024-01-01T00:00:00Z"));
    }
}
