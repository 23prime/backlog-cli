# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## General agent rules

- When users ask questions, answer them instead of doing the work.

### Shell Rules

- Always use `rm -f` (never bare `rm`)
- Before running a series of `git` commands, confirm you are in the project root; if not, `cd` there first. Then run all subsequent `git` commands from that directory without the `-C` option.

## Project Overview

Backlog CLI tool written in Rust. The main command is `bl`.

### Key commands

- `bl auth login` — Save space key and API key (stored in system keyring)
- `bl auth status` — Show auth info and verify credentials via API
- `bl auth logout` — Remove credentials
- `bl space` — Show space information

### Architecture

```text
src/
  main.rs         - CLI entry point (clap subcommands)
  config.rs       - ~/.config/bl/config.toml (non-sensitive metadata)
  secret.rs       - System keyring access via keyring crate
  api/
    mod.rs        - BacklogClient (Bearer / apiKey auth, loads config + keyring)
    space.rs      - GET /api/v2/space
    user.rs       - GET /api/v2/users/myself
  cmd/
    auth.rs       - auth login / status / logout
    space.rs      - space show
```

### Conventions

- Use `anyhow` for error handling throughout.
- API Key is stored in the system keyring; `space_key` is stored in `config.toml`.
- HTTP client uses `reqwest` (blocking) with `rustls-tls` (no OpenSSL dependency).
- `BacklogClient::from_config()` loads both config and keyring automatically.

### Build and release

```bash
cargo build --locked   # build
mise run check         # clippy + fmt + test
mise run release -- patch   # bump version and tag
mise run tag-push           # push tag to trigger CI release
```
