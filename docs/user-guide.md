# User Guide

## Table of contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Authentication](#authentication)
- [Commands](#commands)
- [Command coverage](#command-coverage)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Prerequisites

- A [Backlog](https://backlog.com) account with access to at least one space
- A Backlog API key (see [Authentication](#authentication))

## Installation

### Using the install script (Linux, macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

The binary is installed to `~/.local/bin/bl` by default.
To install to a different location, set the `INSTALL_DIR` environment variable:

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

### Building from source

```bash
git clone https://github.com/23prime/backlog-cli.git
cd backlog-cli
cargo install --path .
```

## Authentication

### Issuing an API key

1. Log in to your Backlog space
2. Go to **Personal settings** → **API**
3. Enter a memo and click **Submit**
4. Copy the generated API key

### Logging in

```bash
bl auth login
```

You will be prompted for:

- **Space key** — the subdomain of your Backlog space.
  For `mycompany.backlog.com`, enter `mycompany`.
- **API key** — the key issued in the step above (input is hidden)

### Checking auth status

```bash
bl auth status
```

This verifies your credentials against the Backlog API and shows:

```text
Space: mycompany.backlog.com
  - API key: abcd...
  - Stored in: System keyring
  - Logged in as Your Name (your-id)
```

### Logging out

```bash
bl auth logout
```

Removes the API key from the keyring and credentials file, and clears the stored space key.

## Commands

### `bl auth`

| Command | Description |
| --- | --- |
| `bl auth login` | Authenticate with a Backlog API key |
| `bl auth status` | Show current auth status and verify credentials |
| `bl auth logout` | Remove stored credentials |

### `bl space`

Show information about your Backlog space.

```bash
bl space
```

Example output:

```text
Space key:  mycompany
Name:       My Company
Language:   ja
Timezone:   Asia/Tokyo
Formatting: markdown
Created:    2020-01-01T00:00:00Z
Updated:    2024-06-01T00:00:00Z
```

## Command coverage

The table below maps Backlog API v2 endpoints to `bl` commands.

### Space

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl space` | `GET /api/v2/space` | ✅ Implemented |
| `bl space activities` | `GET /api/v2/space/activities` | Planned |
| `bl space disk-usage` | `GET /api/v2/space/diskUsage` | Planned |

### Issues

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl issue list` | `GET /api/v2/issues` | Planned |
| `bl issue show <id>` | `GET /api/v2/issues/{issueIdOrKey}` | Planned |
| `bl issue create` | `POST /api/v2/issues` | Planned |
| `bl issue update <id>` | `PATCH /api/v2/issues/{issueIdOrKey}` | Planned |
| `bl issue delete <id>` | `DELETE /api/v2/issues/{issueIdOrKey}` | Planned |
| `bl issue comment list <id>` | `GET /api/v2/issues/{issueIdOrKey}/comments` | Planned |
| `bl issue comment add <id>` | `POST /api/v2/issues/{issueIdOrKey}/comments` | Planned |

### Projects

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl project list` | `GET /api/v2/projects` | Planned |
| `bl project show <key>` | `GET /api/v2/projects/{projectIdOrKey}` | Planned |

### Wiki

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl wiki list` | `GET /api/v2/wikis` | Planned |
| `bl wiki show <id>` | `GET /api/v2/wikis/{wikiId}` | Planned |

### Users

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl auth status` | `GET /api/v2/users/myself` | ✅ Implemented (internal) |
| `bl user list` | `GET /api/v2/users` | Planned |
| `bl user show <id>` | `GET /api/v2/users/{userId}` | Planned |

## Configuration

| Location | Contents |
| --- | --- |
| `~/.config/bl/config.toml` | Space key (non-sensitive metadata) |
| System keyring | API key (primary; GNOME Keyring / Keychain) |
| `~/.config/bl/credentials.toml` | API key fallback (mode 0600, used when keyring is unavailable) |

### `~/.config/bl/config.toml`

```toml
[auth]
space_key = "mycompany"
```

## Troubleshooting

### `API key not found. Run bl auth login to authenticate.`

The API key is missing from the keyring. Run `bl auth login` again.

### `API error (401 Unauthorized): Authentication failure`

The space key or API key is incorrect. Check:

- The space key matches your Backlog URL (e.g. `mycompany` for `mycompany.backlog.com`)
- The API key is still valid in Backlog personal settings

Run `bl auth login` to re-enter your credentials.

### Keyring is unavailable

On Linux, the keyring requires a running Secret Service daemon (GNOME Keyring or KWallet).
If no daemon is available (e.g. headless or SSH environments), `bl` automatically falls back
to storing the API key in `~/.config/bl/credentials.toml` with mode 0600.

The `bl auth status` output shows which backend is in use:

```text
  - Stored in: System keyring
```

or

```text
  - Stored in: Credentials file
```
