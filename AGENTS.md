# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## General agent rules

- When users ask questions, answer them instead of doing the work.
- When making changes to user-visible behavior (commands, output, storage, configuration),
  update `website/docs/` (and `website/i18n/ja/`) and `README.md` accordingly.

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
  secret.rs       - Credential storage: keyring (primary) with file fallback (~/.config/bl/credentials.toml)
  api/
    mod.rs        - BacklogClient + BacklogApi trait
    space.rs      - GET /api/v2/space → Space struct
    user.rs       - GET /api/v2/users/myself → User struct
  cmd/
    auth.rs       - auth login / status / logout
    space/
      mod.rs      - pub use re-exports
      show.rs     - space show
      activities.rs
      disk_usage.rs
      notification.rs
    project/
      mod.rs      - pub use re-exports
      list.rs     - project list
      show.rs     - project show
```

#### Data flow

```text
main.rs (clap) → cmd/* → BacklogClient::from_config() → BacklogClient::get() → Backlog API
```

`from_config()` loads `config.toml` for the space key, then retrieves the API key from the credential store.

#### Adding a new API endpoint

1. Add a new file `src/api/<resource>.rs` with the response struct and an `impl BacklogClient` block.
2. Add the method to the `BacklogApi` trait in `src/api/mod.rs`.
3. Add `pub mod <resource>` in `src/api/mod.rs`.

#### Adding a new command

Commands are organized as directories (`src/cmd/<resource>/`) with one file per subcommand.

1. Create `src/cmd/<resource>/<subcommand>.rs` with:
   - A public `<subcommand>(args…) -> Result<()>` that calls `BacklogClient::from_config()` and delegates.
   - A public `<subcommand>_with(args…, api: &dyn BacklogApi) -> Result<()>` for the real logic (testable).
2. Add `mod <subcommand>;` and `pub use <subcommand>::<subcommand>;` to `src/cmd/<resource>/mod.rs`.
3. Register the subcommand in `src/main.rs`.

#### Testing strategy

See [`docs/TESTING.md`](docs/TESTING.md) for the full testing guide.

Summary:

- **`api/` layer**: Use `httpmock` to spin up a local HTTP server; construct `BacklogClient::new_with(base_url, api_key)`.
- **`cmd/` layer**: Implement `BacklogApi` on a `MockApi` struct; call `*_with(json, &mock)` directly.
- Never call `BacklogClient::from_config()` in tests — it requires real credentials on disk.

### Conventions

#### General

- Use `anyhow` for error handling throughout.
- API key is stored via `CredentialStore` trait: keyring first, falling back to `~/.config/bl/credentials.toml` (0600).
- `space_key` is stored in `~/.config/bl/config.toml` (non-sensitive).
- HTTP client uses `reqwest` (blocking) with `rustls-tls` (no OpenSSL dependency).
- `BacklogClient::from_config()` loads both config and credentials automatically.

#### Validation layer boundaries

See [`docs/VALIDATION.md`](docs/VALIDATION.md) for the full validation guide.

Summary:

- **`main.rs` (clap)** — Syntactic/type-level checks only (e.g. "cannot parse as u64").
- **`Args::try_new`** — Domain invariants that must hold before any logic runs.
- **`cmd/*_with`** — API call logic and output formatting. API-spec constraints that depend on runtime data may also live here.
- **`api/`** — HTTP-level error translation only.

### Build, lint, and test

All Rust commands must be run via mise tasks — never call `cargo` directly.

```bash
mise run rs-build      # build
mise run rs-check      # clippy + fmt check + tests (Rust only)
mise run rs-fix        # auto-fix clippy and fmt
mise run check         # full check: run before committing (markdown + actions + spelling + rust)
mise run fix           # auto-fix all: run before committing
```

### Release

```bash
mise run release -- patch   # bump version and tag
mise run tag-push           # push tag to trigger CI release
```
