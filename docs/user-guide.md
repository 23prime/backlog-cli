# User Guide

## Table of contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Uninstallation](#uninstallation)
- [Authentication](#authentication)
- [Commands](#commands)
- [Command coverage](#command-coverage)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Prerequisites

- A [Backlog](https://backlog.com) account with access to at least one space
- A Backlog API key (see [Authentication](#authentication))

## Installation

### Supported platforms

| OS | Architecture |
| --- | --- |
| Linux | x86\_64, aarch64 |
| macOS | x86\_64 (Intel), arm64 (Apple Silicon) |
| Windows | x86\_64 |

### Using the install script (Linux, macOS)

Requires `curl` and `tar`. The script auto-detects your OS and architecture, selects the matching binary,
and verifies its SHA-256 checksum before installing.

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

The binary is installed to `~/.local/bin/bl` by default.
To install to a different location, set the `INSTALL_DIR` environment variable:

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

### Using the install script (Windows)

Requires PowerShell 5.1 or later (built-in on Windows 10/11).

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1 | iex
```

The binary is installed to `%USERPROFILE%\.local\bin\bl.exe` by default.
To install to a different location:

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1))) -InstallDir 'C:\Tools'
```

### Building from source

```bash
git clone https://github.com/23prime/backlog-cli.git
cd backlog-cli
cargo install --path .
```

## Uninstallation

### Uninstall script (Linux, macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh
```

To also remove stored credentials and configuration files, pass `--purge`:

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh -s -- --purge
```

### Uninstall script (Windows)

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1 | iex
```

To also remove stored credentials and configuration files, pass `-Purge`:

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1))) -Purge
```

> **Note:** `--purge` / `-Purge` runs `bl auth logout` before removing the binary,
> which clears the API key from the system keyring and deletes the configuration directory.
> Without this flag, only the binary is removed and credentials are left intact
> (useful if you plan to reinstall later).

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

### `bl space activities`

Show recent activities in your Backlog space.

```bash
bl space activities
bl space activities --json
```

Example output:

```text
[123] type=1 project=TEST user=John Doe created=2024-06-01T00:00:00Z
[124] type=2 project=TEST user=Jane Smith created=2024-06-02T00:00:00Z
```

### `bl space disk-usage`

Show disk usage of your Backlog space.
Requires Space Administrator privileges. Non-admin users will receive `403 Forbidden`.

```bash
bl space disk-usage
bl space disk-usage --json
```

Example output:

```text
Capacity:   5242880 bytes
Issue:      2048 bytes
Wiki:       512 bytes
File:       1024 bytes
Subversion: 64 bytes
Git:        256 bytes
Git LFS:    128 bytes
Details:    3 project(s) — use --json for breakdown
```

### `bl space notification`

Show the notification message set for your Backlog space.

```bash
bl space notification
bl space notification --json
```

Example output:

```text
Updated: 2024-06-18T07:55:37Z

Scheduled maintenance on 2024-07-01.
```

When no notification has been set:

```text
Updated: (not set)

(no notification set)
```

### `bl project list`

List all projects you have access to.

```bash
bl project list
bl project list --json
```

Example output:

```text
[TEST] Test Project
[PROD] Production [archived]
```

### `bl project show`

Show details of a specific project.

```bash
bl project show <id-or-key>
bl project show <id-or-key> --json
```

Example output:

```text
ID:         1
Key:        TEST
Name:       Test Project
Formatting: markdown
Archived:   false
```

## Command coverage

The table below maps Backlog API v2 endpoints to `bl` commands.
Commands that target a specific project accept a `--project <key>` flag.

### Space

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl space` | `GET /api/v2/space` | ✅ Implemented |
| `bl space activities` | `GET /api/v2/space/activities` | ✅ Implemented |
| `bl space disk-usage` | `GET /api/v2/space/diskUsage` | ✅ Implemented |
| `bl space notification` | `GET /api/v2/space/notification` | ✅ Implemented |

### Projects

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl project list` | `GET /api/v2/projects` | ✅ Implemented |
| `bl project show <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}` | ✅ Implemented |
| `bl project activities <key>` | `GET /api/v2/projects/{projectIdOrKey}/activities` | Planned |
| `bl project disk-usage <key>` | `GET /api/v2/projects/{projectIdOrKey}/diskUsage` | Planned |
| `bl project user list <key>` | `GET /api/v2/projects/{projectIdOrKey}/users` | Planned |
| `bl project status list <key>` | `GET /api/v2/projects/{projectIdOrKey}/statuses` | Planned |
| `bl project issue-type list <key>` | `GET /api/v2/projects/{projectIdOrKey}/issueTypes` | Planned |
| `bl project category list <key>` | `GET /api/v2/projects/{projectIdOrKey}/categories` | Planned |
| `bl project version list <key>` | `GET /api/v2/projects/{projectIdOrKey}/versions` | Planned |

### Issues

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl issue list` | `GET /api/v2/issues` | Planned |
| `bl issue count` | `GET /api/v2/issues/count` | Planned |
| `bl issue show <id>` | `GET /api/v2/issues/{issueIdOrKey}` | Planned |
| `bl issue create` | `POST /api/v2/issues` | Planned |
| `bl issue update <id>` | `PATCH /api/v2/issues/{issueIdOrKey}` | Planned |
| `bl issue delete <id>` | `DELETE /api/v2/issues/{issueIdOrKey}` | Planned |
| `bl issue comment list <id>` | `GET /api/v2/issues/{issueIdOrKey}/comments` | Planned |
| `bl issue comment add <id>` | `POST /api/v2/issues/{issueIdOrKey}/comments` | Planned |
| `bl issue comment update <id> <comment-id>` | `PUT /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | Planned |
| `bl issue comment delete <id> <comment-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | Planned |
| `bl issue attachment list <id>` | `GET /api/v2/issues/{issueIdOrKey}/attachments` | Planned |

### Wiki

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl wiki list` | `GET /api/v2/wikis` | Planned |
| `bl wiki show <id>` | `GET /api/v2/wikis/{wikiId}` | Planned |
| `bl wiki create` | `POST /api/v2/wikis` | Planned |
| `bl wiki update <id>` | `PUT /api/v2/wikis/{wikiId}` | Planned |
| `bl wiki delete <id>` | `DELETE /api/v2/wikis/{wikiId}` | Planned |
| `bl wiki history <id>` | `GET /api/v2/wikis/{wikiId}/history` | Planned |
| `bl wiki attachment list <id>` | `GET /api/v2/wikis/{wikiId}/attachments` | Planned |

### Pull Requests

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl pr list` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests` | Planned |
| `bl pr show <number>` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests/{number}` | Planned |
| `bl pr create` | `POST /api/v2/projects/{projectIdOrKey}/pullRequests` | Planned |
| `bl pr update <number>` | `PUT /api/v2/projects/{projectIdOrKey}/pullRequests/{number}` | Planned |
| `bl pr comment list <number>` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments` | Planned |
| `bl pr comment add <number>` | `POST /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments` | Planned |
| `bl pr comment update <number> <comment-id>` | `PUT /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments/{commentId}` | Planned |

### Git Repositories

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl git repo list` | `GET /api/v2/projects/{projectIdOrKey}/repositories` | Planned |
| `bl git repo show <repo>` | `GET /api/v2/projects/{projectIdOrKey}/repositories/{repoId}` | Planned |

### Users

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl auth status` | `GET /api/v2/users/myself` | ✅ Implemented (internal) |
| `bl user list` | `GET /api/v2/users` | Planned |
| `bl user show <id>` | `GET /api/v2/users/{userId}` | Planned |
| `bl user activities <id>` | `GET /api/v2/users/{userId}/activities` | Planned |
| `bl user recently-viewed <id>` | `GET /api/v2/users/{userId}/recentlyViewedIssues` | Planned |

### Notifications

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl notification list` | `GET /api/v2/notifications` | Planned |
| `bl notification read <id>` | `PUT /api/v2/notifications/{notificationId}` | Planned |
| `bl notification read-all` | `DELETE /api/v2/notifications/unread` | Planned |

### Watching

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl watch list` | `GET /api/v2/watching` | Planned |
| `bl watch add` | `POST /api/v2/watching` | Planned |
| `bl watch delete <id>` | `DELETE /api/v2/watching/{watchingId}` | Planned |

### Teams

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl team list` | `GET /api/v2/teams` | Planned |
| `bl team show <id>` | `GET /api/v2/teams/{teamId}` | Planned |

## Configuration

### Linux / macOS

| Location | Contents |
| --- | --- |
| `~/.config/bl/config.toml` | Space key (non-sensitive metadata) |
| System keyring | API key (primary; GNOME Keyring / Keychain) |
| `~/.config/bl/credentials.toml` | API key fallback (mode 0600, used when keyring is unavailable) |

### Windows

| Location | Contents |
| --- | --- |
| `%APPDATA%\bl\config.toml` | Space key (non-sensitive metadata) |
| Windows Credential Manager | API key (primary) |
| `%APPDATA%\bl\credentials.toml` | API key fallback (used when Credential Manager is unavailable) |

### Config file format

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

On macOS, the system Keychain is used. On Windows, the Windows Credential Manager is used.
If the Credential Manager is unavailable, `bl` falls back to `%APPDATA%\bl\credentials.toml`.

The `bl auth status` output shows which backend is in use:

```text
  - Stored in: System keyring
```

or

```text
  - Stored in: Credentials file
```
