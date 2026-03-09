use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::oauth::OAuthTokens;

const SERVICE: &str = "bl";
const OAUTH_SERVICE: &str = "bl-oauth";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Keyring,
    File,
    Env,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backend::Keyring => write!(f, "System keyring"),
            Backend::File => write!(f, "Credentials file"),
            Backend::Env => write!(f, "Environment variable"),
        }
    }
}

trait CredentialStore {
    fn backend(&self) -> Backend;
    fn set(&self, space_key: &str, api_key: &str) -> Result<()>;
    fn get(&self, space_key: &str) -> Result<String>;
    fn delete(&self, space_key: &str) -> Result<()>;
}

struct KeyringStore;

impl CredentialStore for KeyringStore {
    fn backend(&self) -> Backend {
        Backend::Keyring
    }

    fn set(&self, space_key: &str, api_key: &str) -> Result<()> {
        keyring::Entry::new(SERVICE, space_key)
            .context("Failed to access keyring")?
            .set_password(api_key)
            .context("Failed to store API key in keyring")
    }

    fn get(&self, space_key: &str) -> Result<String> {
        keyring::Entry::new(SERVICE, space_key)
            .context("Failed to access keyring")?
            .get_password()
            .context("Failed to retrieve API key from keyring")
    }

    fn delete(&self, space_key: &str) -> Result<()> {
        let entry = keyring::Entry::new(SERVICE, space_key).context("Failed to access keyring")?;
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e).context("Failed to delete API key from keyring"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Credentials {
    #[serde(default)]
    keys: std::collections::HashMap<String, String>,
}

struct FileStore {
    path: PathBuf,
}

impl FileStore {
    fn new() -> Result<Self> {
        let config_dir = dirs::config_dir().context("Could not determine config directory")?;
        Ok(Self {
            path: config_dir.join("bl").join("credentials.toml"),
        })
    }

    fn load(&self) -> Result<Credentials> {
        if !self.path.exists() {
            return Ok(Credentials::default());
        }
        let contents = std::fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read credentials from {}", self.path.display()))?;
        toml::from_str(&contents).context("Failed to parse credentials file")
    }

    fn save(&self, credentials: &Credentials) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory {}", parent.display())
            })?;
        }
        let contents =
            toml::to_string_pretty(credentials).context("Failed to serialize credentials")?;
        std::fs::write(&self.path, &contents)
            .with_context(|| format!("Failed to write credentials to {}", self.path.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&self.path, std::fs::Permissions::from_mode(0o600))
                .context("Failed to set credentials file permissions")?;
        }
        Ok(())
    }
}

impl CredentialStore for FileStore {
    fn backend(&self) -> Backend {
        Backend::File
    }

    fn set(&self, space_key: &str, api_key: &str) -> Result<()> {
        let mut credentials = self.load().unwrap_or_default();
        credentials
            .keys
            .insert(space_key.to_string(), api_key.to_string());
        self.save(&credentials)
    }

    fn get(&self, space_key: &str) -> Result<String> {
        let credentials = self.load()?;
        credentials
            .keys
            .get(space_key)
            .cloned()
            .with_context(|| format!("API key not found for space '{}'", space_key))
    }

    fn delete(&self, space_key: &str) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }
        let mut credentials = self.load()?;
        credentials.keys.remove(space_key);
        self.save(&credentials)
    }
}

fn default_stores() -> Result<Vec<Box<dyn CredentialStore>>> {
    Ok(vec![Box::new(KeyringStore), Box::new(FileStore::new()?)])
}

pub fn set(space_key: &str, api_key: &str) -> Result<Backend> {
    set_impl(space_key, api_key, &default_stores()?)
}

pub fn get(space_key: &str) -> Result<(String, Backend)> {
    get_impl(space_key, &default_stores()?)
}

