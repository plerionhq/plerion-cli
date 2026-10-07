# CLAUDE.md - Plerion CLI

## What this project is

A Rust CLI (`plerion`) that wraps the Plerion REST API (v1), covering the endpoints defined in the [OpenAPI spec](https://github.com/plerionhq/docs/blob/main/api-reference/openapi.yaml). It follows the AWS CLI UX pattern: INI profiles, `--output`, `--query`, coloured table output.

## Build & test

```bash
cargo build                    # debug build
cargo build --release          # release build (LTO, stripped)
cargo test                     # run all 315+ tests
cargo tarpaulin --skip-clean   # measure coverage (~92%)
```

## Architecture

```
src/
  main.rs           → entry point, parses Cli struct, dispatches to cli/*.rs handlers
  lib.rs            → re-exports for integration tests
  error.rs          → PlerionError enum (thiserror)
  config/
    mod.rs          → Config::load() with precedence: CLI flags > env vars > profile files > default
    credentials.rs  → INI read/write for ~/.plerion/credentials and ~/.plerion/config
  api/
    client.rs       → PlerionClient: reqwest wrapper with auth, base_url, HTTP methods
    models/*.rs     → 14 files: serde structs + TableRenderable impls
    endpoints/*.rs  → 15 files: async functions that call client methods
  cli/*.rs          → 16 files: clap Args/Subcommand + run() handlers
  output/
    mod.rs          → OutputFormat enum, apply_query (JMESPath), render/render_list/render_json_value
    table.rs        → comfy-table renderer, colorize_cell() applies Cell::fg(Color)
    json.rs, yaml.rs, text.rs → format-specific renderers
```

## Key patterns

### Adding an endpoint
1. Add model in `src/api/models/<resource>.rs` with `#[serde(rename_all = "camelCase")]`
2. Implement `TableRenderable` (return plain text from `row()` -- colors applied by table renderer)
3. Add endpoint function in `src/api/endpoints/<resource>.rs`; escape ids in the path with `segment` from `endpoints/path.rs`
4. Add CLI handler in `src/cli/<resource>.rs` with clap derive macros
5. Register in `mod.rs` files and add match arm in `main.rs`
6. Write mockito-based test in `tests/<resource>_test.rs`
7. Write CLI integration test in `tests/cli_integration_test.rs` or `tests/cli_crud_test.rs`
8. Run `python3 scripts/api_coverage.py` before opening the PR

### Table coloring
Colors are applied in `src/output/table.rs::colorize_cell()` using `Cell::fg(Color)`. Models must NOT embed ANSI codes in strings -- comfy-table handles width calculation correctly only when using its native color API.

### Config precedence
1. CLI flags (`--api-key`, `--region`, `--endpoint-url`)
2. Env vars: `PLERION_API_KEY`, `PLERION_REGION`, `PLERION_PROFILE`, `PLERION_ENDPOINT_URL`
3. Profile from `~/.plerion/credentials` + `~/.plerion/config`
4. `[default]` profile

### Regions
`au`, `sg1`, `in1`, `us1` → `https://{region}.api.plerion.com`
Custom endpoint via `--endpoint-url` bypasses region validation.

### Pagination
- **Cursor-based**: findings, alerts, risks, audit-logs, integrations, asset-groups, vuln-exemptions, access-grants, custom-checks, custom-reports (use `cursor` param)
- **Page-based**: assets, vulnerabilities, iac-scans, iac-findings, iac-vulnerabilities (use `page` param)
- `--all` flag auto-paginates through all pages on every list command that supports pagination
- Vuln exemptions use `limit`/`cursor` params and `hasNext`/`nextCursor` response fields (different from standard `PaginationMeta`)
- Access grants return no `hasNextPage`: `meta.cursor` is null on the last page and is the only end-of-pages signal, so `--all` loops until the cursor is null. `meta.total` is returned on the first page only
- Custom checks return `items` and a top-level `nextCursor` (no `data`/`meta`), and take `limit` rather than `perPage`; `--all` loops until `nextCursor` is absent
- Custom reports return no `hasNextPage`: `--all` loops until `meta.cursor` is null, and also stops on an empty page, since a page ending on the last dashboard still carries a cursor

## Testing approach

- `tests/*.rs` are integration tests that link against the `plerion` lib crate
- `mockito::Server` for HTTP mocking
- `PlerionClient::with_base_url()` for test clients
- CLI integration tests spawn the binary as a subprocess with `env!("CARGO_BIN_EXE_plerion")`
- Use `mockito::Matcher::Any` for query strings when the CLI adds default params (e.g. `perPage=50`)

## Dependencies

| Purpose | Crate |
|---|---|
| CLI parsing | `clap` v4 (derive) |
| HTTP | `reqwest` + `tokio` |
| Serialization | `serde` + `serde_json` + `serde_yaml` |
| INI config | `configparser` |
| Tables | `comfy-table` |
| JMESPath | `jmespatch` |
| Errors | `anyhow` + `thiserror` |
| Test mocking | `mockito` |

## OpenAPI coverage

`scripts/api_coverage.py` compares the `/v1/tenant/*` operations in the published API reference with the calls in `src/api/endpoints/` (and the IaC upload in `src/api/client.rs`) and lists any the CLI doesn't make. `.github/workflows/api-coverage.yml` runs it weekly and keeps one open issue, "CLI is missing customer API operations", up to date; it also runs on PRs that touch the endpoints, the script or the ignore file. Operations deliberately left out of the CLI go in `api-coverage-ignore.txt`, one operationId per line with a `#` reason. Use `--list-covered` to see what the script extracted.

## Release

Tag-triggered via `.github/workflows/release.yml`. Builds 5 platform binaries using `cross`.

## Docs

The CLI reference lives in plerionhq/docs under `cli-reference/`. It is hand-written, so update it in that repo when you add or change a flag or column.
