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

## `bl space image`

Download the space icon image.

```bash
bl space image
bl space image --output my_icon.png
```

Example output:

```text
Saved: space_image.png (1234 bytes)
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

## `bl project user add`

Add a user to a project by numeric user ID.

```bash
bl project user add <id-or-key> --user-id <user-id>
bl project user add <id-or-key> --user-id <user-id> --json
```

Example output:

```text
Added: [john] John Doe
```

## `bl project user delete`

Remove a user from a project by numeric user ID.

```bash
bl project user delete <id-or-key> --user-id <user-id>
bl project user delete <id-or-key> --user-id <user-id> --json
```

Example output:

```text
Deleted: [john] John Doe
```

## `bl project admin list`

List administrators of a specific project.

```bash
bl project admin list <id-or-key>
bl project admin list <id-or-key> --json
```

Example output:

```text
[admin1] Alice Admin
```

## `bl project admin add`

Add a user as an administrator of a project by numeric user ID.

```bash
bl project admin add <id-or-key> --user-id <user-id>
bl project admin add <id-or-key> --user-id <user-id> --json
```

Example output:

```text
Added: [john] John Doe
```

## `bl project admin delete`

Remove a user from the administrators of a project by numeric user ID.

```bash
bl project admin delete <id-or-key> --user-id <user-id>
bl project admin delete <id-or-key> --user-id <user-id> --json
```

Example output:

```text
Deleted: [john] John Doe
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

## `bl project status add`

Add a status to a project.

```bash
bl project status add <id-or-key> --name <name> --color <color>
bl project status add <id-or-key> --name <name> --color <color> --json
```

The `--color` value must be a 6-digit hex color code with a `#` prefix (e.g., `#ed8077`). Only hex colors are accepted; CSS color names and shorthand hex are not supported.

Example output:

```text
Added: [5] In Review
```

## `bl project status update`

Update a project status.

```bash
bl project status update <id-or-key> --status-id <id> --name <name>
bl project status update <id-or-key> --status-id <id> --color <color>
bl project status update <id-or-key> --status-id <id> --name <name> --color <color> --json
```

At least one of `--name` or `--color` must be specified.

Example output:

```text
Updated: [5] In Review
```

## `bl project status delete`

Delete a project status. Issues with the deleted status are migrated to the substitute status.

```bash
bl project status delete <id-or-key> --status-id <id> --substitute-status-id <id>
bl project status delete <id-or-key> --status-id <id> --substitute-status-id <id> --json
```

Example output:

```text
Deleted: [5] In Review
```

## `bl project status reorder`

Reorder project statuses by specifying status IDs in the desired display order.

```bash
bl project status reorder <id-or-key> --status-id <id1> --status-id <id2> ...
bl project status reorder <id-or-key> --status-id <id1> --status-id <id2> --json
```

Example output:

```text
[2] In Progress
[1] Open
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

## `bl project issue-type add`

Add an issue type to a project.

```bash
bl project issue-type add <id-or-key> --name <name> --color <color>
bl project issue-type add <id-or-key> --name <name> --color <color> --json
```

`--color` must be a 6-digit hex code with a `#` prefix (e.g. `#e30000`).

Example output:

```text
Added: [1] Bug
```

## `bl project issue-type update`

Update a project issue type.

```bash
bl project issue-type update <id-or-key> --issue-type-id <id> --name <name>
bl project issue-type update <id-or-key> --issue-type-id <id> --color <color>
bl project issue-type update <id-or-key> --issue-type-id <id> --name <name> --color <color> --json
```

At least one of `--name` or `--color` must be specified. `--color` must be a valid hex code.

Example output:

```text
Updated: [1] Bug
```

## `bl project issue-type delete`

Delete a project issue type. Issues using the deleted type are moved to the substitute type.

```bash
bl project issue-type delete <id-or-key> --issue-type-id <id> --substitute-issue-type-id <id>
bl project issue-type delete <id-or-key> --issue-type-id <id> --substitute-issue-type-id <id> --json
```

`--substitute-issue-type-id` must differ from `--issue-type-id`.

Example output:

