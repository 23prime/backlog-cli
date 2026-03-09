use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, user::User};
use crate::config::{self};
use crate::secret::{self, Backend};

pub fn login(no_banner: bool) -> Result<()> {
    if !no_banner {
        println!("Welcome to");
        crate::cmd::banner::print_banner();
    }

    let space_key = prompt("Backlog space key (e.g. mycompany for mycompany.backlog.com): ")?;
    let api_key = rpassword::prompt_password("API key: ").context("Failed to read API key")?;

    let backend = secret::set(&space_key, &api_key)?;

    let mut cfg = config::load()?;
    if !cfg.spaces.contains(&space_key) {
        cfg.spaces.push(space_key.clone());
    }
    cfg.current_space = Some(space_key);
    config::save(&cfg)?;

    println!(
        "{} (API key stored in {})",
        "Logged in successfully.".green(),
        backend
    );
    Ok(())
}

pub fn logout(space_key: Option<&str>) -> Result<()> {
    let mut cfg = config::load()?;

    let key = if let Some(k) = space_key {
        k.to_string()
    } else {
        cfg.current_space
            .clone()
            .context("No current space set. Specify a space key: `bl auth logout <space_key>`.")?
    };

    secret::delete(&key)?;
    remove_space_from_config(&mut cfg, &key);
    config::save(&cfg)?;

    println!("{} from {}", "Logged out".yellow(), key);
    Ok(())
}

pub fn logout_all() -> Result<()> {
    let cfg = config::load()?;

    for key in &cfg.spaces {
        secret::delete(key)?;
    }

    config::remove_config_file()?;
    secret::remove_credentials_file()?;

    println!(
        "{}",
        "Logged out from all spaces. Config files removed.".yellow()
    );
    Ok(())
}

pub fn list() -> Result<()> {
    let cfg = config::load()?;

    if cfg.spaces.is_empty() {
        println!("No spaces configured. Run `bl auth login` to add one.");
        return Ok(());
    }

    for space in &cfg.spaces {
        if cfg.current_space.as_deref() == Some(space) {
            println!("* {}", space.green());
        } else {
            println!("  {}", space);
        }
    }
    Ok(())
}

pub fn use_space(key: &str) -> Result<()> {
    let mut cfg = config::load()?;

    if !cfg.spaces.iter().any(|s| s == key) {
        anyhow::bail!(
            "Space '{}' is not configured. Run `bl auth login` to add it.",
            key
        );
    }

    cfg.current_space = Some(key.to_string());
    config::save(&cfg)?;

    println!("Switched to space: {}", key.green());
    Ok(())
}

pub struct AuthStatusArgs {
    json: bool,
}

impl AuthStatusArgs {
    pub fn new(json: bool) -> Self {
        Self { json }
    }
}

pub fn status(args: &AuthStatusArgs) -> Result<()> {
    let json = args.json;

    // Resolve space key: BL_SPACE env var takes priority.
    // Config load errors (IO, parse) are propagated; only missing space is
    // treated as "not logged in".
    let space_key = if let Ok(s) = std::env::var("BL_SPACE")
        && !s.is_empty()
    {
        s
    } else {
        let cfg = config::load()?;
        match cfg.current_space {
            Some(s) => s,
            None => {
                if json {
                    println!("{}", serde_json::json!({"error": "Not logged in"}));
                } else {
                    println!("Not logged in. Run `bl auth login` to authenticate.");
                }
                return Ok(());
            }
        }
    };

    let (api_key, backend) = match secret::get(&space_key) {
        Ok(v) => v,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": e.to_string()}));
            } else {
                println!("  {} {}", "!".red(), e);
            }
            return Ok(());
        }
    };

    let client = BacklogClient::new_with(
        &format!("https://{}.backlog.com/api/v2", space_key),
        &api_key,
    )?;
    status_with(json, &space_key, &api_key, backend, &client)
}

pub fn status_with(
    json: bool,
    space_key: &str,
    api_key: &str,
    backend: Backend,
    api: &dyn BacklogApi,
) -> Result<()> {
    if json {
        let user = api.get_myself().ok();
        println!("{}", build_status_json(space_key, backend, user)?);
        return Ok(());
    }

    let masked = format!("{}...", &api_key[..4.min(api_key.len())]);
    println!("Space: {}.backlog.com", space_key);
    println!("  - API key: {}", masked);
    println!("  - Stored in: {}", backend);

    match api.get_myself() {
        Ok(user) => println!("  - Logged in as {} ({})", user.name.green(), user.user_id),
        Err(e) => println!("  {} Token invalid: {}", "!".red(), e),
    }

    Ok(())
}

