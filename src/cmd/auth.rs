use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, user::User};
use crate::config::{self};
use crate::oauth::{self};
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

pub fn login_oauth(no_banner: bool, port: u16) -> Result<()> {
    if !no_banner {
        println!("Welcome to");
        crate::cmd::banner::print_banner();
    }

    println!("Backlog OAuth Login\n");
    println!("  Register an OAuth 2.0 application (Confidential Client) at:");
    println!(
        "    {}",
        "https://backlog.com/developer/applications/oauth2Clients/add".bold()
    );
    println!(
        "    Redirect URI: {}\n",
        format!("http://127.0.0.1:{port}/callback").bold()
    );

    let space_key = prompt("Space key (e.g. mycompany for mycompany.backlog.com): ")?;
    let client_id = prompt("Client ID: ")?;
    let client_secret =
        rpassword::prompt_password("Client secret: ").context("Failed to read client secret")?;

    let tokens = oauth::run_oauth_flow(&space_key, &client_id, &client_secret, port)?;
    secret::set_oauth_tokens(&space_key, &tokens)?;

    let mut cfg = config::load()?;
    if !cfg.spaces.contains(&space_key) {
        cfg.spaces.push(space_key.clone());
    }
    cfg.current_space = Some(space_key);
    config::save(&cfg)?;

    println!("{}", "Logged in successfully via OAuth.".green());
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
    secret::delete_oauth_tokens(&key)?;
    remove_space_from_config(&mut cfg, &key);
    config::save(&cfg)?;

    println!("{} from {}", "Logged out".yellow(), key);
    Ok(())
}

