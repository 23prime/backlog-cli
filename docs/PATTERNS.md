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

## Real API test safety

`mise run rs-run` calls the real Backlog API. Before running:

- **GET commands**: safe to run directly
- **POST / PATCH / DELETE commands**: always confirm with the user before executing — these create,
  modify, or delete real data
