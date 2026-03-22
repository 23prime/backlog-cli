# Testing Guide

## Overview

Tests are split by layer. Each layer has a distinct strategy that avoids requiring real
credentials or a live Backlog API.

## `api/` layer — httpmock

Use `httpmock` to spin up a local HTTP server and construct `BacklogClient::new_with(base_url, api_key)`.

```rust
use httpmock::prelude::*;

#[test]
fn get_space_returns_parsed_struct() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/space");
            then.status(200)
            .json_body(serde_json::json!({ "spaceKey": "my-space", "name": "My Space" }));
    });

    let client = BacklogClient::new_with(&server.base_url(), "test-key").unwrap();
    let space = client.get_space().unwrap();
    assert_eq!(space.space_key, "my-space");
}
```

Test both happy paths and error responses (4xx, 5xx).

## `cmd/` layer — MockApi

Implement `BacklogApi` on a minimal `MockApi` struct and call `*_with()` directly.

```rust
struct MockApi { data: Option<MyResource> }

impl BacklogApi for MockApi {
    fn get_my_resource(&self, _params: &[(String, String)]) -> Result<MyResource> {
        self.data.clone().ok_or_else(|| anyhow!("no data"))
    }
}

#[test]
fn show_with_json_output_succeeds() {
    let api = MockApi { data: Some(sample_resource()) };
    let args = MyArgs::new("TEST-1".to_string(), true);
    assert!(show_with(&args, &api).is_ok());
}

#[test]
fn show_with_propagates_api_error() {
    let api = MockApi { data: None };
    let args = MyArgs::new("TEST-1".to_string(), false);
    assert!(show_with(&args, &api).unwrap_err().to_string().contains("no data"));
}
```

### Default `unimplemented!()` bodies

`BacklogApi` has default `unimplemented!()` bodies for every trait method.
A `MockApi` only overrides the methods actually exercised by the test — no boilerplate stubs needed.
If a test exercises an unexpected code path, it panics with `not implemented`, which is the desired
behavior.

### Capturing params

When asserting that query/form parameters are built correctly, use `RefCell` to capture the params
passed to the mock:

```rust
use std::cell::RefCell;

struct MockApi {
    captured: RefCell<Vec<(String, String)>>,
}

impl BacklogApi for MockApi {
    fn my_method(&self, params: &[(String, String)]) -> Result<()> {
        *self.captured.borrow_mut() = params.to_vec();
        Ok(())
    }
}

#[test]
fn builds_correct_params() {
    let api = MockApi { captured: RefCell::new(vec![]) };
    my_cmd_with(&args, &api).unwrap();
    let params = api.captured.borrow();
    assert!(params.iter().any(|(k, v)| k == "targetKey" && v == "expected"));
}
```

## httpmock method constants

`httpmock::prelude::*` exports `GET`, `POST`, `PUT`, `DELETE`, `OPTIONS` —
but **not `PATCH`**. Use the fully-qualified path for PATCH requests:

```rust
when.method(httpmock::Method::PATCH).path("/projects/TEST");
```

## httpmock body matching

`body_contains` was removed in httpmock 0.8.x. Use `body_includes` instead:

```rust
// ✅ httpmock 0.8.x
when.body_includes("content=New+notification+text.");

// ❌ removed in 0.8.x
when.body_contains("content=New+notification+text.");
```

## Tests that change the working directory

When a test must call `std::env::set_current_dir`, always restore with a Drop guard so the
original directory is recovered even if the test panics:

```rust
struct DirGuard(std::path::PathBuf);
impl Drop for DirGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.0);
    }
}

let original = std::env::current_dir().unwrap();
std::env::set_current_dir(&tmp_dir).unwrap();
let _guard = DirGuard(original); // restored on drop, even on panic
```

Without this, a panicking test leaves the process in the wrong directory and breaks
subsequent tests that write relative paths.

## Test coverage for JSON vs text output branches

Every `*_with` function that prints differently based on `args.json` must have
**two** success tests: one with `json: false` and one with `json: true`. Omitting
the `json: true` test leaves the serialization path untested and is routinely
flagged by reviewers.

```rust
#[test]
fn add_with_text_output_succeeds() { /* json: false */ }

#[test]
fn add_with_json_output_succeeds() { /* json: true */ }
```

## ANSI color codes in format row tests

`owo_colors` wraps values in ANSI escape sequences. Asserting on bracket-wrapped strings like
`"[1]"` fails because the brackets are not adjacent to the digits in the raw string:

```rust
// ❌ fails — ANSI codes are inserted between "[" and "1" and "]"
assert!(row.contains("[1]"));

// ✅ check the text content only
assert!(row.contains("1"));
assert!(row.contains("Fix bug"));
```

This applies to any `format_*_row` function that uses `.cyan()`, `.bold()`, or similar.

## `#[cfg_attr(test, derive(Debug))]` for `try_new` Args structs

`Result::unwrap_err()` requires `T: Debug` so it can format the *Ok value* when the result is
unexpectedly `Ok`. When an Args struct using `try_new` is the `T` in `Result<T, _>`, tests that
call `.unwrap_err()` will fail to compile without `Debug`. Add the conditional derive to avoid
a compile error without bloating release builds:

```rust
#[cfg_attr(test, derive(Debug))]
pub struct MyUpdateArgs { ... }

#[test]
fn try_new_fails_when_no_fields_provided() {
    let err = MyUpdateArgs::try_new(...).unwrap_err();  // T=MyUpdateArgs needs Debug
    assert!(err.to_string().contains("at least one"));
}
```

## Rules

- **Never** call `BacklogClient::from_config()` in tests — it requires real credentials on disk.
- Each test file should define its own `MockApi` struct that overrides only the methods it needs.
- Place shared test fixtures (e.g. `sample_foo()`) at module level with `#[cfg(test)]`, not inside
  `mod tests { ... }`, so sibling modules can reuse them.
