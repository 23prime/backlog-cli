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

```rust
pub fn show(json: bool) -> Result<()> {
    let client = BacklogClient::from_config()?;
    show_with(json, &client)
}

pub fn show_with(json: bool, api: &dyn BacklogApi) -> Result<()> {
    let params: Vec<(String, String)> = vec![];
    let data = api.get_my_resource(&params)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&data).context("Failed to serialize JSON")?);
    } else {
        println!("{}", format_text(&data));
    }
    Ok(())
}
```

### `src/main.rs`

For a simple new top-level command:

```rust
Commands::MyCmd { json } => cmd::my_cmd::show(json),
```

For a new subcommand under an existing group (e.g. `bl space <sub>`):

```rust
Some(SpaceCommands::NewSub { json: sub_json }) => cmd::space::new_sub(sub_json),
```

Rename inner binding to avoid shadowing: `json: sub_json`.

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
- General rule: when a deserialization error occurs with "expected a string, got null", check the raw JSON printed in the error and make only that specific field `Option<String>`

## Nullable field strategy

Don't preemptively make all fields `Option`. Instead:

1. Run `mise run rs-run <cmd>` against the real API
2. Read the raw JSON in the error output
3. Make only the confirmed-null field `Option<T>`
4. Add a test case with the null value

## Real API test safety

`mise run rs-run` calls the real Backlog API. Before running:

- **GET commands**: safe to run directly
- **POST / PATCH / DELETE commands**: always confirm with the user before executing — these create, modify, or delete real data

## Commit rules

- Format: Conventional Commits (`feat:`, `fix:`, `docs:`, etc.)
- Language: English
- **No `Co-Authored-By:` line**