pub fn logout_all() -> Result<()> {
    let cfg = config::load()?;

    for key in &cfg.spaces {
        secret::delete(key)?;
        secret::delete_oauth_tokens(key)?;
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

/// Auth information displayed by `bl auth status`.
pub enum AuthDisplay {
    ApiKey {
        masked: String,
        backend: Backend,
    },
    OAuth {
        masked_token: String,
        client_id: String,
        masked_client_secret: String,
        backend: Backend,
    },
}

pub fn status(args: &AuthStatusArgs) -> Result<()> {
    let json = args.json;

    // Resolve space key: BL_SPACE env var takes priority.
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

    // OAuth tokens take priority over API key.
    if let Ok((tokens, backend)) = secret::get_oauth_tokens(&space_key) {
        let auth = AuthDisplay::OAuth {
            masked_token: format!(
                "{}...",
                tokens.access_token.chars().take(4).collect::<String>()
            ),
            client_id: tokens.client_id.clone(),
            masked_client_secret: format!(
                "{}...",
                tokens.client_secret.chars().take(4).collect::<String>()
            ),
            backend,
        };
        // Build a client that will use the stored OAuth tokens.
        let client = match BacklogClient::from_config() {
            Ok(c) => c,
            Err(e) => {
                if json {
                    println!("{}", serde_json::json!({"error": e.to_string()}));
                } else {
                    println!("  {} {}", "!".red(), e);
                }
                return Ok(());
            }
        };
        return status_with(json, &space_key, &auth, &client);
    }

    // Fall back to API key.
    let (api_key, backend) = match secret::current_api_key(&space_key) {
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

    let auth = AuthDisplay::ApiKey {
        masked: format!("{}...", api_key.chars().take(4).collect::<String>()),
        backend,
    };
    let client = BacklogClient::new_with(
        &format!("https://{}.backlog.com/api/v2", space_key),
        &api_key,
    )?;
    status_with(json, &space_key, &auth, &client)
}

pub fn status_with(
    json: bool,
    space_key: &str,
    auth: &AuthDisplay,
    api: &dyn BacklogApi,
) -> Result<()> {
    if json {
        let user = api.get_myself().ok();
        println!("{}", build_status_json(space_key, auth, user)?);
        return Ok(());
    }

    println!("Space: {}.backlog.com", space_key);
    match auth {
        AuthDisplay::ApiKey { masked, backend } => {
            println!("  - Auth method: API key");
            println!("  - API key: {}", masked);
            println!("  - Stored in: {}", backend);
        }
        AuthDisplay::OAuth {
            masked_token,
            client_id,
            masked_client_secret,
            backend,
        } => {
            println!("  - Auth method: OAuth 2.0");
            println!("  - Client ID: {}", client_id);
            println!("  - Client Secret: {}", masked_client_secret);
            println!("  - Access token: {}", masked_token);
            println!("  - Stored in: {}", backend);
        }
    }

    match api.get_myself() {
        Ok(user) => println!(
            "  - Logged in as {} ({})",
            user.name.green(),
            user.user_id.as_deref().unwrap_or("-")
        ),
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

fn build_status_json(space_key: &str, auth: &AuthDisplay, user: Option<User>) -> Result<String> {
    let output = match auth {
        AuthDisplay::ApiKey { backend, .. } => serde_json::json!({
            "space_key": space_key,
            "auth_method": "api_key",
            "stored_in": backend.to_string(),
            "user": user,
        }),
        AuthDisplay::OAuth { client_id, .. } => serde_json::json!({
            "space_key": space_key,
            "auth_method": "oauth",
            "client_id": client_id,
            "user": user,
        }),
    };
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

        fn get_users(&self) -> anyhow::Result<Vec<User>> {
            unimplemented!()
        }

        fn get_user(&self, _user_id: u64) -> anyhow::Result<User> {
            unimplemented!()
        }

        fn get_space_activities(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
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
            _: &[(String, String)],
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
        fn get_teams(&self, _: &[(String, String)]) -> anyhow::Result<Vec<crate::api::team::Team>> {
            unimplemented!()
        }
        fn get_team(&self, _team_id: u64) -> anyhow::Result<crate::api::team::Team> {
            unimplemented!()
        }
        fn get_user_activities(
            &self,
            _user_id: u64,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::activity::Activity>> {
            unimplemented!()
        }
        fn get_recently_viewed_issues(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::user::RecentlyViewedIssue>> {
            unimplemented!()
        }
        fn get_notifications(
            &self,
            _: &[(String, String)],
        ) -> anyhow::Result<Vec<crate::api::notification::Notification>> {
            unimplemented!()
        }
        fn count_notifications(
            &self,
        ) -> anyhow::Result<crate::api::notification::NotificationCount> {
            unimplemented!()
        }
        fn read_notification(&self, _: u64) -> anyhow::Result<()> {
            unimplemented!()
        }
        fn reset_unread_notifications(
            &self,
        ) -> anyhow::Result<crate::api::notification::NotificationCount> {
            unimplemented!()
        }
    }

    fn sample_user() -> User {
        User {
            id: 1,
            user_id: Some("john".to_string()),
            name: "John Doe".to_string(),
            mail_address: Some("john@example.com".to_string()),
            role_type: 1,
            lang: None,
            last_login_time: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    fn api_key_auth(backend: Backend) -> AuthDisplay {
        AuthDisplay::ApiKey {
            masked: "abcd...".to_string(),
            backend,
        }
    }

    fn oauth_auth() -> AuthDisplay {
        AuthDisplay::OAuth {
            masked_token: "toke...".to_string(),
            client_id: "my-client-id".to_string(),
            masked_client_secret: "my-c...".to_string(),
            backend: Backend::Keyring,
        }
    }

    #[test]
    fn build_status_json_with_user() {
        let auth = api_key_auth(Backend::Keyring);
        let json = build_status_json("mycompany", &auth, Some(sample_user())).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["space_key"], "mycompany");
        assert_eq!(value["auth_method"], "api_key");
        assert_eq!(value["stored_in"], "System keyring");
        assert_eq!(value["user"]["userId"], "john");
        assert_eq!(value["user"]["name"], "John Doe");
    }

    #[test]
    fn build_status_json_without_user() {
        let auth = api_key_auth(Backend::File);
        let json = build_status_json("mycompany", &auth, None).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["space_key"], "mycompany");
        assert_eq!(value["auth_method"], "api_key");
        assert_eq!(value["stored_in"], "Credentials file");
        assert!(value["user"].is_null());
    }

    #[test]
    fn build_status_json_with_env_backend() {
        let auth = api_key_auth(Backend::Env);
        let json = build_status_json("mycompany", &auth, Some(sample_user())).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["stored_in"], "Environment variable");
        assert_eq!(value["user"]["userId"], "john");
    }

    #[test]
    fn build_status_json_with_oauth() {
        let auth = oauth_auth();
        let json = build_status_json("mycompany", &auth, Some(sample_user())).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["space_key"], "mycompany");
        assert_eq!(value["auth_method"], "oauth");
        assert_eq!(value["client_id"], "my-client-id");
        assert_eq!(value["user"]["userId"], "john");
    }

    #[test]
    fn status_with_text_shows_user_info() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let auth = api_key_auth(Backend::Keyring);
        let result = status_with(false, "mycompany", &auth, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_text_shows_token_invalid_on_error() {
        let api = MockApi { user: None };
        let auth = api_key_auth(Backend::Keyring);
        let result = status_with(false, "mycompany", &auth, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_json_includes_user_fields() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let auth = api_key_auth(Backend::File);
        let result = status_with(true, "mycompany", &auth, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_json_null_user_on_api_error() {
        let api = MockApi { user: None };
        let auth = api_key_auth(Backend::File);
        let result = status_with(true, "mycompany", &auth, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_oauth_text_shows_method() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let auth = oauth_auth();
        let result = status_with(false, "mycompany", &auth, &api);
        assert!(result.is_ok());
    }

    #[test]
    fn status_with_oauth_json_includes_client_id() {
        let api = MockApi {
            user: Some(sample_user()),
        };
        let auth = oauth_auth();
        let result = status_with(true, "mycompany", &auth, &api);
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
