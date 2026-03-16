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
bl space activities --count 50 --order asc
bl space activities --activity-type-id 1 --activity-type-id 2 --min-id 100 --json
bl space activities --max-id 500 --json
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

## `bl space licence`

Show the licence information for your Backlog space.

```bash
bl space licence
bl space licence --json
```

Example output:

```text
Contract:  premium
Storage:   5242880 / 1073741824 bytes
Start:     2020-01-01
```

## `bl space update-notification`

Update the notification message set for your Backlog space.

```bash
bl space update-notification --content "Scheduled maintenance on 2024-07-01."
bl space update-notification --content "Hello." --json
```

Example output:

```text
Updated: 2024-07-01T00:00:00Z

Scheduled maintenance on 2024-07-01.
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
bl project activities <id-or-key> --count 50 --order asc
bl project activities <id-or-key> --activity-type-id 1 --min-id 100 --json
bl project activities <id-or-key> --max-id 200 --json
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

## `bl project create`

Create a new project.

```bash
bl project create --name "My Project" --key MYPRJ
bl project create --name "My Project" --key MYPRJ --chart-enabled --subtasking-enabled
bl project create --name "My Project" --key MYPRJ --text-formatting-rule backlog --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--name` | — | Project name (required) |
| `--key` | — | Project key: uppercase letters, numbers, underscore; 2–10 chars (required) |
| `--chart-enabled` | `false` | Enable burndown chart feature |
| `--subtasking-enabled` | `false` | Enable subtasking |
| `--text-formatting-rule` | `markdown` | `backlog` or `markdown` |
| `--json` | — | Output as JSON |

Example output:

```text
ID:         1
Key:        MYPRJ
Name:       My Project
Formatting: markdown
Archived:   false
```

## `bl project update`

Update an existing project. At least one field must be specified.

```bash
bl project update <id-or-key> --name "New Name"
bl project update <id-or-key> --key NEWKEY --chart-enabled true
bl project update <id-or-key> --archived true --json
```

| Flag | Description |
| --- | --- |
| `--name` | New project name |
| `--key` | New project key |
| `--chart-enabled` | `true` or `false` |
| `--subtasking-enabled` | `true` or `false` |
| `--text-formatting-rule` | `backlog` or `markdown` |
| `--archived` | `true` or `false` |
| `--json` | Output as JSON |

Example output:

```text
ID:         1
Key:        MYPRJ
Name:       New Name
Formatting: markdown
Archived:   false
```

## `bl project delete`

Delete a project permanently.

```bash
bl project delete <id-or-key>
bl project delete <id-or-key> --json
```

Example output:

```text
Deleted: My Project (MYPRJ)
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

## `bl issue comment count`

Count comments on an issue.

```bash
bl issue comment count TEST-1
bl issue comment count TEST-1 --json
```

## `bl issue comment show`

Show a specific comment.

```bash
bl issue comment show TEST-1 42
bl issue comment show TEST-1 42 --json
```

## `bl issue comment notification list`

List notifications for a comment.

```bash
bl issue comment notification list TEST-1 42
bl issue comment notification list TEST-1 42 --json
```

## `bl issue comment notification add`

Add notifications for a comment.