```text
Deleted: [1] Bug
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

## `bl project category add`

Add a category to a project.

```bash
bl project category add <id-or-key> --name <name>
bl project category add <id-or-key> --name <name> --json
```

Example output:

```text
Added: [11] Development
```

## `bl project category update`

Update a project category.

```bash
bl project category update <id-or-key> --category-id <id> --name <name>
bl project category update <id-or-key> --category-id <id> --name <name> --json
```

Example output:

```text
Updated: [11] Development
```

## `bl project category delete`

Delete a project category.

```bash
bl project category delete <id-or-key> --category-id <id>
bl project category delete <id-or-key> --category-id <id> --json
```

Example output:

```text
Deleted: [11] Development
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

## `bl project version add`

Add a version (milestone) to a project.

```bash
bl project version add <id-or-key> --name "v1.0"
bl project version add <id-or-key> --name "v1.0" --start-date 2024-01-01 --release-due-date 2024-03-31
bl project version add <id-or-key> --name "v1.0" --description "First release" --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--name` | — | Version name (required) |
| `--description` | — | Description |
| `--start-date` | — | Start date (YYYY-MM-DD) |
| `--release-due-date` | — | Release due date (YYYY-MM-DD) |

Example output:

```text
Added: [5] v1.0 (2024-01-01 → 2024-03-31)
```

## `bl project version update`

Update a version in a project.

```bash
bl project version update <id-or-key> --version-id 5 --name "v1.0.1"
bl project version update <id-or-key> --version-id 5 --name "v1.0" --archived true --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--version-id` | — | Version ID (required) |
| `--name` | — | Version name (required) |
| `--description` | — | Description |
| `--start-date` | — | Start date (YYYY-MM-DD) |
| `--release-due-date` | — | Release due date (YYYY-MM-DD) |
| `--archived` | — | `true` to archive, `false` to unarchive |

Example output:

```text
Updated: [5] v1.0.1 (2024-01-01 → 2024-03-31)
```

## `bl project version delete`

Delete a version from a project.

```bash
bl project version delete <id-or-key> --version-id 5
bl project version delete <id-or-key> --version-id 5 --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--version-id` | — | Version ID (required) |

Example output:

```text
Deleted: [5] v1.0
```

## `bl project custom-field list`

List custom fields defined for a specific project.

```bash
bl project custom-field list <id-or-key>
bl project custom-field list <id-or-key> --json
```

Example output:

```text
[1] Priority (type:6)
[2] Severity (type:7) [required]
```

## `bl project custom-field add`

Add a custom field to a project.

```bash
bl project custom-field add <id-or-key> --type-id 6 --name "Priority"
bl project custom-field add <id-or-key> --type-id 1 --name "Notes" --description "Free text" --required true --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--type-id` | — | Field type (required): 1=Text, 2=Sentence, 3=Number, 4=Date, 5=SingleList, 6=MultipleList, 7=Checkbox, 8=Radio |
| `--name` | — | Field name (required) |
| `--description` | — | Description |
| `--required` | — | `true` to mark as required |

Example output:

```text
Added: [1] Priority (type:6)
```

## `bl project custom-field update`

Update a custom field in a project. At least one of `--name`, `--description`, or `--required` must be provided.

```bash
bl project custom-field update <id-or-key> --custom-field-id 1 --name "Importance"
bl project custom-field update <id-or-key> --custom-field-id 1 --required true --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--custom-field-id` | — | Custom field ID (required) |
| `--name` | — | Field name |
| `--description` | — | Description |
| `--required` | — | `true` to mark as required, `false` to unmark |

Example output:

```text
Updated: [1] Importance (type:6)
```

## `bl project custom-field delete`

Delete a custom field from a project.

```bash
bl project custom-field delete <id-or-key> --custom-field-id 1
bl project custom-field delete <id-or-key> --custom-field-id 1 --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--custom-field-id` | — | Custom field ID (required) |

Example output:

```text
Deleted: [1] Priority (type:6)
```

## `bl project custom-field item add`

Add an item to a list-type custom field.

```bash
bl project custom-field item add <id-or-key> --custom-field-id 1 --name "High"
bl project custom-field item add <id-or-key> --custom-field-id 1 --name "High" --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--custom-field-id` | — | Custom field ID (required) |
| `--name` | — | Item name (required) |

