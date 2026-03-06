use anyhow::{Context, Result};

use crate::api::BacklogClient;
use crate::config::{self, AuthConfig};
use crate::secret;

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

pub fn status() -> Result<()> {
    let cfg = config::load()?;
    let Some(auth) = cfg.auth else {
        println!("Not logged in. Run `bl auth login` to authenticate.");
        return Ok(());
    };

    println!("Space: {}.backlog.com", auth.space_key);

    let (api_key, backend) = match secret::get(&auth.space_key) {
        Ok(v) => v,
        Err(e) => {
            println!("  ! {}", e);
            return Ok(());
        }
    };

    let masked = format!("{}...", &api_key[..4.min(api_key.len())]);
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