```bash
bl issue comment notification add TEST-1 42 --notified-user-id 1
bl issue comment notification add TEST-1 42 --notified-user-id 1 --notified-user-id 2 --json
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

## `bl issue attachment get`

Download an attachment from an issue.

```bash
bl issue attachment get TEST-1 1
bl issue attachment get TEST-1 1 --output ./downloaded.png
```

Saves the file to the specified path (`--output`) or to the original filename in the current directory.

Example output:

```text
Saved: screenshot.png (204800 bytes)
```

## `bl issue attachment delete`

Delete an attachment from an issue.

```bash
bl issue attachment delete TEST-1 1
bl issue attachment delete TEST-1 1 --json
```

## `bl issue participant list`

List participants of an issue.

```bash
bl issue participant list TEST-1
bl issue participant list TEST-1 --json
```

## `bl issue shared-file list`

List shared files linked to an issue.

```bash
bl issue shared-file list TEST-1
bl issue shared-file list TEST-1 --json
```

## `bl issue shared-file link`

Link shared files to an issue.

```bash
bl issue shared-file link TEST-1 --shared-file-id 1
bl issue shared-file link TEST-1 --shared-file-id 1 --shared-file-id 2 --json
```

## `bl issue shared-file unlink`

Unlink a shared file from an issue.

```bash
bl issue shared-file unlink TEST-1 1
bl issue shared-file unlink TEST-1 1 --json
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
bl team list --count 50 --offset 10
bl team list --json
```

Example output:

```text
[1] dev-team (3 members)
[2] design-team (2 members)
```

## `bl team show`

Show details of a specific team.
Depending on the Backlog space configuration, this command may return `403 Forbidden`.

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

## `bl user activities`

Show recent activities of a specific user.

```bash
bl user activities <id>
bl user activities <id> --count 50 --order asc
bl user activities <id> --activity-type-id 1 --min-id 100 --json
bl user activities <id> --max-id 200 --json
```

Example output:

```text
[3153] type=2 project=SUB user=admin created=2024-06-01T00:00:00Z
```

## `bl user recently-viewed`

Show issues recently viewed by the authenticated user.

```bash
bl user recently-viewed
bl user recently-viewed --count 50 --offset 20 --order asc
bl user recently-viewed --json
```

Example output:

```text
[BLG-1] Fix login (Open, -)
[BLG-2] Add dark mode (In Progress, John Doe)
```

## `bl user add`

Add a new user. Requires Space Administrator privileges.

```bash
bl user add --user-id john --password secret --name "John Doe" --mail-address john@example.com --role-type normal
bl user add --user-id john --password secret --name "John Doe" --mail-address john@example.com --role-type normal --json
```

Role types: `administrator`, `normal`, `reporter`, `viewer`, `guest-reporter`, `guest-viewer`.

Example output:

```text
Added: john (John Doe) [roleType: 2]
```

## `bl user update`

Update an existing user. Requires Space Administrator privileges.

```bash
bl user update <id> --name "New Name"
bl user update <id> --mail-address new@example.com --role-type viewer --json
```

Example output:

```text
Updated: john (New Name) [roleType: 4]
```

## `bl user delete`

Delete a user. Requires Space Administrator privileges.

```bash
bl user delete <id>
bl user delete <id> --json
```

Example output:

```text
Deleted: john (John Doe)
```

## `bl user recently-viewed-projects`

Show projects recently viewed by the authenticated user.

```bash
bl user recently-viewed-projects
bl user recently-viewed-projects --count 50 --offset 20 --order asc
bl user recently-viewed-projects --json
```

Example output:

```text
[MYPRJ] My Project
[TEST] Test Project
```

## `bl user recently-viewed-wikis`

Show wiki pages recently viewed by the authenticated user.

```bash
bl user recently-viewed-wikis
bl user recently-viewed-wikis --count 50 --offset 20 --order asc
bl user recently-viewed-wikis --json
```

Example output:

```text
[1] Home (project: 1)
[2] API Reference (project: 2)
```

## `bl user star list`

List stars given by a user.

```bash
bl user star list <id>
bl user star list <id> --count 50 --order asc --json
```

Example output:

```text
[1] Fix login bug
[2] Add dark mode feature
```

## `bl user star count`

Count stars given by a user.

```bash
bl user star count <id>
bl user star count <id> --since 2024-01-01 --until 2024-12-31 --json
```

Example output:

```text
42
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

## `bl notification list`

List notifications for the authenticated user.

```bash
bl notification list
bl notification list --count 50 --order asc
bl notification list --min-id 100 --sender-id 123 --json
bl notification list --unread --json
```

Example output:

```text
[101] reason=2 project=TEST issue=TEST-1 read=false created=2024-06-01T00:00:00Z
[102] reason=6 project=TEST issue=TEST-2 read=true  created=2024-06-02T00:00:00Z
```

## `bl notification count`

Count unread notifications for the authenticated user.