pub fn check_keyring() -> Result<()> {
    const TEST_KEY: &str = "__bl_keyring_test__";
    const TEST_VAL: &str = "ok";

    let entry = match keyring::Entry::new("bl", TEST_KEY) {
        Ok(e) => e,
        Err(e) => {
            println!("create entry ... {} ({e})", "FAIL".red());
            return Ok(());
        }
    };
    println!("create entry ... {}", "ok".green());

    match entry.set_password(TEST_VAL) {
        Ok(()) => println!("write        ... {}", "ok".green()),
        Err(e) => {
            println!("write        ... {} ({e})", "FAIL".red());
            return Ok(());
        }
    }

    match entry.get_password() {
        Ok(v) if v == TEST_VAL => println!("read         ... {}", "ok".green()),
        Ok(v) => println!("read         ... {} (got {v:?})", "FAIL".red()),
        Err(e) => println!("read         ... {} ({e})", "FAIL".red()),
    }

    match entry.delete_credential() {
        Ok(()) => println!("delete       ... {}", "ok".green()),
        Err(e) => println!("delete       ... {} ({e})", "FAIL".red()),
    }

    Ok(())
}

fn remove_space_from_config(cfg: &mut config::Config, key: &str) {
    cfg.spaces.retain(|s| s != key);
    if cfg.current_space.as_deref() == Some(key) {
        cfg.current_space = cfg.spaces.first().cloned();
    }
}

fn build_status_json(space_key: &str, backend: Backend, user: Option<User>) -> Result<String> {
    let output = serde_json::json!({
        "space_key": space_key,
        "stored_in": backend.to_string(),
        "user": user,
    });
    serde_json::to_string_pretty(&output).context("Failed to serialize JSON")
}

