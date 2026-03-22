# Implementation Patterns and Gotchas

## File-by-file patterns

### `src/api/<resource>.rs`

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};
use super::BacklogClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyResource {
    pub id: u64,
    pub name: String,
    pub nullable_field: Option<String>,  // use Option for fields that can be null
}

impl BacklogClient {
    pub fn get_my_resource(&self, params: &[(String, String)]) -> Result<MyResource> {
        let value = self.get_with_query("/my-resource", params)?;
        serde_json::from_value(value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize response: {}\nRaw JSON:\n{}",
                e,
                serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
            )
        })
    }
}
```

Always include raw JSON in deserialization errors to diagnose null-field issues.

### `src/api/mod.rs`

Add in three places:

```rust
pub mod my_resource;          // 1. module declaration
use my_resource::MyResource;  // 2. import

pub trait BacklogApi {
    // 3a. trait method — default body suppresses compile errors in MockApis
    fn get_my_resource(&self, _params: &[(String, String)]) -> Result<MyResource> { unimplemented!() }
}

impl BacklogApi for BacklogClient {
    fn get_my_resource(&self, params: &[(String, String)]) -> Result<MyResource> {  // 3b. impl
        self.get_my_resource(params)
    }
}
```

### `src/cmd/<command>.rs`

Commands use an args struct. The public entry point constructs the client; `*_with` contains the
real logic and is directly testable.

```rust
pub struct MyArgs {
    key: String,
    json: bool,
}

impl MyArgs {
    pub fn new(key: String, json: bool) -> Self {
        Self { key, json }
    }
}

pub fn show(args: &MyArgs) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(args, &client)
}

pub fn show_with(args: &MyArgs, api: &dyn BacklogApi) -> Result<()> {
    let params: Vec<(String, String)> = vec![];
    let data = api.get_my_resource(&params)?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&data).context("Failed to serialize JSON")?);
    } else {
        println!("{}", format_text(&data));
    }
    Ok(())
}
```

Use `try_new` instead of `new` when construction can fail (see [`docs/VALIDATION.md`](VALIDATION.md)).

### `src/main.rs`

Construct the args struct and pass a reference to the command function:

```rust
Commands::MyCmd { key, json } => cmd::my_cmd::show(&MyArgs::new(key, json)),
```

For a new subcommand under an existing group (e.g. `bl space <sub>`):

```rust
Some(SpaceCommands::NewSub { json: sub_json }) => {
    cmd::space::new_sub(&NewSubArgs::new(sub_json))
}
```

Rename inner binding to avoid shadowing: `json: sub_json`.

#### clap bool flags

`bool` fields in clap derive become **presence flags** (no value needed).
`default_value = "false"` is redundant — do not add it:

```rust
// ✅ correct
#[arg(long)]
chart_enabled: bool,

// ❌ redundant — clap already defaults bool to false
#[arg(long, default_value = "false")]
chart_enabled: bool,
```

#### Constrained string options — use `clap::ValueEnum`

When a flag accepts only a fixed set of values, define a `ValueEnum` enum
instead of `String`. Follow the `Order` enum pattern already in `main.rs`:

```rust
#[derive(clap::ValueEnum, Clone)]
enum TextFormattingRule {
    Backlog,
    Markdown,
}

impl TextFormattingRule {
    fn as_str(&self) -> &'static str {
        match self {
            TextFormattingRule::Backlog => "backlog",
            TextFormattingRule::Markdown => "markdown",
        }
    }
}
```

Use with a default:

```rust
#[arg(long, default_value = "markdown")]
text_formatting_rule: TextFormattingRule,
```

For optional fields (e.g. update commands):

```rust
#[arg(long)]
text_formatting_rule: Option<TextFormattingRule>,
```

Convert to `String` in the match arm before passing to the args struct:

```rust
text_formatting_rule.as_str().to_string()          // required field
text_formatting_rule.map(|r| r.as_str().to_string()) // optional field
```

## Test MockApi pattern

`BacklogApi` has default `unimplemented!()` bodies for all methods, so a
`MockApi` only needs to override the methods actually exercised by the test.
Do **not** add `unimplemented!()` stubs for unused methods — the default fires
automatically.

```rust
struct MockApi { data: Option<MyResource> }