```bash
bl notification count
bl notification count --json
```

Example output:

```text
3
```

## `bl notification read <id>`

Mark a specific notification as read.

```bash
bl notification read <id>
```

## `bl notification reset-unread`

Reset the unread notification count.
Note: this resets the counter, but does not mark individual notifications as read.

```bash
bl notification reset-unread
```

Example output:

```text
Unread count reset.
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
| `bl space licence` | `GET /api/v2/space/licence` | ✅ Implemented |
| `bl space update-notification` | `PUT /api/v2/space/notification` | ✅ Implemented |
| — | `GET /api/v2/space/image` | Planned |
| — | `POST /api/v2/space/attachment` | Planned |

### Projects

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl project list` | `GET /api/v2/projects` | ✅ Implemented |
| `bl project show <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}` | ✅ Implemented |
| `bl project create` | `POST /api/v2/projects` | ✅ Implemented |
| `bl project update <id-or-key>` | `PATCH /api/v2/projects/{projectIdOrKey}` | ✅ Implemented |
| `bl project delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}` | ✅ Implemented |
| `bl project activities <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/activities` | ✅ Implemented |
| `bl project disk-usage <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/diskUsage` | ✅ Implemented |
| `bl project user list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/users` | ✅ Implemented |
| `bl project user add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/users` | Planned |
| `bl project user delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/users` | Planned |
| `bl project admin list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/administrators` | Planned |
| `bl project admin add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/administrators` | Planned |
| `bl project admin delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/administrators` | Planned |
| — | `GET /api/v2/projects/{projectIdOrKey}/image` | Planned |
| `bl project status list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/statuses` | ✅ Implemented |
| `bl project status add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/statuses` | Planned |
| `bl project status update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/statuses/{id}` | Planned |
| `bl project status delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/statuses/{id}` | Planned |
| `bl project status reorder <id-or-key>` | `PATCH /api/v2/projects/{projectIdOrKey}/statuses/updateDisplayOrder` | Planned |
| `bl project issue-type list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/issueTypes` | ✅ Implemented |
| `bl project issue-type add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/issueTypes` | Planned |
| `bl project issue-type update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` | Planned |
| `bl project issue-type delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` | Planned |
| `bl project category list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/categories` | ✅ Implemented |
| `bl project category add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/categories` | Planned |
| `bl project category update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/categories/{id}` | Planned |
| `bl project category delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/categories/{id}` | Planned |
| `bl project version list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/versions` | ✅ Implemented |
| `bl project version add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/versions` | Planned |
| `bl project version update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/versions/{id}` | Planned |
| `bl project version delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/versions/{id}` | Planned |
| `bl project custom-field list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/customFields` | Planned |
| `bl project custom-field add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/customFields` | Planned |
| `bl project custom-field update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}` | Planned |
| `bl project custom-field delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}` | Planned |
| `bl project custom-field item add <id-or-key> <id>` | `POST /api/v2/projects/{projectIdOrKey}/customFields/{id}/items` | Planned |
| `bl project custom-field item update <id-or-key> <id> <item-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` | Planned |
| `bl project custom-field item delete <id-or-key> <id> <item-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` | Planned |
| `bl project webhook list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/webhooks` | Planned |
| `bl project webhook show <id-or-key> <webhook-id>` | `GET /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | Planned |
| `bl project webhook add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/webhooks` | Planned |
| `bl project webhook update <id-or-key> <webhook-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | Planned |
| `bl project webhook delete <id-or-key> <webhook-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | Planned |
| `bl project team list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/teams` | Planned |
| `bl project team add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/teams` | Planned |
| `bl project team delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/teams` | Planned |

### Priorities & Resolutions

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl priority list` | `GET /api/v2/priorities` | Planned |
| `bl resolution list` | `GET /api/v2/resolutions` | Planned |

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
| `bl issue comment count <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/comments/count` | ✅ Implemented |
| `bl issue comment show <id-or-key> <comment-id>` | `GET /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ Implemented |
| `bl issue comment update <id-or-key> <comment-id>` | `PATCH /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ Implemented |
| `bl issue comment delete <id-or-key> <comment-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ Implemented |
| `bl issue comment notification list <id-or-key> <comment-id>` | `GET /api/v2/issues/{issueIdOrKey}/comments/{commentId}/notifications` | ✅ Implemented |
| `bl issue comment notification add <id-or-key> <comment-id>` | `POST /api/v2/issues/{issueIdOrKey}/comments/{commentId}/notifications` | ✅ Implemented |
| `bl issue attachment list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/attachments` | ✅ Implemented |
| `bl issue attachment get <id-or-key> <attachment-id>` | `GET /api/v2/issues/{issueIdOrKey}/attachments/{attachmentId}` | ✅ Implemented |
| `bl issue attachment delete <id-or-key> <attachment-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/attachments/{attachmentId}` | ✅ Implemented |
| `bl issue participant list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/participants` | ✅ Implemented |
| `bl issue shared-file list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/sharedFiles` | ✅ Implemented |
| `bl issue shared-file link <id-or-key>` | `POST /api/v2/issues/{issueIdOrKey}/sharedFiles` | ✅ Implemented |
| `bl issue shared-file unlink <id-or-key> <shared-file-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/sharedFiles/{id}` | ✅ Implemented |

### Documents

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl document list` | `GET /api/v2/documents` | Planned |
| `bl document tree` | `GET /api/v2/documents/tree` | Planned |
| `bl document show <id>` | `GET /api/v2/documents/{documentId}` | Planned |
| `bl document create` | `POST /api/v2/documents` | Planned |
| `bl document delete <id>` | `DELETE /api/v2/documents/{documentId}` | Planned |
| `bl document attachment get <id> <attachment-id>` | `GET /api/v2/documents/{documentId}/attachments/{attachmentId}` | Planned |