## `bl project custom-field item update`

Update an item in a list-type custom field.

```bash
bl project custom-field item update <id-or-key> --custom-field-id 1 --item-id 10 --name "Very High"
bl project custom-field item update <id-or-key> --custom-field-id 1 --item-id 10 --name "Very High" --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--custom-field-id` | — | Custom field ID (required) |
| `--item-id` | — | Item ID (required) |
| `--name` | — | Item name (required) |

## `bl project custom-field item delete`

Delete an item from a list-type custom field.

```bash
bl project custom-field item delete <id-or-key> --custom-field-id 1 --item-id 10
bl project custom-field item delete <id-or-key> --custom-field-id 1 --item-id 10 --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--custom-field-id` | — | Custom field ID (required) |
| `--item-id` | — | Item ID (required) |

## `bl project team list`

List teams assigned to a project.

```bash
bl project team list <id-or-key>
bl project team list <id-or-key> --json
```

## `bl project team add`

Add a team to a project.

```bash
bl project team add <id-or-key> --team-id 1
bl project team add <id-or-key> --team-id 1 --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--team-id` | — | Numeric team ID to add (required) |

## `bl project team delete`

Remove a team from a project.

```bash
bl project team delete <id-or-key> --team-id 1
bl project team delete <id-or-key> --team-id 1 --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--team-id` | — | Numeric team ID to remove (required) |

## `bl project webhook list`

List webhooks defined for a project.

```bash
bl project webhook list <id-or-key>
bl project webhook list <id-or-key> --json
```

## `bl project webhook show`

Show details of a specific webhook.

```bash
bl project webhook show <id-or-key> <webhook-id>
bl project webhook show <id-or-key> <webhook-id> --json
```

## `bl project webhook add`

Add a webhook to a project.

```bash
bl project webhook add <id-or-key> --name "Deploy hook" --hook-url "https://example.com/hook"
bl project webhook add <id-or-key> --name "All events" --hook-url "https://example.com/hook" --all-event true --json
bl project webhook add <id-or-key> --name "Filtered" --hook-url "https://example.com/hook" --activity-type-id 1 --activity-type-id 2
```

| Flag | Default | Description |
| --- | --- | --- |
| `--name` | — | Webhook name (required) |
| `--hook-url` | — | Webhook URL (required) |
| `--description` | — | Description |
| `--all-event` | — | Trigger on all events (`true`/`false`) |
| `--activity-type-id` | — | Activity type ID to trigger on (repeatable) |

## `bl project webhook update`

Update a webhook. At least one of `--name`, `--hook-url`, `--description`, `--all-event`, or `--activity-type-id` must be provided.

```bash
bl project webhook update <id-or-key> <webhook-id> --name "New Name"
bl project webhook update <id-or-key> <webhook-id> --hook-url "https://new.example.com/hook" --json
```

| Flag | Default | Description |
| --- | --- | --- |
| `--name` | — | New webhook name |
| `--hook-url` | — | New webhook URL |
| `--description` | — | New description |
| `--all-event` | — | Trigger on all events (`true`/`false`) |
| `--activity-type-id` | — | Activity type IDs (repeatable; replaces existing list) |

## `bl project webhook delete`

Delete a webhook from a project.

```bash
bl project webhook delete <id-or-key> <webhook-id>
bl project webhook delete <id-or-key> <webhook-id> --json
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

## `bl document list`

List documents in a project.

```bash
bl document list --project-id 1
bl document list --project-id 1 --keyword design
bl document list --project-id 1 --count 50 --offset 0 --json
```

Example output:

```text
[abc123] Design Document
[def456] API Reference
```

## `bl document tree`

Show the document tree for a project.

```bash
bl document tree TEST
bl document tree TEST --json
```

Example output:

```text
Project: 1
Active:
Root [root-id]
  Design Document [abc123]
    Frontend [def456]
    Backend [ghi789]
