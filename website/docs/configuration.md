# Configuration

## Linux / macOS

| Location | Contents |
| --- | --- |
| `~/.config/bl/config.toml` | Space key (non-sensitive metadata) |
| System keyring | API key and OAuth tokens (primary; GNOME Keyring / Keychain) |
| `~/.config/bl/credentials.toml` | API key fallback (mode 0600, used when keyring is unavailable) |
| `~/.config/bl/oauth_tokens.toml` | OAuth token fallback (mode 0600, used when keyring is unavailable) |

## Windows

| Location | Contents |
| --- | --- |
| `%APPDATA%\bl\config.toml` | Space key (non-sensitive metadata) |
| Windows Credential Manager | API key and OAuth tokens (primary) |
| `%APPDATA%\bl\credentials.toml` | API key fallback (used when Credential Manager is unavailable) |
| `%APPDATA%\bl\oauth_tokens.toml` | OAuth token fallback (used when Credential Manager is unavailable) |

## Config file format

```toml
current_space = "mycompany"
spaces = ["mycompany", "another-company"]
```

Old configs using the `[auth] space_key` format are migrated automatically on first run.