### Wiki

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl wiki list` | `GET /api/v2/wikis` | ✅ Implemented |
| `bl wiki count` | `GET /api/v2/wikis/count` | Planned |
| `bl wiki tag list` | `GET /api/v2/wikis/tags` | Planned |
| `bl wiki show <id>` | `GET /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki create` | `POST /api/v2/wikis` | ✅ Implemented |
| `bl wiki update <id>` | `PATCH /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki delete <id>` | `DELETE /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki history <id>` | `GET /api/v2/wikis/{wikiId}/history` | ✅ Implemented |
| `bl wiki star list <id>` | `GET /api/v2/wikis/{wikiId}/stars` | Planned |
| `bl wiki attachment list <id>` | `GET /api/v2/wikis/{wikiId}/attachments` | ✅ Implemented |
| `bl wiki attachment add <id>` | `POST /api/v2/wikis/{wikiId}/attachments` | Planned |
| `bl wiki attachment get <id> <attachment-id>` | `GET /api/v2/wikis/{wikiId}/attachments/{attachmentId}` | Planned |
| `bl wiki attachment delete <id> <attachment-id>` | `DELETE /api/v2/wikis/{wikiId}/attachments/{attachmentId}` | Planned |
| `bl wiki shared-file list <id>` | `GET /api/v2/wikis/{wikiId}/sharedFiles` | Planned |
| `bl wiki shared-file link <id>` | `POST /api/v2/wikis/{wikiId}/sharedFiles` | Planned |
| `bl wiki shared-file unlink <id> <shared-file-id>` | `DELETE /api/v2/wikis/{wikiId}/sharedFiles/{id}` | Planned |

### Shared Files

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl shared-file list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/files/metadata/{path}` | Planned |
| `bl shared-file get <id-or-key> <id>` | `GET /api/v2/projects/{projectIdOrKey}/files/{sharedFileId}` | Planned |