Trash:
Trash [trash-id]
```

## `bl document show`

Show details of a document.

```bash
bl document show abc123
bl document show abc123 --json
```

Example output:

```text
ID: abc123
Title: Design Document
Project ID: 1
Status: 1
Content:
Hello world
Created: Taro (2024-01-01T00:00:00Z)
Updated: Taro (2024-06-01T00:00:00Z)
```

## `bl document create`

Create a new document.

```bash
bl document create --project-id 1 --title "New Doc" --content "# New Doc"
bl document create --project-id 1 --title "Child Doc" --parent-id abc123 --json
```

## `bl document delete`

Delete a document.

```bash
bl document delete abc123
bl document delete abc123 --json
```

## `bl document attachment get`

Download a document attachment.

```bash
bl document attachment get abc123 1
bl document attachment get abc123 1 --output /path/to/file.pdf
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

## `bl wiki count`

Count wiki pages in a project.

```bash
bl wiki count
bl wiki count TEST
bl wiki count TEST --json
```

Example output:

```text
42
```

## `bl wiki tag list`

List tags used in wiki pages.

```bash
bl wiki tag list
bl wiki tag list TEST
bl wiki tag list TEST --json
```

Example output:

```text
[1] guide
[2] api
```

## `bl wiki star list`

List stars on a wiki page.

```bash
bl wiki star list 12345
bl wiki star list 12345 --json
```

Example output:

```text
[1] Home (John Doe)
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

## `bl wiki attachment add`

Add attachments to a wiki page by pre-uploaded attachment ID.

```bash
bl wiki attachment add 12345 --attachment-id 1
bl wiki attachment add 12345 --attachment-id 1 --attachment-id 2 --json
```

## `bl wiki attachment get`

Download an attachment from a wiki page.

```bash
bl wiki attachment get 12345 1
bl wiki attachment get 12345 1 --output ./downloads/diagram.png
```

## `bl wiki attachment delete`

Delete an attachment from a wiki page.

```bash
bl wiki attachment delete 12345 1
bl wiki attachment delete 12345 1 --json
```

## `bl wiki shared-file list`

List shared files linked to a wiki page.

```bash
bl wiki shared-file list 12345
bl wiki shared-file list 12345 --json
```

Example output:

```text
[1] /docs/spec.pdf (204800 bytes)
```

## `bl wiki shared-file link`

Link shared files to a wiki page.

```bash
bl wiki shared-file link 12345 --shared-file-id 1
bl wiki shared-file link 12345 --shared-file-id 1 --shared-file-id 2 --json
```

## `bl wiki shared-file unlink`

Unlink a shared file from a wiki page.

```bash
bl wiki shared-file unlink 12345 1
bl wiki shared-file unlink 12345 1 --json
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

## `bl team add`

Create a new team.
Depending on the Backlog space configuration, this command may return `403 Forbidden`.

```bash
bl team add --name <name>
bl team add --name <name> --member <user-id> --member <user-id>
bl team add --name <name> --json
```

Example output:

```text
Created: [1] dev-team (0 members)
```

## `bl team update`

Update a team. At least one of `--name` or `--member` must be provided.
Depending on the Backlog space configuration, this command may return `403 Forbidden`.

```bash
bl team update <id> --name <new-name>
bl team update <id> --member <user-id> --member <user-id>
bl team update <id> --name <new-name> --json
```

Example output:

```text
Updated: [1] dev-team (3 members)
```

## `bl team delete`

Delete a team.
Depending on the Backlog space configuration, this command may return `403 Forbidden`.

```bash
bl team delete <id>
bl team delete <id> --json
```

Example output:

```text
Deleted: [1] dev-team (3 members)
```

## `bl team icon`

Download the team icon image.
Depending on the Backlog space configuration, this command may return `403 Forbidden`.

```bash
bl team icon <id>
bl team icon <id> --output <path>
```

Example output:

```text
Saved: icon.png (10240 bytes)
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

## `bl shared-file list`

List shared files in a project directory.

```bash
bl shared-file list <id-or-key>
bl shared-file list <id-or-key> --path docs --count 50 --order asc --json
```

| Option | Default | Description |
| --- | --- | --- |
| `--path` | root | Directory path |
| `--count` | 20 | Number of files to retrieve (1–100) |
| `--order` | — | Sort order (`asc` or `desc`) |
| `--offset` | — | Offset for pagination |

Example output:

```text
[1] /test.txt (1024 bytes)
[2] /docs/readme.txt (512 bytes)
```

## `bl shared-file get`

Download a shared file.

```bash
bl shared-file get <id-or-key> <id>
bl shared-file get <id-or-key> <id> --output /tmp/file.txt
```

| Option | Default | Description |
| --- | --- | --- |
| `--output` / `-o` | original filename | Save path |

Example output:

```text
Saved: file.txt (1024 bytes)
```

## `bl star add`

Add a star to an issue, comment, wiki page, pull request, or pull request comment.
Exactly one target must be specified.

```bash
bl star add --issue-id <id>
bl star add --comment-id <id>
bl star add --wiki-id <id>
bl star add --pull-request-id <id>
bl star add --pull-request-comment-id <id>
```

| Option | Default | Description |
| --- | --- | --- |
| `--issue-id` | — | ID of the issue to star |
| `--comment-id` | — | ID of the issue comment to star |
| `--wiki-id` | — | ID of the wiki page to star |
| `--pull-request-id` | — | ID of the pull request to star |
| `--pull-request-comment-id` | — | ID of the pull request comment to star |

On success, this command prints no output (HTTP 204 No Content).

## `bl star delete`

Remove a star.

```bash
bl star delete <id>
```

On success, this command prints no output (HTTP 204 No Content).

## `bl user star list`

List stars given by a user.

```bash
bl user star list <id>
bl user star list <id> --count 50 --order asc --json
bl user star list <id> --min-id 100 --max-id 200 --json
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

## `bl watch list`

List watchings for a user.

```bash
bl watch list <user-id> [--order asc|desc] [--sort created|updated|issue-updated] [--count N] [--offset N] [--resource-already-read true|false] [--issue-id ID ...] [--json]
```

Example output:

```text
[1] Fix login bug (TEST-42)
[2] Update docs (TEST-7)
```

## `bl watch count`

Count watchings for a user.

```bash
bl watch count <user-id> [--resource-already-read true|false] [--already-read true|false] [--json]
```

Example output:

```text
5
```

## `bl watch show <id>`

Show details of a watching.

```bash
bl watch show <id> [--json]
```

Example output:

```text
[1] Fix login bug (TEST-42)
note: Check this later
```

## `bl watch add`

Add a watching.

```bash
bl watch add --issue <issue-id-or-key> [--note NOTE] [--json]
```

Example output:

```text
Added: [1] Fix login bug (TEST-42)
```

## `bl watch update <id>`

Update the note of a watching.

```bash
bl watch update <id> --note NOTE [--json]
```

Example output:

```text
Updated: [1] Fix login bug (TEST-42)
```

## `bl watch delete <id>`

Delete a watching.

```bash
bl watch delete <id> [--json]
```

Example output:

```text
Deleted: [1] Fix login bug (TEST-42)
```

## `bl watch read <id>`

Mark a watching as read.

```bash
bl watch read <id>
```

On success, this command prints no output (HTTP 204 No Content).

## `bl priority list`

List issue priorities.

```bash
bl priority list [--json]
```

Example output:

```text
[2] High
[3] Normal
[4] Low
```

## `bl resolution list`

List issue resolutions.

```bash
bl resolution list [--json]
```

Example output:

```text
[0] Fixed
[1] Won't Fix
[2] Invalid
[3] Duplication
[4] Cannot Reproduce
```

## `bl pr list`

List pull requests in a repository.

```bash
bl pr list <project-id-or-key> <repo-id-or-name>
bl pr list <project-id-or-key> <repo-id-or-name> --json
```

## `bl pr count`

Count pull requests in a repository.

```bash
bl pr count <project-id-or-key> <repo-id-or-name>
bl pr count <project-id-or-key> <repo-id-or-name> --json
```

## `bl pr show`

Show details of a pull request.

```bash
bl pr show <project-id-or-key> <repo-id-or-name> <number>
bl pr show <project-id-or-key> <repo-id-or-name> <number> --json
```

## `bl pr create`

Create a pull request.

```bash
bl pr create <project-id-or-key> <repo-id-or-name> --summary <summary> --base <base> --branch <branch>
bl pr create <project-id-or-key> <repo-id-or-name> --summary <summary> --base <base> --branch <branch> --json
bl pr create <project-id-or-key> <repo-id-or-name> --summary <summary> --base <base> --branch <branch> --description <desc> --issue-id <id> --assignee-id <id>
```

