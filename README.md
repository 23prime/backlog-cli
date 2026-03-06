# backlog-cli

An unofficial CLI tool for [Backlog](https://backlog.com).

## Installation

```bash
cargo install --path .
```

## Usage

### Authentication

Issue an API key from Backlog personal settings → API, then login.

```bash
bl auth login
```

Show current auth status and verify credentials against the API.

```bash
bl auth status
```

Logout and remove stored credentials.

```bash
bl auth logout
```

### Space

```bash
bl space
```

## Credential storage

The API key is stored in the system keyring (GNOME Keyring / Keychain / Windows Credential Manager).
Non-sensitive metadata such as the space key is stored in `~/.config/bl/config.toml`.

## Development

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
