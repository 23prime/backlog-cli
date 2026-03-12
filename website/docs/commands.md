# Commands

## Root options

These options are only available on the root `bl` command, not on subcommands.

| Option | Description |
| --- | --- |
| `--banner` | Print the Backlog CLI banner and exit |

## Global options

These options are available on all subcommands.

| Option | Description |
| --- | --- |
| `--no-color` | Disable colored output |
| `--space <SPACE_KEY>` | Override the active space for this command |
| `--verbose` / `-v` | Print verbose logs to stderr |

## `bl auth`

| Command | Description |
| --- | --- |
| `bl auth login` | Authenticate with a Backlog API key (adds or updates a space); use `--no-banner` to skip the banner |
| `bl auth login-oauth` | Authenticate via browser-based OAuth 2.0; use `--port <port>` to override the default callback port (54321); use `--no-banner` to skip the banner |
| `bl auth status` | Show current auth status and verify credentials |
| `bl auth list` | List all configured spaces |
| `bl auth use <space-key>` | Switch the current space |
| `bl auth keyring` | Check if the system keyring is available |
| `bl auth logout [<space-key>]` | Remove credentials for the current or specified space |
| `bl auth logout --all` | Remove all spaces and delete all config files |

## `bl space`

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

## `bl space activities`

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

## `bl space disk-usage`

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

## `bl space notification`

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

## `bl project list`

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

## `bl project show`

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

## `bl project activities`

Show recent activities for a specific project.

```bash
bl project activities <id-or-key>
bl project activities <id-or-key> --json
```

Example output:

```text
[123] type=1 project=TEST user=John Doe created=2024-06-01T00:00:00Z
```

## `bl project disk-usage`

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

## `bl project user list`

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

## `bl project status list`

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

## `bl project issue-type list`

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

## `bl project category list`

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

## `bl project version list`

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

## `bl issue list`

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

## `bl issue count`

Count issues with optional filters. Accepts the same filters as `bl issue list`.

```bash
bl issue count
bl issue count --project-id 1 --issue-type-id 1 --parent-child not-child --json
```

Example output:

```text
42
```

## `bl issue show`

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

## `bl issue create`

Create a new issue. Requires `--project-id`, `--summary`, `--issue-type-id`, and `--priority-id`.

```bash
bl issue create --project-id 1 --summary "Fix login" --issue-type-id 1 --priority-id 2
bl issue create --project-id 1 --summary "Bug" --issue-type-id 1 --priority-id 2 \
  --description "Details..." --assignee-id 123 --due-date 2024-12-31 --json
```

Priority IDs: `1` = High, `2` = Normal, `3` = Low.

## `bl issue update`

Update an existing issue. All fields are optional.

```bash
bl issue update TEST-1 --summary "Updated summary"
bl issue update TEST-1 --status-id 2 --comment "Fixed in v1.2" --json
```

## `bl issue delete`

Delete an issue.

```bash
bl issue delete TEST-1
bl issue delete TEST-1 --json
```

Example output:

```text
Deleted: TEST-1
```

## `bl issue comment list`

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

## `bl issue comment add`

Add a comment to an issue.

```bash
bl issue comment add TEST-1 --content "This is a comment"
bl issue comment add TEST-1 --content "Done" --json
```

## `bl issue comment update`

Update an existing comment.

```bash
bl issue comment update TEST-1 42 --content "Updated comment"
bl issue comment update TEST-1 42 --content "Fixed" --json
```

## `bl issue comment delete`

Delete a comment.

```bash
bl issue comment delete TEST-1 42
bl issue comment delete TEST-1 42 --json
```

## `bl issue attachment list`

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

## `bl wiki list`

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

## `bl wiki show`

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

## `bl wiki create`

Create a new wiki page.

```bash
bl wiki create --project-id 1 --name "Setup" --content "# Setup\nSee README."
bl wiki create --project-id 1 --name "Setup" --content "# Setup" --mail-notify --json
```

## `bl wiki update`

Update an existing wiki page. At least one of `--name` or `--content` is required.

```bash
bl wiki update 12345 --content "# Updated content"
bl wiki update 12345 --name "New Title" --content "New content" --mail-notify
bl wiki update 12345 --name "Renamed" --json
```

## `bl wiki delete`

Delete a wiki page.

```bash
bl wiki delete 12345
bl wiki delete 12345 --mail-notify --json
```

## `bl wiki history`

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

## `bl wiki attachment list`

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

## `bl team list`

List all teams in the space.

```bash
bl team list
bl team list --json
```

Example output:

```text
[1] dev-team (3 members)
[2] design-team (2 members)
```

## `bl team show`

Show details of a specific team.

```bash
bl team show <id>
bl team show <id> --json
```

Example output:

```text
ID:      1
Name:    dev-team
Created: 2024-01-01T00:00:00Z
Updated: 2024-06-01T00:00:00Z
Members:
    [2] Developer
    [3] Engineer
```

## `bl user list`

List all users in the space.
Requires Space Administrator privileges. Non-admin users will receive `403 Forbidden`.

```bash
bl user list
bl user list --json
```

Example output:

```text
[john] John Doe
[jane] Jane Smith
[12345] Bot User
```

## `bl user show`

Show details of a specific user by numeric ID.

```bash
bl user show <id>
bl user show <id> --json
```

Example output:

```text
ID:           123
User ID:      john
Name:         John Doe
Mail:         john@example.com
Role:         1
Lang:         ja
Last login:   2024-06-01T00:00:00Z
```

## Command coverage

The table below maps Backlog API v2 endpoints to `bl` commands.

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
| `bl user list` | `GET /api/v2/users` | ✅ Implemented |
| `bl user show <id>` | `GET /api/v2/users/{userId}` | ✅ Implemented |
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
| `bl team list` | `GET /api/v2/teams` | ✅ Implemented |
| `bl team show <id>` | `GET /api/v2/teams/{teamId}` | ✅ Implemented |