## `bl pr update`

Update a pull request. At least one of `--summary`, `--description`, `--base`, `--issue-id`, `--assignee-id`, or `--comment` is required.

```bash
bl pr update <project-id-or-key> <repo-id-or-name> <number> --summary <summary>
bl pr update <project-id-or-key> <repo-id-or-name> <number> --summary <summary> --json
bl pr update <project-id-or-key> <repo-id-or-name> <number> --comment <comment>
```

## `bl pr comment list`

List comments on a pull request.

```bash
bl pr comment list <project-id-or-key> <repo-id-or-name> <number>
bl pr comment list <project-id-or-key> <repo-id-or-name> <number> --json
```

## `bl pr comment count`

Count comments on a pull request.

```bash
bl pr comment count <project-id-or-key> <repo-id-or-name> <number>
bl pr comment count <project-id-or-key> <repo-id-or-name> <number> --json
```

## `bl pr comment add`

Add a comment to a pull request.

```bash
bl pr comment add <project-id-or-key> <repo-id-or-name> <number> --content <content>
bl pr comment add <project-id-or-key> <repo-id-or-name> <number> --content <content> --json
```

## `bl pr comment update`

Update a comment on a pull request.

```bash
bl pr comment update <project-id-or-key> <repo-id-or-name> <number> <comment-id> --content <content>
bl pr comment update <project-id-or-key> <repo-id-or-name> <number> <comment-id> --content <content> --json
```

## `bl pr attachment list`

List attachments of a pull request.

```bash
bl pr attachment list <project-id-or-key> <repo-id-or-name> <number>
bl pr attachment list <project-id-or-key> <repo-id-or-name> <number> --json
```

## `bl pr attachment get`

Download an attachment from a pull request.

```bash
bl pr attachment get <project-id-or-key> <repo-id-or-name> <number> <attachment-id>
bl pr attachment get <project-id-or-key> <repo-id-or-name> <number> <attachment-id> --output <path>
```

## `bl pr attachment delete`

Delete an attachment from a pull request.

```bash
bl pr attachment delete <project-id-or-key> <repo-id-or-name> <number> <attachment-id>
bl pr attachment delete <project-id-or-key> <repo-id-or-name> <number> <attachment-id> --json
```

## `bl git repo list`

List Git repositories in a project.

```bash
bl git repo list <project-id-or-key>
bl git repo list <project-id-or-key> --json
```

Example output:

```text
main
develop
```

## `bl git repo show`

Show details of a Git repository.

```bash
bl git repo show <project-id-or-key> <repo-id-or-name>
bl git repo show <project-id-or-key> <repo-id-or-name> --json
```

Example output:

```text
main
  Description: Main repository
  HTTP URL:    https://example.backlog.com/git/TEST/main.git
  SSH URL:     git@example.backlog.com:/TEST/main.git
  Created:     2024-01-01T00:00:00Z
  Updated:     2024-01-02T00:00:00Z
```

## `bl rate-limit`

Show the current API rate limit status.

```bash
bl rate-limit
bl rate-limit --json
```

Example output:

