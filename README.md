# Backlog CLI

An unofficial CLI tool for [Nulab's Backlog](https://backlog.com).

## Features

- 🌐 **Cross-platform** — Runs on Linux, macOS, and Windows (x86\_64 / aarch64 / Apple Silicon)
- 🔐 **Flexible authentication** — API key or browser-based OAuth 2.0; credentials stored in the system keyring (GNOME Keyring, macOS Keychain, or Windows Credential Manager) with a file fallback
- 🏢 **Multi-space support** — Manage multiple Backlog spaces and switch between them with `bl auth use`
- 🔧 **JSON output** — All primary commands support `--json` for machine-readable output
- 🤖 **CI/CD friendly** — Inject credentials via `BL_API_KEY` and `BL_SPACE` environment variables; no interactive prompts needed
- 📦 **Single binary** — Just download and run; no extra setup required
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

For other installation methods (building from source, etc.), see the [Documentation](https://23prime.github.io/backlog-cli/installation).

## Usage

1. Authenticate — choose one method:

    ```bash
    bl auth login         # API key
    bl auth login-oauth   # OAuth 2.0 (browser-based)
    ```

2. Run commands.
    e.g.) Show your space information.

    ```bash
    bl space
    ```

See the [Documentation](https://23prime.github.io/backlog-cli/) for full documentation.

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
