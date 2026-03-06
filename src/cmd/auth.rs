use anyhow::{Context, Result};

use crate::api::{BacklogClient, user::User};
use crate::config::{self, AuthConfig};
use crate::secret::{self, Backend};

pub fn login() -> Result<()> {
    let space_key = prompt("Backlog space key (e.g. mycompany for mycompany.backlog.com): ")?;
    let api_key = rpassword::prompt_password("API key: ").context("Failed to read API key")?;

    let backend = secret::set(&space_key, &api_key)?;

    let mut cfg = config::load()?;
    cfg.auth = Some(AuthConfig { space_key });
    config::save(&cfg)?;

    println!("Logged in successfully. (API key stored in {})", backend);
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
    println!("Logged out.");
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
                println!("  ! {}", e);
            }
            return Ok(());
        }
    };

    if json {
        let user = BacklogClient::from_config()
            .and_then(|c| c.get_myself())
            .ok();
        println!("{}", build_status_json(&auth.space_key, backend, user)?);
        return Ok(());
    }

    let masked = format!("{}...", &api_key[..4.min(api_key.len())]);
    println!("Space: {}.backlog.com", auth.space_key);
    println!("  - API key: {}", masked);
    println!("  - Stored in: {}", backend);

    // Verify credentials against the API
    match BacklogClient::from_config().and_then(|c| c.get_myself()) {
        Ok(user) => println!("  - Logged in as {} ({})", user.name, user.user_id),
        Err(e) => println!("  ! Token invalid: {}", e),
    }

    Ok(())
}

pub fn check_keyring() -> Result<()> {
    const TEST_KEY: &str = "__bl_keyring_test__";
    const TEST_VAL: &str = "ok";

    let entry = match keyring::Entry::new("bl", TEST_KEY) {
        Ok(e) => e,
        Err(e) => {
            println!("create entry ... FAIL ({e})");
            return Ok(());
        }
    };
    println!("create entry ... ok");

    match entry.set_password(TEST_VAL) {
        Ok(()) => println!("write        ... ok"),
        Err(e) => {
            println!("write        ... FAIL ({e})");
            return Ok(());
        }
    }

    match entry.get_password() {
        Ok(v) if v == TEST_VAL => println!("read         ... ok"),
        Ok(v) => println!("read         ... FAIL (got {v:?})"),
        Err(e) => println!("read         ... FAIL ({e})"),
    }

    match entry.delete_credential() {
        Ok(()) => println!("delete       ... ok"),
        Err(e) => println!("delete       ... FAIL ({e})"),
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
}