```text
Limit:     600
Remaining: 599
Reset:     1698230400
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
| `bl space image` | `GET /api/v2/space/image` | ✅ Implemented |
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
| `bl project user add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/users` | ✅ Implemented |
| `bl project user delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/users` | ✅ Implemented |
| `bl project admin list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/administrators` | ✅ Implemented |
| `bl project admin add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/administrators` | ✅ Implemented |
| `bl project admin delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/administrators` | ✅ Implemented |
| — | `GET /api/v2/projects/{projectIdOrKey}/image` | Planned |
| `bl project status list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/statuses` | ✅ Implemented |
| `bl project status add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/statuses` | ✅ Implemented |
| `bl project status update <id-or-key> --status-id <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/statuses/{id}` | ✅ Implemented |
| `bl project status delete <id-or-key> --status-id <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/statuses/{id}` | ✅ Implemented |
| `bl project status reorder <id-or-key>` | `PATCH /api/v2/projects/{projectIdOrKey}/statuses/updateDisplayOrder` | ✅ Implemented |
| `bl project issue-type list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/issueTypes` | ✅ Implemented |
| `bl project issue-type add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/issueTypes` | ✅ Implemented |
| `bl project issue-type update <id-or-key> --issue-type-id <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` | ✅ Implemented |
| `bl project issue-type delete <id-or-key> --issue-type-id <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` | ✅ Implemented |
| `bl project category list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/categories` | ✅ Implemented |
| `bl project category add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/categories` | ✅ Implemented |
| `bl project category update <id-or-key> --category-id <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/categories/{id}` | ✅ Implemented |
| `bl project category delete <id-or-key> --category-id <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/categories/{id}` | ✅ Implemented |
| `bl project version list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/versions` | ✅ Implemented |
| `bl project version add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/versions` | ✅ Implemented |
| `bl project version update <id-or-key> --version-id <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/versions/{id}` | ✅ Implemented |
| `bl project version delete <id-or-key> --version-id <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/versions/{id}` | ✅ Implemented |
| `bl project custom-field list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/customFields` | ✅ Implemented |
| `bl project custom-field add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/customFields` | ✅ Implemented |
| `bl project custom-field update <id-or-key> --custom-field-id <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}` | ✅ Implemented |
| `bl project custom-field delete <id-or-key> --custom-field-id <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}` | ✅ Implemented |
| `bl project custom-field item add <id-or-key> --custom-field-id <id>` | `POST /api/v2/projects/{projectIdOrKey}/customFields/{id}/items` | ✅ Implemented |
| `bl project custom-field item update <id-or-key> --custom-field-id <id> --item-id <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` | ✅ Implemented |
| `bl project custom-field item delete <id-or-key> --custom-field-id <id> --item-id <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` | ✅ Implemented |
| `bl project webhook list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/webhooks` | ✅ Implemented |
| `bl project webhook show <id-or-key> <webhook-id>` | `GET /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | ✅ Implemented |
| `bl project webhook add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/webhooks` | ✅ Implemented |
| `bl project webhook update <id-or-key> <webhook-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | ✅ Implemented |
| `bl project webhook delete <id-or-key> <webhook-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | ✅ Implemented |
| `bl project team list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/teams` | ✅ Implemented |
| `bl project team add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/teams` | ✅ Implemented |
| `bl project team delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/teams/{teamId}` | ✅ Implemented |

### Priorities & Resolutions

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl priority list` | `GET /api/v2/priorities` | ✅ Implemented |
| `bl resolution list` | `GET /api/v2/resolutions` | ✅ Implemented |

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
| `bl document list` | `GET /api/v2/documents` | ✅ Implemented |
| `bl document tree` | `GET /api/v2/documents/tree` | ✅ Implemented |
| `bl document show <id>` | `GET /api/v2/documents/{documentId}` | ✅ Implemented |
| `bl document create` | `POST /api/v2/documents` | ✅ Implemented |
| `bl document delete <id>` | `DELETE /api/v2/documents/{documentId}` | ✅ Implemented |
| `bl document attachment get <id> <attachment-id>` | `GET /api/v2/documents/{documentId}/attachments/{attachmentId}` | ✅ Implemented |

### Wiki

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl wiki list` | `GET /api/v2/wikis` | ✅ Implemented |
| `bl wiki count` | `GET /api/v2/wikis/count` | ✅ Implemented |
| `bl wiki tag list` | `GET /api/v2/wikis/tags` | ✅ Implemented |
| `bl wiki show <id>` | `GET /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki create` | `POST /api/v2/wikis` | ✅ Implemented |
| `bl wiki update <id>` | `PATCH /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki delete <id>` | `DELETE /api/v2/wikis/{wikiId}` | ✅ Implemented |
| `bl wiki history <id>` | `GET /api/v2/wikis/{wikiId}/history` | ✅ Implemented |
| `bl wiki star list <id>` | `GET /api/v2/wikis/{wikiId}/stars` | ✅ Implemented |
| `bl wiki attachment list <id>` | `GET /api/v2/wikis/{wikiId}/attachments` | ✅ Implemented |
| `bl wiki attachment add <id>` | `POST /api/v2/wikis/{wikiId}/attachments` | ✅ Implemented |
| `bl wiki attachment get <id> <attachment-id>` | `GET /api/v2/wikis/{wikiId}/attachments/{attachmentId}` | ✅ Implemented |
| `bl wiki attachment delete <id> <attachment-id>` | `DELETE /api/v2/wikis/{wikiId}/attachments/{attachmentId}` | ✅ Implemented |
| `bl wiki shared-file list <id>` | `GET /api/v2/wikis/{wikiId}/sharedFiles` | ✅ Implemented |
| `bl wiki shared-file link <id>` | `POST /api/v2/wikis/{wikiId}/sharedFiles` | ✅ Implemented |
| `bl wiki shared-file unlink <id> <shared-file-id>` | `DELETE /api/v2/wikis/{wikiId}/sharedFiles/{id}` | ✅ Implemented |