/// Resolve the effective API key: `BL_API_KEY` env var → credential store.
pub fn current_api_key(space_key: &str) -> Result<(String, Backend)> {
    if let Ok(key) = std::env::var("BL_API_KEY")
        && !key.is_empty()
    {
        return Ok((key, Backend::Env));
    }
    get(space_key)
}

pub fn delete(space_key: &str) -> Result<()> {
    delete_impl(space_key, &default_stores()?)
}

pub fn remove_credentials_file() -> Result<()> {
    let store = FileStore::new()?;
    if store.path.exists() {
        std::fs::remove_file(&store.path)
            .with_context(|| format!("Failed to remove {}", store.path.display()))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// OAuth token storage
//
// Tokens are serialized as JSON and stored in the keyring under the service
// name "bl-oauth" with the space key as the username.  A file fallback
// (`oauth_tokens.toml`) is used when the keyring is unavailable.
// ---------------------------------------------------------------------------

pub fn get_oauth_tokens(space_key: &str) -> Result<(OAuthTokens, Backend)> {
    // Keyring first
    if let Ok(entry) = keyring::Entry::new(OAUTH_SERVICE, space_key)
        && let Ok(json) = entry.get_password()
    {
        return serde_json::from_str(&json)
            .context("Failed to deserialize OAuth tokens from keyring")
            .map(|tokens| (tokens, Backend::Keyring));
    }
    // File fallback
    oauth_file_get(space_key)
}

pub fn set_oauth_tokens(space_key: &str, tokens: &OAuthTokens) -> Result<()> {
    let json = serde_json::to_string(tokens).context("Failed to serialize OAuth tokens")?;
    if let Ok(entry) = keyring::Entry::new(OAUTH_SERVICE, space_key)
        && entry.set_password(&json).is_ok()
    {
        return Ok(());
    }
    oauth_file_set(space_key, tokens)
}

pub fn delete_oauth_tokens(space_key: &str) -> Result<()> {
    if let Ok(entry) = keyring::Entry::new(OAUTH_SERVICE, space_key) {
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => {}
            Err(e) => return Err(e).context("Failed to delete OAuth tokens from keyring"),
        }
    }
    oauth_file_delete(space_key)
}

// --- file fallback helpers --------------------------------------------------

fn oauth_file_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Could not determine config directory")?;
    Ok(config_dir.join("bl").join("oauth_tokens.toml"))
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct OAuthFile {
    #[serde(default)]
    tokens: std::collections::HashMap<String, OAuthTokens>,
}

fn oauth_file_load() -> Result<OAuthFile> {
    let path = oauth_file_path()?;
    if !path.exists() {
        return Ok(OAuthFile::default());
    }
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    toml::from_str(&contents).context("Failed to parse oauth_tokens.toml")
}

fn oauth_file_save(file: &OAuthFile) -> Result<()> {
    let path = oauth_file_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    let contents = toml::to_string_pretty(file).context("Failed to serialize oauth_tokens.toml")?;
    std::fs::write(&path, &contents)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
            .context("Failed to set oauth_tokens.toml permissions")?;
    }
    Ok(())
}

fn oauth_file_get(space_key: &str) -> Result<(OAuthTokens, Backend)> {
    let file = oauth_file_load()?;
    file.tokens
        .get(space_key)
        .cloned()
        .with_context(|| format!("OAuth tokens not found for space '{space_key}'"))
        .map(|tokens| (tokens, Backend::File))
}

fn oauth_file_set(space_key: &str, tokens: &OAuthTokens) -> Result<()> {
    let mut file = oauth_file_load()?;
    file.tokens.insert(space_key.to_string(), tokens.clone());
    oauth_file_save(&file)
}

fn oauth_file_delete(space_key: &str) -> Result<()> {
    let path = oauth_file_path()?;
    if !path.exists() {
        return Ok(());
    }
    let mut file = oauth_file_load()?;
    file.tokens.remove(space_key);
    oauth_file_save(&file)
}

