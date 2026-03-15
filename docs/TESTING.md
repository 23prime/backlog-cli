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
        then.status(200).json_body(json!({ "spaceKey": "my-space", ... }));
    });

    let client = BacklogClient::new_with(&server.base_url(), "test-key");
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
    assert!(show_with(true, &api).is_ok());
}

#[test]
fn show_with_propagates_api_error() {
    let api = MockApi { data: None };
    assert!(show_with(false, &api).unwrap_err().to_string().contains("no data"));
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

## Rules

- **Never** call `BacklogClient::from_config()` in tests — it requires real credentials on disk.
- Each test file should define its own `MockApi` struct that overrides only the methods it needs.
- Place shared test fixtures (e.g. `sample_foo()`) at module level with `#[cfg(test)]`, not inside
  `mod tests { ... }`, so sibling modules can reuse them.