### Stars

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl star add` | `POST /api/v2/stars` | Planned |
| `bl star delete <id>` | `DELETE /api/v2/stars/{starId}` | Planned |
| `bl user star list <id>` | `GET /api/v2/users/{userId}/stars` | ✅ Implemented |
| `bl user star count <id>` | `GET /api/v2/users/{userId}/stars/count` | ✅ Implemented |

### Pull Requests

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl pr list <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` | Planned |
| `bl pr count <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/count` | Planned |
| `bl pr show <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` | Planned |
| `bl pr create <id-or-key> <repo>` | `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` | Planned |
| `bl pr update <id-or-key> <repo> <number>` | `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` | Planned |
| `bl pr comment list <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` | Planned |
| `bl pr comment count <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/count` | Planned |
| `bl pr comment add <id-or-key> <repo> <number>` | `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` | Planned |
| `bl pr comment update <id-or-key> <repo> <number> <comment-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/{commentId}` | Planned |
| `bl pr attachment list <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments` | Planned |
| `bl pr attachment get <id-or-key> <repo> <number> <attachment-id>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments/{attachmentId}` | Planned |
| `bl pr attachment delete <id-or-key> <repo> <number> <attachment-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments/{attachmentId}` | Planned |

### Git Repositories

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl git repo list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories` | Planned |
| `bl git repo show <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}` | Planned |

### Users

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl auth status` | `GET /api/v2/users/myself` | ✅ Implemented (internal) |
| `bl user list` | `GET /api/v2/users` | ✅ Implemented |
| `bl user show <id>` | `GET /api/v2/users/{userId}` | ✅ Implemented |
| `bl user add` | `POST /api/v2/users` | ✅ Implemented |
| `bl user update <id>` | `PATCH /api/v2/users/{userId}` | ✅ Implemented |
| `bl user delete <id>` | `DELETE /api/v2/users/{userId}` | ✅ Implemented |
| `bl user activities <id>` | `GET /api/v2/users/{userId}/activities` | ✅ Implemented |
| `bl user recently-viewed` | `GET /api/v2/users/myself/recentlyViewedIssues` | ✅ Implemented |
| `bl user recently-viewed-projects` | `GET /api/v2/users/myself/recentlyViewedProjects` | ✅ Implemented |
| `bl user recently-viewed-wikis` | `GET /api/v2/users/myself/recentlyViewedWikis` | ✅ Implemented |
| `bl user star list <id>` | `GET /api/v2/users/{userId}/stars` | ✅ Implemented |
| `bl user star count <id>` | `GET /api/v2/users/{userId}/stars/count` | ✅ Implemented |
| — | `GET /api/v2/users/{userId}/icon` | Planned |

### Notifications

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl notification list` | `GET /api/v2/notifications` | ✅ Implemented |
| `bl notification count` | `GET /api/v2/notifications/count` | ✅ Implemented |
| `bl notification read <id>` | `POST /api/v2/notifications/{notificationId}/markAsRead` | ✅ Implemented |
| `bl notification reset-unread` | `POST /api/v2/notifications/markAsRead` | ✅ Implemented |

### Watching

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl watch list` | `GET /api/v2/users/{userId}/watchings` | Planned |
| `bl watch count` | `GET /api/v2/users/{userId}/watchings/count` | Planned |
| `bl watch show <id>` | `GET /api/v2/watchings/{watchingId}` | Planned |
| `bl watch add` | `POST /api/v2/watchings` | Planned |
| `bl watch update <id>` | `PATCH /api/v2/watchings/{watchingId}` | Planned |
| `bl watch delete <id>` | `DELETE /api/v2/watchings/{watchingId}` | Planned |
| `bl watch read <id>` | `POST /api/v2/watchings/{watchingId}/markAsRead` | Planned |

### Teams

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl team list` | `GET /api/v2/teams` | ✅ Implemented |
| `bl team show <id>` | `GET /api/v2/teams/{teamId}` | ✅ Implemented |
| `bl team add` | `POST /api/v2/teams` | Planned |
| `bl team update <id>` | `PATCH /api/v2/teams/{teamId}` | Planned |
| `bl team delete <id>` | `DELETE /api/v2/teams/{teamId}` | Planned |
| — | `GET /api/v2/teams/{teamId}/icon` | Planned |

### System

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl rate-limit` | `GET /api/v2/rateLimit` | Planned |