fn set_impl(
    space_key: &str,
    api_key: &str,
    stores: &[Box<dyn CredentialStore>],
) -> Result<Backend> {
    for store in stores {
        if store.set(space_key, api_key).is_ok() {
            return Ok(store.backend());
        }
    }
    anyhow::bail!("No credential store available")
}

fn get_impl(space_key: &str, stores: &[Box<dyn CredentialStore>]) -> Result<(String, Backend)> {
    let mut last_err = anyhow::anyhow!("API key not found. Run `bl auth login` to authenticate.");
    for store in stores {
        match store.get(space_key) {
            Ok(key) => return Ok((key, store.backend())),
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

fn delete_impl(space_key: &str, stores: &[Box<dyn CredentialStore>]) -> Result<()> {
    for store in stores {
        let _ = store.delete(space_key);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn file_store(dir: &TempDir) -> Box<dyn CredentialStore> {
        Box::new(FileStore {
            path: dir.path().join("credentials.toml"),
        })
    }

    #[test]
    fn set_and_get_roundtrip_via_file() {
        let dir = TempDir::new().unwrap();
        let stores: Vec<Box<dyn CredentialStore>> = vec![file_store(&dir)];
        set_impl("mycompany", "my-api-key", &stores).unwrap();
        let (key, backend) = get_impl("mycompany", &stores).unwrap();
        assert_eq!(key, "my-api-key");
        assert_eq!(backend, Backend::File);
    }

    #[test]
    fn get_returns_error_when_key_missing() {
        let dir = TempDir::new().unwrap();
        let stores: Vec<Box<dyn CredentialStore>> = vec![file_store(&dir)];
        assert!(get_impl("mycompany", &stores).is_err());
    }

    #[test]
    fn delete_removes_key() {
        let dir = TempDir::new().unwrap();
        let stores: Vec<Box<dyn CredentialStore>> = vec![file_store(&dir)];
        set_impl("mycompany", "my-api-key", &stores).unwrap();
        delete_impl("mycompany", &stores).unwrap();
        assert!(get_impl("mycompany", &stores).is_err());
    }

    #[test]
    fn get_falls_back_to_second_store() {
        let dir1 = TempDir::new().unwrap();
        let dir2 = TempDir::new().unwrap();
        // Only the second store has the key
        FileStore {
            path: dir2.path().join("credentials.toml"),
        }
        .set("mycompany", "fallback-key")
        .unwrap();
        let stores: Vec<Box<dyn CredentialStore>> = vec![file_store(&dir1), file_store(&dir2)];
        let (key, backend) = get_impl("mycompany", &stores).unwrap();
        assert_eq!(key, "fallback-key");
        assert_eq!(backend, Backend::File);
    }

    #[test]
    fn set_uses_first_available_store() {
        let dir1 = TempDir::new().unwrap();
        let dir2 = TempDir::new().unwrap();
        let stores: Vec<Box<dyn CredentialStore>> = vec![file_store(&dir1), file_store(&dir2)];
        let backend = set_impl("mycompany", "my-api-key", &stores).unwrap();
        assert_eq!(backend, Backend::File);
        // Only the first store should have been written to
        assert!(get_impl("mycompany", &[file_store(&dir1)]).is_ok());
        assert!(get_impl("mycompany", &[file_store(&dir2)]).is_err());
    }

    #[test]
    fn set_multiple_keys() {
        let dir = TempDir::new().unwrap();
        let stores: Vec<Box<dyn CredentialStore>> = vec![file_store(&dir)];
        set_impl("space1", "key1", &stores).unwrap();
        set_impl("space2", "key2", &stores).unwrap();
        assert_eq!(get_impl("space1", &stores).unwrap().0, "key1");
        assert_eq!(get_impl("space2", &stores).unwrap().0, "key2");
    }

    #[test]
    fn backend_display() {
        assert_eq!(Backend::Keyring.to_string(), "System keyring");
        assert_eq!(Backend::File.to_string(), "Credentials file");
        assert_eq!(Backend::Env.to_string(), "Environment variable");
    }
}
