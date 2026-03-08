use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Current active space key.
    pub current_space: Option<String>,
    /// All configured space keys.
    #[serde(default)]
    pub spaces: Vec<String>,
    /// Legacy auth field — read-only for migration, never written back.
    #[serde(skip_serializing)]
    pub auth: Option<LegacyAuthConfig>,
}

/// Old config format: `[auth] space_key = "..."`. Used only for migration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegacyAuthConfig {
    pub space_key: String,
}

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Could not determine config directory")?;
    Ok(config_dir.join("bl").join("config.toml"))
}

pub fn load() -> Result<Config> {
    let mut cfg = load_from(&config_path()?)?;
    migrate(&mut cfg);
    Ok(cfg)
}

pub fn save(config: &Config) -> Result<()> {
    save_to(&config_path()?, config)
}

pub fn remove_config_file() -> Result<()> {
    let path = config_path()?;
    if path.exists() {
        std::fs::remove_file(&path)
            .with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

/// Resolve the effective space key: `BL_SPACE` env var → `current_space` in config.
pub fn current_space_key() -> Result<String> {
    if let Ok(s) = std::env::var("BL_SPACE")
        && !s.is_empty()
    {
        return Ok(s);
    }
    load()?
        .current_space
        .context("No current space set. Run `bl auth login` or `bl auth use <space_key>`.")
}

/// Migrate old `[auth] space_key` format to the new multi-space format.
fn migrate(cfg: &mut Config) {
    if cfg.current_space.is_none()
        && let Some(auth) = cfg.auth.take()
    {
        if !cfg.spaces.contains(&auth.space_key) {
            cfg.spaces.push(auth.space_key.clone());
        }
        cfg.current_space = Some(auth.space_key);
    }
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
        assert!(cfg.current_space.is_none());
        assert!(cfg.spaces.is_empty());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        let cfg = Config {
            current_space: Some("mycompany".to_string()),
            spaces: vec!["mycompany".to_string(), "another".to_string()],
            auth: None,
        };
        save_to(&path, &cfg).unwrap();
        let loaded = load_from(&path).unwrap();
        assert_eq!(loaded.current_space.unwrap(), "mycompany");
        assert_eq!(loaded.spaces, vec!["mycompany", "another"]);
    }

    #[test]
    fn save_creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("dir").join("config.toml");
        let cfg = Config::default();
        save_to(&path, &cfg).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn migrate_from_legacy_auth_field() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        // Write old-format config
        std::fs::write(&path, "[auth]\nspace_key = \"mycompany\"\n").unwrap();
        let mut cfg = load_from(&path).unwrap();
        migrate(&mut cfg);
        assert_eq!(cfg.current_space.as_deref(), Some("mycompany"));
        assert_eq!(cfg.spaces, vec!["mycompany"]);
    }

    #[test]
    fn migrate_does_not_duplicate_space() {
        let mut cfg = Config {
            current_space: None,
            spaces: vec!["mycompany".to_string()],
            auth: Some(LegacyAuthConfig {
                space_key: "mycompany".to_string(),
            }),
        };
        migrate(&mut cfg);
        assert_eq!(cfg.spaces.len(), 1);
        assert_eq!(cfg.current_space.as_deref(), Some("mycompany"));
    }

    #[test]
    fn migrate_skips_when_current_space_already_set() {
        let mut cfg = Config {
            current_space: Some("other".to_string()),
            spaces: vec!["other".to_string()],
            auth: Some(LegacyAuthConfig {
                space_key: "mycompany".to_string(),
            }),
        };
        migrate(&mut cfg);
        // current_space should not change
        assert_eq!(cfg.current_space.as_deref(), Some("other"));
    }

    #[test]
    fn save_does_not_write_auth_field() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        let cfg = Config {
            current_space: Some("mycompany".to_string()),
            spaces: vec!["mycompany".to_string()],
            auth: None,
        };
        save_to(&path, &cfg).unwrap();
        let raw = std::fs::read_to_string(&path).unwrap();
        assert!(!raw.contains("auth"));
    }
}
