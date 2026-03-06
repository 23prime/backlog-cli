use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub auth: Option<AuthConfig>,
}

/// Non-sensitive auth metadata stored in config.toml.
/// The API key is stored separately in the system keyring.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub space_key: String,
}

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Could not determine config directory")?;
    Ok(config_dir.join("bl").join("config.toml"))
}

pub fn load() -> Result<Config> {
    load_from(&config_path()?)
}

pub fn save(config: &Config) -> Result<()> {
    save_to(&config_path()?, config)
}

pub fn load_from(path: &std::path::Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path.display()))?;
    let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;
    Ok(config)
}

pub fn save_to(path: &std::path::Path, config: &Config) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory {}", parent.display()))?;
    }
    let contents = toml::to_string_pretty(config).context("Failed to serialize config")?;
    std::fs::write(path, contents)
        .with_context(|| format!("Failed to write config to {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_returns_default_when_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        let cfg = load_from(&path).unwrap();
        assert!(cfg.auth.is_none());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        let cfg = Config {
            auth: Some(AuthConfig {
                space_key: "mycompany".to_string(),
            }),
        };
        save_to(&path, &cfg).unwrap();
        let loaded = load_from(&path).unwrap();
        assert_eq!(loaded.auth.unwrap().space_key, "mycompany");
    }

    #[test]
    fn save_creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("dir").join("config.toml");
        let cfg = Config::default();
        save_to(&path, &cfg).unwrap();
        assert!(path.exists());
    }
}
