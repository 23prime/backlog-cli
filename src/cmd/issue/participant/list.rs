use anstream::println;
use anyhow::Result;
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient};

pub struct IssueParticipantListArgs {
    key: String,
    json: bool,
}

impl IssueParticipantListArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn list(args: &IssueParticipantListArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    list_with(args, &client)
}

pub fn list_with(args: &IssueParticipantListArgs, api: &dyn BacklogApi) -> Result<()> {
    let participants = api.get_issue_participants(&args.key)?;
    if args.json {
        crate::cmd::print_json(&participants)?;
    } else {
        for p in &participants {
            let uid = p
                .user_id
                .as_deref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| p.id.to_string());
            println!("[{}] {}", uid.cyan().bold(), p.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::issue::IssueParticipant;
    use anyhow::anyhow;
    use std::collections::BTreeMap;

    fn sample_participant() -> IssueParticipant {
        IssueParticipant {
            id: 1,
            user_id: Some("alice".to_string()),
            name: "Alice".to_string(),
            role_type: Some(1),
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    fn bot_participant() -> IssueParticipant {
        IssueParticipant {
            id: 99,
            user_id: None,
            name: "Bot".to_string(),
            role_type: None,
            lang: None,
            mail_address: None,
            extra: BTreeMap::new(),
        }
    }

    struct MockApi {
        participants: Option<Vec<IssueParticipant>>,
    }

    impl crate::api::BacklogApi for MockApi {
        fn get_issue_participants(&self, _key: &str) -> anyhow::Result<Vec<IssueParticipant>> {
            self.participants
                .clone()
                .ok_or_else(|| anyhow!("no participants"))
        }
    }

    fn args(json: bool) -> IssueParticipantListArgs {
        IssueParticipantListArgs::new("TEST-1".to_string(), json)
    }

    #[test]
    fn list_with_text_output_succeeds() {
        let api = MockApi {
            participants: Some(vec![sample_participant()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }

    #[test]
    fn list_with_json_output_succeeds() {
        let api = MockApi {
            participants: Some(vec![sample_participant()]),
        };
        assert!(list_with(&args(true), &api).is_ok());
    }

    #[test]
    fn list_with_propagates_api_error() {
        let api = MockApi { participants: None };
        let err = list_with(&args(false), &api).unwrap_err();
        assert!(err.to_string().contains("no participants"));
    }

    #[test]
    fn list_with_bot_falls_back_to_numeric_id() {
        let api = MockApi {
            participants: Some(vec![bot_participant()]),
        };
        assert!(list_with(&args(false), &api).is_ok());
    }
}
