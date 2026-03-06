use anyhow::{Context, Result};

const SERVICE: &str = "bl";

pub fn set(space_key: &str, api_key: &str) -> Result<()> {
    keyring::Entry::new(SERVICE, space_key)
        .context("Failed to access keyring")?
        .set_password(api_key)
        .context("Failed to store API key in keyring")
}

pub fn get(space_key: &str) -> Result<String> {
    keyring::Entry::new(SERVICE, space_key)
        .context("Failed to access keyring")?
        .get_password()
        .context("Failed to retrieve API key from keyring")
}

pub fn delete(space_key: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE, space_key).context("Failed to access keyring")?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e).context("Failed to delete API key from keyring"),
    }
}