impl BacklogApi for MockApi {
    fn get_my_resource(&self, _params: &[(String, String)]) -> Result<MyResource> {
        self.data.clone().ok_or_else(|| anyhow!("no data"))
    }
}
```

When a test calls a method that isn't overridden, it will panic with
`not implemented` — which is the desired behavior (it means the test is
exercising an unexpected code path).

## Binary file download

For endpoints that return a binary file (e.g. `GET /issues/{key}/attachments/{id}`), use
`BacklogClient::download` rather than `BacklogClient::get`. It returns `(Vec<u8>, String)` where
the second element is the filename extracted from the `Content-Disposition` response header.

The filename is parsed according to RFC 5987: `filename*=` (percent-encoded, supports non-ASCII)
takes precedence over `filename=`. Path traversal is prevented by stripping directory components
with `Path::file_name()`, which also returns `None` for `.` and `..`, falling back to `"attachment"`.

## Known Backlog API gotchas

- **Bot/system users**: `userId` is `null` → declare as `Option<String>`
- **Some user fields**: `lang`, `mailAddress` can be `null` in certain contexts
- **`project` in activities**: can be `null` for space-level events → `Option<ActivityProject>`
- General rule: when a deserialization error occurs with "expected a string, got null",
  check the raw JSON printed in the error and make only that specific field `Option<String>`

## Nullable field strategy

Don't preemptively make all fields `Option`. Instead:

1. Run `mise run rs-run <cmd>` against the real API
2. Read the raw JSON in the error output
3. Make only the confirmed-null field `Option<T>`
4. Add a test case with the null value

## `show` vs `list` text output convention

`list` commands display a compact single-line row per item (e.g. `[id] name (url)`).
`show` commands should display a **detailed** multi-line view that includes all
meaningful fields (description, configuration, etc.).

Define two separate formatters:

```rust
fn format_webhook_row(h: &ProjectWebhook) -> String {
    format!("[{}] {} ({})", h.id, h.name, h.hook_url)  // used by list, add, update, delete
}

fn format_webhook_detail(h: &ProjectWebhook) -> String {
    // used by show — include description, event config, etc.
    format!("ID: {}\nName: {}\nURL: {}\nDescription: {}\nEvents: {}",
        h.id, h.name, h.hook_url, h.description, ...)
}
```

This pattern prevents `show` from looking identical to `list` in non-JSON mode,
which is confusing to users.

## Shared row formatters in `mod.rs`

When multiple subcommands under the same resource group (e.g. `list`, `add`, `delete`,
`update`) all print the same single-line row format, define the formatter once as
`pub(crate)` in `mod.rs` and import it via `use super::format_*_row` in each subcommand:

```rust
// src/cmd/team/mod.rs
pub(crate) fn format_team_row(t: &crate::api::team::Team) -> String {
    format!("[{}] {} ({} members)", t.id, t.name, t.members.len())
}
```

```rust
// src/cmd/team/add.rs  (and delete.rs, update.rs)
use super::format_team_row;
// …
println!("Created: {}", format_team_row(&team));
```

Without this, each file grows its own private copy that drifts — e.g. `delete`
might omit the member count that `list` includes.

## Project-scoped API methods — module placement

When a project-scoped endpoint (e.g. `/projects/{key}/teams`) returns a type
defined in a dedicated module (e.g. `Team` in `team.rs`), implement the method
in that module — **not** in `project.rs`. Moving it to `project.rs` would
introduce a cross-module import of `Team` from `team.rs` with no benefit.

Keep all methods returning `Team` in `src/api/team.rs`, even if the path starts
with `/projects/`.

## Command coverage table conventions

In `website/docs/commands.md` and the JA equivalent, the command column of the
coverage table should **not** include required flags. List only the subcommand
and positional arguments:

```markdown
| `bl project team add <id-or-key>` | ... |       ✅ correct
| `bl project team add <id-or-key> --team-id <id>` | ... |  ❌ too verbose
```

## Real API test safety

`mise run rs-run` calls the real Backlog API. Before running:

- **GET commands**: safe to run directly
- **POST / PATCH / DELETE commands**: always confirm with the user before executing — these create,
  modify, or delete real data

## Shared file / directory entries

The `/api/v2/projects/{key}/files/metadata/{path}` endpoint returns both files and
directories in the same array. Directory entries have `"size": null`, so model
`size` as `Option<u64>` — not `u64`. Rendering: omit the byte count for `None`.

When embedding a user-supplied path in a URL, percent-encode each segment while
preserving `/` separators, and strip any leading `/` so that `--path /docs` and
`--path docs` behave identically. Implement inline without an external crate:

```rust
fn encode_path(path: &str) -> String {
    let path = path.trim_start_matches('/');
    path.split('/')
        .map(|seg| {
            seg.bytes()
                .flat_map(|b| {
                    if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
                        vec![b as char]
                    } else {
                        format!("%{b:02X}").chars().collect()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/")
}
```

When printing `dir + name`, always normalize the separator — `dir` may or may not
end with `/`:

```rust
let sep = if f.dir.ends_with('/') { "" } else { "/" };
println!("[{}] {}{}{}", f.id, f.dir, sep, f.name);
```

## Document API specifics

- Document IDs are `String` (UUID-like), not `u64` like most other resources.
- POST `/api/v2/documents` returns a simplified response with `createdUserId`/`updatedUserId`
  (flat integer fields) instead of embedded `createdUser`/`updatedUser` objects.
  Model these as `Option<DocumentUser>` on the struct so both POST and GET responses deserialize.
- The document list endpoint requires `count` and `offset` params (400 error if omitted).
  Always send them — default to `count=20, offset=0`.
