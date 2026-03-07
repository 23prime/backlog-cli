# backlog-cli

An unofficial CLI tool for [Backlog](https://backlog.com).

## Features

- 🌐 **Cross-platform** — Runs on Linux, macOS, and Windows (x86\_64 / aarch64 / Apple Silicon)
- 🔐 **Secure authentication** — API key stored in the system keyring (GNOME Keyring, macOS Keychain, or Windows Credential Manager); if the keyring is unavailable, falls back to a local `credentials.toml` file (owner-only permissions on Unix)
- 📦 **No OpenSSL dependency** — Built with rustls for a clean, portable binary
- ⚡ **Easy install** — Single-command installation via shell script or PowerShell

## Installation

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

### Windows

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1 | iex
```

## Usage

1. Issue an API key from Backlog personal settings > API.

2. Authenticate.

    ```bash
    bl auth login
    ```

3. Run commands.
    e.g.) Show your space information.

    ```bash
    bl space
    ```

See the [User Guide](docs/user-guide.md) for full documentation.

## Development

### Pre-requirements

- [mise](https://mise.jdx.dev)
- [rustup](https://rustup.rs)

### Commands

```bash
mise run setup   # Install tools
mise run check   # Lint / format / test
mise run fix     # Auto fix
```

### Release

```bash
mise run release -- patch   # Bump version (patch / minor / major) and tag
mise run tag-push           # Push tag to trigger CI release
```
