# Contributing to Plerion CLI

## Prerequisites

- **Rust** (stable, latest) - install via `rustup` or `brew install rust`
- **cargo** (comes with Rust)
- **git**

## Project structure

```
plerion-cli/
├── src/
│   ├── main.rs                # Entry point, CLI arg parsing
│   ├── lib.rs                 # Library re-exports
│   ├── error.rs               # PlerionError enum
│   ├── config/
│   │   ├── mod.rs             # Config struct, load() with precedence
│   │   └── credentials.rs     # INI file read/write
│   ├── api/
│   │   ├── client.rs          # HTTP client with auth
│   │   ├── models/            # Serde structs per resource (13 files)
│   │   └── endpoints/         # API call functions per resource (13 files)
│   ├── cli/                   # Clap subcommand definitions + handlers (14 files)
│   └── output/
│       ├── mod.rs             # OutputFormat enum, render dispatch, JMESPath
│       ├── json.rs            # JSON renderer
│       ├── yaml.rs            # YAML renderer
│       ├── table.rs           # comfy-table renderer with colors
│       └── text.rs            # Tab-separated renderer
├── tests/                     # Integration tests (30 files, 195+ tests)
├── docs/                      # Mintlify-compatible documentation
└── .github/workflows/         # CI/CD
```

## Development workflow

### Build

```bash
cargo build              # Debug build
cargo build --release    # Optimised release build
```

### Test

```bash
cargo test               # Run all tests
cargo test --test <name> # Run a specific test file
```

### Measure coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --skip-clean --out stdout
```

### Lint

```bash
cargo clippy             # Lint
cargo fmt                # Format
```

## Adding a new API endpoint

Follow this pattern (TDD):

### 1. Write the test first

Create or update a file in `tests/`:

```rust
// tests/my_resource_test.rs
use mockito::Server;
use plerion::api::{client::PlerionClient, endpoints::my_resource};

#[tokio::test]
async fn test_list_my_resource() {
    let mut server = Server::new_async().await;
    let body = serde_json::json!({
        "data": [{ "id": "r-1", "name": "Example" }],
        "meta": { "cursor": null, "perPage": 50 }
    });
    let _mock = server
        .mock("GET", "/v1/tenant/my-resource")
        .with_status(200)
        .with_body(body.to_string())
        .create_async()
        .await;

    let client = PlerionClient::with_base_url(&server.url(), "key").unwrap();
    let resp = my_resource::list(&client).await.unwrap();
    assert_eq!(resp.data.len(), 1);
}
```

### 2. Add the model

Create `src/api/models/my_resource.rs`:

```rust
use serde::{Deserialize, Serialize};
use crate::output::TableRenderable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MyResource {
    pub id: Option<String>,
    pub name: Option<String>,
}

impl TableRenderable for MyResource {
    fn headers() -> Vec<&'static str> { vec!["ID", "NAME"] }
    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
        ]
    }
}
```

Register in `src/api/models/mod.rs`.

### 3. Add the endpoint

Create `src/api/endpoints/my_resource.rs`:

```rust
use crate::api::client::PlerionClient;
use crate::error::PlerionError;

pub async fn list(client: &PlerionClient) -> Result<MyResponse, PlerionError> {
    client.execute(client.get("/v1/tenant/my-resource")).await
}
```

Register in `src/api/endpoints/mod.rs`.

### 4. Add the CLI handler

Create `src/cli/my_resource.rs` with clap `Args`/`Subcommand` structs and an async `run()` function. Register in `src/cli/mod.rs` and add the match arm in `src/main.rs`.

### 5. Add a CLI integration test

Add a test in `tests/cli_integration_test.rs` that invokes the binary with a mock server.

### 6. Run tests

```bash
cargo test
```

## Table rendering

Colors are applied by the table renderer in `src/output/table.rs` using `comfy-table`'s `Cell::fg(Color)` API. Models return plain text from `row()` -- never embed ANSI codes in cell strings.

## Key conventions

- **All struct fields are `Option<T>`** except where the API guarantees the field (e.g. `TenantData.tenant_id`)
- **Use `#[serde(rename_all = "camelCase")]`** on all model structs
- **Use `mockito`** for HTTP mocking in tests
- **Use `PlerionClient::with_base_url()`** in tests to point at the mock server
- **Query params** use camelCase in the API (e.g. `perPage`, `severityLevels`)

## Release process

Releases are automated via GitHub Actions on version tags:

```bash
# Update version in Cargo.toml
git tag v0.2.0
git push origin v0.2.0
```

The workflow builds binaries for 5 platforms and creates a GitHub Release.

### Supported targets

| Target | Platform |
|---|---|
| `x86_64-unknown-linux-musl` | Linux x86_64 (static) |
| `aarch64-unknown-linux-musl` | Linux ARM64 (static) |
| `x86_64-pc-windows-msvc` | Windows x64 |
| `x86_64-apple-darwin` | macOS Intel |
| `aarch64-apple-darwin` | macOS Apple Silicon |

## OpenAPI sync

A GitHub Action monitors the upstream [OpenAPI spec](https://github.com/plerionhq/docs/blob/main/api-reference/openapi.yaml) weekly. If the spec changes, the `openapi-monitor` workflow fails and the README badge turns red. Check the workflow output for the diff.
