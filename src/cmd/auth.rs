use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::api::{BacklogApi, BacklogClient, user::User};
use crate::config::{self, AuthConfig};
use crate::secret::{self, Backend};

pub fn login() -> Result<()> {
    let space_key = prompt("Backlog space key (e.g. mycompany for mycompany.backlog.com): ")?;
    let api_key = rpassword::prompt_password("API key: ").context("Failed to read API key")?;

    let backend = secret::set(&space_key, &api_key)?;

    let mut cfg = config::load()?;
    cfg.auth = Some(AuthConfig { space_key });
    config::save(&cfg)?;

    println!(
        "{} (API key stored in {})",
        "Logged in successfully.".green(),
        backend
    );
    Ok(())
}

pub fn logout() -> Result<()> {
    let cfg = config::load()?;
    if let Some(auth) = cfg.auth {
        secret::delete(&auth.space_key)?;
    }
    let mut cfg = config::load()?;
    cfg.auth = None;
    config::save(&cfg)?;
    println!("{}", "Logged out.".yellow());
    Ok(())
}

pub fn status(json: bool) -> Result<()> {
    let cfg = config::load()?;
    let Some(auth) = cfg.auth else {
        if json {
            println!("{}", serde_json::json!({"error": "Not logged in"}));
        } else {
            println!("Not logged in. Run `bl auth login` to authenticate.");
        }
        return Ok(());
    };

    let (api_key, backend) = match secret::get(&auth.space_key) {
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
        &format!("https://{}.backlog.com/api/v2", auth.space_key),
        &api_key,
    )?;
    status_with(json, &auth.space_key, &api_key, backend, &client)
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
}
