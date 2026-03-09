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
- A Backlog API key or OAuth 2.0 client credentials (see [Authentication](#authentication))

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

> **Note:** With `--purge` / `-Purge`, the uninstall script first runs `bl auth logout --all`,
> which clears all API keys from the system keyring and removes all configuration files,
> and then deletes the Backlog CLI configuration directory along with the binary.
> Without this flag, only the binary is removed and credentials are left intact
> (useful if you plan to reinstall later).
>
> You can also clean up credentials manually at any time with `bl auth logout --all`.

## Authentication

### Issuing an API key

1. Log in to your Backlog space
2. Go to **Personal settings** → **API**
3. Enter a memo and click **Submit**
4. Copy the generated API key

### Logging in with an API key

```bash
bl auth login
```

You will be prompted for:

- **Space key** — the subdomain of your Backlog space.
  For `mycompany.backlog.com`, enter `mycompany`.
- **API key** — the key issued in the step above (input is hidden)

Running `bl auth login` again with a different space key adds another space.
The most recently logged-in space becomes the current (active) space.

### Logging in with OAuth 2.0

`bl` supports browser-based OAuth 2.0 login as an alternative to API keys.

#### Step 1 — Register an OAuth application in Backlog

1. Open <https://backlog.com/developer/applications/oauth2Clients/add>
2. Create a new application:
   - **Application type**: Confidential Client
   - **Redirect URI**: `http://127.0.0.1:54321/callback`
     (use `http://127.0.0.1:<port>/callback` if you will pass `--port <port>`)
3. Note the **Client ID** and **Client Secret**

#### Step 2 — Run the OAuth login command

```bash
bl auth login-oauth
```

You will be prompted for:

- **Space key** — the subdomain of your Backlog space
- **Client ID** — from the registered application
- **Client Secret** — from the registered application (input is hidden)

The command opens your browser to the Backlog authorization page.
After you approve, the browser is redirected to `http://127.0.0.1:54321/callback`
and the access token is stored automatically.

To use a custom port (must match the Redirect URI registered in Backlog):

```bash
bl auth login-oauth --port 8080
```

### Managing multiple spaces

```bash
# List all configured spaces (* marks the current space)
bl auth list

# Switch the current space
bl auth use another-company

# Use a different space for a single command
bl --space another-company project list

# Or set the BL_SPACE environment variable
export BL_SPACE=another-company
bl project list

# Inject credentials via environment variables (useful in CI/CD)
export BL_SPACE=mycompany
export BL_API_KEY=your-api-key
bl project list
```

### Checking auth status

```bash
bl auth status
```

This verifies your credentials against the Backlog API and shows:

```text
Space: mycompany.backlog.com
  - Auth method: API key
  - API key: abcd...
  - Stored in: System keyring
  - Logged in as Your Name (your-id)
```

When authenticated via OAuth:

```text
Space: mycompany.backlog.com
  - Auth method: OAuth 2.0
  - Client ID: abc123
  - Client Secret: abcd...
  - Access token: abcd...
  - Logged in as Your Name (your-id)
```

When `BL_API_KEY` is set, `Stored in` shows `Environment variable`.

### Logging out

```bash
# Logout from the current space
bl auth logout

# Logout from a specific space
bl auth logout another-company

# Logout from all spaces and remove all config files (useful before uninstalling)
bl auth logout --all
```

## Commands

### Global options

| Option | Description |
| --- | --- |
| `--banner` | Print the Backlog CLI banner and exit |
| `--no-color` | Disable colored output |
| `--space <SPACE_KEY>` | Override the active space for this command |

### `bl auth`

| Command | Description |
| --- | --- |
| `bl auth login` | Authenticate with a Backlog API key (adds or updates a space); use `--no-banner` to skip the banner |
| `bl auth login-oauth` | Authenticate via browser-based OAuth 2.0; use `--port <port>` to override the default callback port (54321); use `--no-banner` to skip the banner |
| `bl auth status` | Show current auth status and verify credentials |
| `bl auth list` | List all configured spaces |
| `bl auth use <space-key>` | Switch the current space |
| `bl auth logout [<space-key>]` | Remove credentials for the current or specified space |
| `bl auth logout --all` | Remove all spaces and delete all config files |

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

### `bl project activities`

Show recent activities for a specific project.

```bash
bl project activities <id-or-key>
bl project activities <id-or-key> --json
```

Example output:

```text
[123] type=1 project=TEST user=John Doe created=2024-06-01T00:00:00Z
```

### `bl project disk-usage`

Show disk usage for a specific project.
Requires Space Administrator privileges. Non-admin users will receive `403 Forbidden`.

```bash
bl project disk-usage <id-or-key>
bl project disk-usage <id-or-key> --json
```

Example output:

```text
Issue:      2048 bytes
Wiki:       512 bytes
Document:   0 bytes
File:       1024 bytes
Subversion: 64 bytes
Git:        256 bytes
Git LFS:    128 bytes
```

### `bl project user list`

List users who are members of a specific project.

```bash
bl project user list <id-or-key>
bl project user list <id-or-key> --json
```

Example output:

```text
[john] John Doe
[jane] Jane Smith
```

### `bl project status list`

List issue statuses defined for a specific project.

```bash
bl project status list <id-or-key>
bl project status list <id-or-key> --json
```

Example output:

```text
[1] Open
[2] In Progress
[3] Resolved
[4] Closed
```

### `bl project issue-type list`

List issue types defined for a specific project.

```bash
bl project issue-type list <id-or-key>
bl project issue-type list <id-or-key> --json
```

Example output:

```text
[1] Bug
[2] Task
[3] Feature Request
```

### `bl project category list`

List categories defined for a specific project.

```bash
bl project category list <id-or-key>
bl project category list <id-or-key> --json
```

Example output:

```text
[11] Development
[12] Design
```

### `bl project version list`

List versions (milestones) defined for a specific project.

```bash
bl project version list <id-or-key>
bl project version list <id-or-key> --json
```

Example output:

```text
[3] Version 0.1 (2024-01-01T00:00:00Z → 2024-01-31T00:00:00Z)
[4] Version 0.2 [archived]
```

### `bl issue list`

List issues with optional filters.

```bash
bl issue list
bl issue list --project-id 1 --status-id 1
bl issue list --issue-type-id 1 --category-id 2 --milestone-id 3
bl issue list --parent-child not-child --keyword "login" --count 50
bl issue list --json
```

`--parent-child` values: `all`, `not-child`, `child`, `standalone`, `parent`.

Example output:

```text
[TEST-1] Fix login issue (Open, Normal, -)
[TEST-2] Add dark mode (In Progress, Normal, John Doe)
```

### `bl issue count`

Count issues with optional filters. Accepts the same filters as `bl issue list`.

```bash
bl issue count
bl issue count --project-id 1 --issue-type-id 1 --parent-child not-child --json
```

Example output:

```text
42
```

### `bl issue show`

Show details of a specific issue.

```bash
bl issue show <id-or-key>
bl issue show TEST-1 --json
```

Example output:

```text
TEST-1 Fix login issue
  Status:     Open
  Priority:   Normal
  Type:       Bug
  Assignee:   -
  Created:    2024-01-01T00:00:00Z
  Updated:    2024-06-01T00:00:00Z
```

### `bl issue create`

Create a new issue. Requires `--project-id`, `--summary`, `--issue-type-id`, and `--priority-id`.

```bash
bl issue create --project-id 1 --summary "Fix login" --issue-type-id 1 --priority-id 2
bl issue create --project-id 1 --summary "Bug" --issue-type-id 1 --priority-id 2 \
  --description "Details..." --assignee-id 123 --due-date 2024-12-31 --json
```

Priority IDs: `1` = High, `2` = Normal, `3` = Low.

### `bl issue update`

Update an existing issue. All fields are optional.

```bash
bl issue update TEST-1 --summary "Updated summary"
bl issue update TEST-1 --status-id 2 --comment "Fixed in v1.2" --json
```

### `bl issue delete`

Delete an issue.

```bash
bl issue delete TEST-1
bl issue delete TEST-1 --json
```

Example output:

```text
Deleted: TEST-1
```

### `bl issue comment list`

List comments on an issue.

```bash
bl issue comment list TEST-1
bl issue comment list TEST-1 --json
```

Example output:

```text
[1] John Doe (2024-01-01T00:00:00Z): Fixed the issue.
[2] Jane Smith (2024-01-02T00:00:00Z): Confirmed.
```

### `bl issue comment add`

Add a comment to an issue.

```bash
bl issue comment add TEST-1 --content "This is a comment"
bl issue comment add TEST-1 --content "Done" --json
```

### `bl issue comment update`

Update an existing comment.

```bash
bl issue comment update TEST-1 42 --content "Updated comment"
bl issue comment update TEST-1 42 --content "Fixed" --json
```

### `bl issue comment delete`

Delete a comment.

```bash
bl issue comment delete TEST-1 42
bl issue comment delete TEST-1 42 --json
```

### `bl issue attachment list`

List attachments on an issue.

```bash
bl issue attachment list TEST-1
bl issue attachment list TEST-1 --json
```

Example output:

```text
[1] screenshot.png (204800 bytes)
[2] log.txt (1024 bytes)
```

## Wiki pages

### `bl wiki list`

List wiki pages in a project.

```bash
bl wiki list TEST
bl wiki list TEST --keyword setup
bl wiki list TEST --json
```

Example output:

```text
Home [guide, onboarding]
Setup
API Reference
```

### `bl wiki show`

Show the content of a wiki page.

```bash
bl wiki show 12345
bl wiki show 12345 --json
```

Example output:

```text
Home
  Tags:    guide, onboarding
  Created: 2024-01-01T00:00:00Z
  Updated: 2024-06-01T00:00:00Z

# Home
Welcome to the project wiki!
```

### `bl wiki create`

Create a new wiki page.

```bash
bl wiki create --project-id 1 --name "Setup" --content "# Setup\nSee README."
bl wiki create --project-id 1 --name "Setup" --content "# Setup" --mail-notify --json
```

### `bl wiki update`

Update an existing wiki page. At least one of `--name` or `--content` is required.

```bash
bl wiki update 12345 --content "# Updated content"
bl wiki update 12345 --name "New Title" --content "New content" --mail-notify
bl wiki update 12345 --name "Renamed" --json
```

### `bl wiki delete`

Delete a wiki page.

```bash
bl wiki delete 12345
bl wiki delete 12345 --mail-notify --json
```

### `bl wiki history`

Show the revision history of a wiki page.

```bash
bl wiki history 12345
bl wiki history 12345 --json
```

Example output:

```text
v3 Home — 2024-06-01T00:00:00Z
v2 Home — 2024-03-15T00:00:00Z
v1 Home — 2024-01-01T00:00:00Z
```

### `bl wiki attachment list`

List attachments of a wiki page.

```bash
bl wiki attachment list 12345
bl wiki attachment list 12345 --json
```

Example output:

```text
[1] diagram.png (204800 bytes)
[2] notes.txt (1024 bytes)
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
| `bl project activities <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/activities` | ✅ Implemented |
| `bl project disk-usage <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/diskUsage` | ✅ Implemented |
| `bl project user list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/users` | ✅ Implemented |
| `bl project status list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/statuses` | ✅ Implemented |
| `bl project issue-type list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/issueTypes` | ✅ Implemented |
| `bl project category list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/categories` | ✅ Implemented |
| `bl project version list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/versions` | ✅ Implemented |

### Issues

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl issue list` | `GET /api/v2/issues` | ✅ Implemented |
| `bl issue count` | `GET /api/v2/issues/count` | ✅ Implemented |
| `bl issue show <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}` | ✅ Implemented |
| `bl issue create` | `POST /api/v2/issues` | ✅ Implemented |
| `bl issue update <id-or-key>` | `PATCH /api/v2/issues/{issueIdOrKey}` | ✅ Implemented |
| `bl issue delete <id-or-key>` | `DELETE /api/v2/issues/{issueIdOrKey}` | ✅ Implemented |
| `bl issue comment list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/comments` | ✅ Implemented |
| `bl issue comment add <id-or-key>` | `POST /api/v2/issues/{issueIdOrKey}/comments` | ✅ Implemented |
| `bl issue comment update <id-or-key> <comment-id>` | `PATCH /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ Implemented |
| `bl issue comment delete <id-or-key> <comment-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ Implemented |
| `bl issue attachment list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/attachments` | ✅ Implemented |

### Wiki

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl wiki list` | `GET /api/v2/wikis` | ✅ Implemented |
| `bl wiki show <id>` | `GET /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki create` | `POST /api/v2/wikis` | ✅ Implemented |
| `bl wiki update <id>` | `PATCH /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki delete <id>` | `DELETE /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki history <id>` | `GET /api/v2/wikis/{wikiId}/history` | ✅ Implemented |
| `bl wiki attachment list <id>` | `GET /api/v2/wikis/{wikiId}/attachments` | ✅ Implemented |

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
| System keyring | API key and OAuth tokens (primary; GNOME Keyring / Keychain) |
| `~/.config/bl/credentials.toml` | API key fallback (mode 0600, used when keyring is unavailable) |
| `~/.config/bl/oauth_tokens.toml` | OAuth token fallback (mode 0600, used when keyring is unavailable) |

### Windows

| Location | Contents |
| --- | --- |
| `%APPDATA%\bl\config.toml` | Space key (non-sensitive metadata) |
| Windows Credential Manager | API key and OAuth tokens (primary) |
| `%APPDATA%\bl\credentials.toml` | API key fallback (used when Credential Manager is unavailable) |
| `%APPDATA%\bl\oauth_tokens.toml` | OAuth token fallback (used when Credential Manager is unavailable) |

### Config file format

```toml
current_space = "mycompany"
spaces = ["mycompany", "another-company"]
```

Old configs using the `[auth] space_key` format are migrated automatically on first run.

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
to storing the API key in `~/.config/bl/credentials.toml` and OAuth tokens in
`~/.config/bl/oauth_tokens.toml`, both with mode 0600.

On macOS, the system Keychain is used. On Windows, the Windows Credential Manager is used.
If the Credential Manager is unavailable, `bl` falls back to `%APPDATA%\bl\credentials.toml`
(API key) and `%APPDATA%\bl\oauth_tokens.toml` (OAuth tokens).

The `bl auth status` output shows which backend is in use:

```text
  - Stored in: System keyring
```

or

```text
  - Stored in: Credentials file
```
