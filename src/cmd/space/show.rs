use anstream::println;
use anyhow::{Context, Result};

use crate::api::{BacklogApi, BacklogClient, space::Space};

pub struct SpaceShowArgs {
    json: bool,
}

impl SpaceShowArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn show(args: &SpaceShowArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &SpaceShowArgs, api: &dyn BacklogApi) -> Result<()> {
    let space = api.get_space()?;
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&space).context("Failed to serialize JSON")?
        );
    } else {
        println!("{}", format_space_text(&space));
    }
    Ok(())
}

fn format_space_text(space: &Space) -> String {
    format!(
        "Space key:  {}\nName:       {}\nLanguage:   {}\nTimezone:   {}\nFormatting: {}\nCreated:    {}\nUpdated:    {}",
        space.space_key,
        space.name,
        space.lang,
        space.timezone,
        space.text_formatting_rule,
        space.created,
        space.updated,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    struct MockApi {
        space: Option<Space>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_space(&self) -> Result<Space> {
            self.space.clone().ok_or_else(|| anyhow!("no space"))
        }
    }

    fn sample_space() -> Space {
        Space {
            space_key: "mycompany".to_string(),
            name: "My Company".to_string(),
            owner_id: 1,
            lang: "ja".to_string(),
            timezone: "Asia/Tokyo".to_string(),
            text_formatting_rule: "markdown".to_string(),
            created: "2020-01-01T00:00:00Z".to_string(),
            updated: "2024-06-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn show_with_text_output_succeeds() {
        let api = MockApi {
            space: Some(sample_space()),
        };
        assert!(show_with(&SpaceShowArgs::new(false), &api).is_ok());
    }

    #[test]
    fn show_with_json_output_succeeds() {
        let api = MockApi {
            space: Some(sample_space()),
        };
        assert!(show_with(&SpaceShowArgs::new(true), &api).is_ok());
    }

    #[test]
    fn show_with_propagates_api_error() {
        let api = MockApi { space: None };
        let err = show_with(&SpaceShowArgs::new(false), &api).unwrap_err();
        assert!(err.to_string().contains("no space"));
    }

    #[test]
    fn format_space_text_contains_all_fields() {
        let text = format_space_text(&sample_space());
        assert!(text.contains("mycompany"));
        assert!(text.contains("My Company"));
        assert!(text.contains("ja"));
        assert!(text.contains("Asia/Tokyo"));
        assert!(text.contains("markdown"));
        assert!(text.contains("2020-01-01T00:00:00Z"));
        assert!(text.contains("2024-06-01T00:00:00Z"));
    }

    #[test]
    fn format_space_text_label_alignment() {
        let text = format_space_text(&sample_space());
        assert!(text.contains("Space key:  mycompany"));
        assert!(text.contains("Name:       My Company"));
    }
}
