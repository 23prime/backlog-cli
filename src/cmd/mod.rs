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

use anyhow::Context;
use std::path::PathBuf;

pub(crate) fn print_json<T: serde::Serialize>(value: &T) -> anyhow::Result<()> {
    anstream::println!(
        "{}",
        serde_json::to_string_pretty(value).context("Failed to serialize JSON")?
    );
    Ok(())
}

/// If `path` has no extension, infer one from image magic bytes and return a new path with it.
/// Returns `path` unchanged when it already has an extension or the format is unrecognized.
pub(crate) fn with_image_extension(path: PathBuf, bytes: &[u8]) -> PathBuf {
    if path.extension().is_some() {
        return path;
    }
    let ext = if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        "png"
    } else if bytes.starts_with(b"\xFF\xD8\xFF") {
        "jpg"
    } else if bytes.starts_with(b"GIF8") {
        "gif"
    } else if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        "webp"
    } else {
        return path;
    };
    path.with_extension(ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_image_extension_adds_png() {
        let path = PathBuf::from("icon");
        let bytes = b"\x89PNG\r\n\x1a\n rest";
        assert_eq!(with_image_extension(path, bytes), PathBuf::from("icon.png"));
    }

    #[test]
    fn with_image_extension_adds_jpg() {
        let path = PathBuf::from("icon");
        let bytes = b"\xFF\xD8\xFF rest";
        assert_eq!(with_image_extension(path, bytes), PathBuf::from("icon.jpg"));
    }

    #[test]
    fn with_image_extension_adds_gif() {
        let path = PathBuf::from("icon");
        let bytes = b"GIF89a rest";
        assert_eq!(with_image_extension(path, bytes), PathBuf::from("icon.gif"));
    }

    #[test]
    fn with_image_extension_adds_webp() {
        let mut bytes = b"RIFF\x00\x00\x00\x00WEBP".to_vec();
        bytes.extend_from_slice(b" rest");
        assert_eq!(
            with_image_extension(PathBuf::from("icon"), &bytes),
            PathBuf::from("icon.webp")
        );
    }

    #[test]
    fn with_image_extension_preserves_existing_extension() {
        let path = PathBuf::from("icon.png");
        let bytes = b"\x89PNG\r\n\x1a\n rest";
        assert_eq!(with_image_extension(path, bytes), PathBuf::from("icon.png"));
    }

    #[test]
    fn with_image_extension_unchanged_for_unknown_format() {
        let path = PathBuf::from("icon");
        assert_eq!(
            with_image_extension(path, b"unknown"),
            PathBuf::from("icon")
        );
    }
}