### Shared Files

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl shared-file list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/files/metadata/{path}` | ✅ Implemented |
| `bl shared-file get <id-or-key> <id>` | `GET /api/v2/projects/{projectIdOrKey}/files/{sharedFileId}` | ✅ Implemented |

### Stars

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl star add` | `POST /api/v2/stars` | ✅ Implemented |
| `bl star delete <id>` | `DELETE /api/v2/stars/{starId}` | ✅ Implemented |
| `bl user star list <id>` | `GET /api/v2/users/{userId}/stars` | ✅ Implemented |
| `bl user star count <id>` | `GET /api/v2/users/{userId}/stars/count` | ✅ Implemented |

### Pull Requests

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl pr list <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` | ✅ Implemented |
| `bl pr count <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/count` | ✅ Implemented |
| `bl pr show <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` | ✅ Implemented |
| `bl pr create <id-or-key> <repo>` | `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` | ✅ Implemented |
| `bl pr update <id-or-key> <repo> <number>` | `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` | ✅ Implemented |
| `bl pr comment list <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` | ✅ Implemented |
| `bl pr comment count <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/count` | ✅ Implemented |
| `bl pr comment add <id-or-key> <repo> <number>` | `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` | ✅ Implemented |
| `bl pr comment update <id-or-key> <repo> <number> <comment-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/{commentId}` | ✅ Implemented |
| `bl pr attachment list <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments` | ✅ Implemented |
| `bl pr attachment get <id-or-key> <repo> <number> <attachment-id>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments/{attachmentId}` | ✅ Implemented |
| `bl pr attachment delete <id-or-key> <repo> <number> <attachment-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments/{attachmentId}` | ✅ Implemented |

### Git Repositories

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl git repo list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories` | ✅ Implemented |
| `bl git repo show <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}` | ✅ Implemented |

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
| `bl watch list` | `GET /api/v2/users/{userId}/watchings` | ✅ Implemented |
| `bl watch count` | `GET /api/v2/users/{userId}/watchings/count` | ✅ Implemented |
| `bl watch show <id>` | `GET /api/v2/watchings/{watchingId}` | ✅ Implemented |
| `bl watch add` | `POST /api/v2/watchings` | ✅ Implemented |
| `bl watch update <id>` | `PATCH /api/v2/watchings/{watchingId}` | ✅ Implemented |
| `bl watch delete <id>` | `DELETE /api/v2/watchings/{watchingId}` | ✅ Implemented |
| `bl watch read <id>` | `POST /api/v2/watchings/{watchingId}/markAsRead` | ✅ Implemented |

### Teams

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl team list` | `GET /api/v2/teams` | ✅ Implemented |
| `bl team show <id>` | `GET /api/v2/teams/{teamId}` | ✅ Implemented |
| `bl team add` | `POST /api/v2/teams` | ✅ Implemented |
| `bl team update <id>` | `PATCH /api/v2/teams/{teamId}` | ✅ Implemented |
| `bl team delete <id>` | `DELETE /api/v2/teams/{teamId}` | ✅ Implemented |
| `bl team icon <id>` | `GET /api/v2/teams/{teamId}/icon` | ✅ Implemented |

### System

| Command | API endpoint | Status |
| --- | --- | --- |
| `bl rate-limit` | `GET /api/v2/rateLimit` | ✅ Implemented |