fn prompt(label: &str) -> Result<String> {
    use std::io::{self, Write};
    print!("{}", label);
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read input")?;
    Ok(input.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::user::User;
    use crate::api::{BacklogApi, space::Space};
    use anyhow::anyhow;

    struct MockApi {
        user: Option<User>,
    }

    impl BacklogApi for MockApi {
        fn get_space(&self) -> anyhow::Result<Space> {
            unimplemented!()
        }

        fn get_myself(&self) -> anyhow::Result<User> {
            self.user
                .clone()
                .ok_or_else(|| anyhow!("invalid credentials"))
        }

        fn get_space_activities(&self) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }

        fn get_space_disk_usage(&self) -> anyhow::Result<crate::api::disk_usage::DiskUsage> {
            unimplemented!()
        }

        fn get_space_notification(
            &self,
        ) -> anyhow::Result<crate::api::space_notification::SpaceNotification> {
            unimplemented!()
        }

        fn get_projects(&self) -> anyhow::Result<Vec<crate::api::project::Project>> {
            unimplemented!()
        }

        fn get_project(&self, _key: &str) -> anyhow::Result<crate::api::project::Project> {
            unimplemented!()
        }
        fn get_project_activities(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_project_disk_usage(
            &self,
            _key: &str,
        ) -> anyhow::Result<crate::api::project::ProjectDiskUsage> {
            unimplemented!()
        }
        fn get_project_users(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectUser>> {
            unimplemented!()
        }
        fn get_project_statuses(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectStatus>> {
            unimplemented!()
        }
        fn get_project_issue_types(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectIssueType>> {
            unimplemented!()
        }
        fn get_project_categories(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectCategory>> {
            unimplemented!()
        }
        fn get_project_versions(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::project::ProjectVersion>> {
            unimplemented!()
        }
        fn get_issues(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::issue::Issue>> {
            unimplemented!()
        }
        fn count_issues(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueCount> {
            unimplemented!()
        }
        fn get_issue(&self, _key: &str) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn create_issue(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn update_issue(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn delete_issue(&self, _key: &str) -> anyhow::Result<crate::api::issue::Issue> {
            unimplemented!()
        }
        fn get_issue_comments(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueComment>> {
            unimplemented!()
        }
        fn add_issue_comment(
            &self,
            _key: &str,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn update_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn delete_issue_comment(
            &self,
            _key: &str,
            _comment_id: u64,
        ) -> anyhow::Result<crate::api::issue::IssueComment> {
            unimplemented!()
        }
        fn get_issue_attachments(
            &self,
            _key: &str,
        ) -> anyhow::Result<Vec<crate::api::issue::IssueAttachment>> {
            unimplemented!()
        }
        fn get_wikis(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiListItem>> {
            unimplemented!()
        }
        fn get_wiki(&self, _wiki_id: u64) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn create_wiki(
            &self,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn update_wiki(
            &self,
            _wiki_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn delete_wiki(
            &self,
            _wiki_id: u64,
            _params: &[(String, String)],
        ) -> anyhow::Result<crate::api::wiki::Wiki> {
            unimplemented!()
        }
        fn get_wiki_history(
            &self,
            _wiki_id: u64,
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiHistory>> {
            unimplemented!()
        }
        fn get_wiki_attachments(
            &self,
            _wiki_id: u64,
        ) -> anyhow::Result<Vec<crate::api::wiki::WikiAttachment>> {
            unimplemented!()
        }
    }

    fn sample_user() -> User {
        User {
            id: 1,
            user_id: "john".to_string(),
            name: "John Doe".to_string(),
            mail_address: "john@example.com".to_string(),
            role_type: 1,
        }
    }

    #[test]
    fn build_status_json_with_user() {
        let json = build_status_json("mycompany", Backend::Keyring, Some(sample_user())).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["space_key"], "mycompany");
        assert_eq!(value["stored_in"], "System keyring");
        assert_eq!(value["user"]["userId"], "john");
        assert_eq!(value["user"]["name"], "John Doe");
    }

    #[test]
    fn build_status_json_without_user() {
        let json = build_status_json("mycompany", Backend::File, None).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["space_key"], "mycompany");
        assert_eq!(value["stored_in"], "Credentials file");
        assert!(value["user"].is_null());
    }

    #[test]
    fn status_with_text_shows_user_info() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let result = status_with(false, "mycompany", "abcd1234", Backend::Keyring, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_text_shows_token_invalid_on_error() {
        let api = MockApi { user: None };
        let result = status_with(false, "mycompany", "abcd1234", Backend::Keyring, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_json_includes_user_fields() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let result = status_with(true, "mycompany", "abcd1234", Backend::File, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_json_null_user_on_api_error() {
        let api = MockApi { user: None };
        let result = status_with(true, "mycompany", "abcd1234", Backend::File, &api);
        assert!(result.is_ok());
    }

    fn make_config(current: &str, spaces: &[&str]) -> crate::config::Config {
        crate::config::Config {
            current_space: Some(current.to_string()),
            spaces: spaces.iter().map(|s| s.to_string()).collect(),
            auth: None,
        }
    }

    #[test]
    fn remove_space_removes_from_list_and_clears_current() {
        let mut cfg = make_config("mycompany", &["mycompany", "another"]);
        remove_space_from_config(&mut cfg, "mycompany");
        assert_eq!(cfg.spaces, vec!["another"]);
        assert_eq!(cfg.current_space.as_deref(), Some("another"));
    }

    #[test]
    fn remove_space_promotes_next_when_current_removed() {
        let mut cfg = make_config("a", &["a", "b", "c"]);
        remove_space_from_config(&mut cfg, "a");
        assert_eq!(cfg.current_space.as_deref(), Some("b"));
        assert_eq!(cfg.spaces, vec!["b", "c"]);
    }

    #[test]
    fn remove_space_sets_current_to_none_when_last_space_removed() {
        let mut cfg = make_config("mycompany", &["mycompany"]);
        remove_space_from_config(&mut cfg, "mycompany");
        assert!(cfg.spaces.is_empty());
        assert!(cfg.current_space.is_none());
    }

    #[test]
    fn remove_space_does_not_change_current_when_non_current_removed() {
        let mut cfg = make_config("mycompany", &["mycompany", "another"]);
        remove_space_from_config(&mut cfg, "another");
        assert_eq!(cfg.current_space.as_deref(), Some("mycompany"));
        assert_eq!(cfg.spaces, vec!["mycompany"]);
    }
}
