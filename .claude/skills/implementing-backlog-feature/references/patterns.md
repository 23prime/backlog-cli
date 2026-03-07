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
    pub fn get_my_resource(&self) -> Result<MyResource> {
        let value = self.get("/my-resource")?;
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
    fn get_my_resource(&self) -> Result<MyResource>;  // 3a. trait method
}

impl BacklogApi for BacklogClient {
    fn get_my_resource(&self) -> Result<MyResource> {  // 3b. impl
        self.get_my_resource()
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
    let data = api.get_my_resource()?;
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

Every test module that implements `BacklogApi` on a `MockApi` **must include ALL trait methods**.
Use `unimplemented!()` for methods not under test:

```rust
struct MockApi { data: Option<MyResource> }

impl BacklogApi for MockApi {
    fn get_space(&self) -> Result<Space> { unimplemented!() }
    fn get_myself(&self) -> Result<User> { unimplemented!() }
    fn get_space_activities(&self) -> Result<Vec<Activity>> { unimplemented!() }
    fn get_my_resource(&self) -> Result<MyResource> {
        self.data.clone().ok_or_else(|| anyhow!("no data"))
    }
}
```

Forgetting a method causes a compile error.

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
