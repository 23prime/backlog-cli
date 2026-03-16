# Validation Guide

## Layer boundaries

Validation is split by responsibility. Each layer handles only what belongs to it.

| Layer | Responsibility |
| ----- | -------------- |
| `main.rs` (clap) | Syntactic/type-level checks only |
| `Args::try_new` | Domain invariants that must hold before any logic runs |
| `cmd/*_with` | API call logic and output formatting; API-spec constraints that depend on runtime data |
| `api/` | HTTP-level error translation only |

## `main.rs` (clap)

Handle syntactic and type-level validation only — things clap can check from the argument
definition itself (e.g. "cannot parse as `u64`", required flags, mutex groups).

```rust
/// Maximum number of results (1–100)
#[arg(long, default_value_t = 20)]
count: u64,
```

Do **not** add domain-level constraints here (e.g. `value_parser` rejecting values > 100).
Those belong in `try_new`.

## `Args::try_new`

Use a fallible constructor for domain invariants that must hold before any logic runs:

- At least one of several flags must be specified
- A count value must be within an API-defined range
- A list argument must be non-empty

```rust
#[derive(Debug)]
pub struct MyArgs {
    key: String,
    ids: Vec<u64>,
    json: bool,
}

impl MyArgs {
    pub fn try_new(key: String, ids: Vec<u64>, json: bool) -> anyhow::Result<Self> {
        if ids.is_empty() {
            anyhow::bail!("at least one --id is required");
        }
        Ok(Self { key, ids, json })
    }
}
```

Errors propagate naturally to `main` via `?`. Do **not** duplicate these checks in `*_with`.

### Update commands — require at least one optional field

`update` subcommands take all optional fields and should reject calls where every
field is `None`, because an empty PATCH request is meaningless and the API will error:

```rust
pub fn try_new(id: u64, name: Option<String>, /* … */, json: bool) -> anyhow::Result<Self> {
    if name.is_none() && /* all other fields */ .is_none() {
        anyhow::bail!("at least one of --name, … must be specified");
    }
    Ok(Self { id, name, /* … */, json })
}
```

### ID range validation (`min_id` / `max_id`)

When both `min_id` and `max_id` are provided, verify that `min_id <= max_id`:

```rust
if let (Some(min), Some(max)) = (min_id, max_id)
    && min > max
{
    anyhow::bail!("min-id must be less than or equal to max-id");
}
```

### When to use `try_new` vs plain `new`

Use `try_new` when construction can fail due to a domain invariant. Use `new` when all
arguments are always valid (e.g. a struct that holds a single string key with no constraints).

## `cmd/*_with`

API call logic and output formatting only. No validation that belongs in `try_new`.

Place API-spec constraints here **only** when they cannot be known at construction time
(e.g. a constraint that depends on a value fetched from the API).

```rust
pub fn list_with(args: &MyArgs, api: &dyn BacklogApi) -> Result<()> {
    let items = api.get_items(&args.key)?;
    // format and print — no domain validation here
    Ok(())
}
```

## `api/`

HTTP-level error translation only. Deserialize the response and surface HTTP errors as
`anyhow::Error`. No business logic.
