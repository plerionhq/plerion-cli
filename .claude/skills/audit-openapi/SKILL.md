---
name: audit-openapi
description: Audit CLI and endpoint implementations against the Plerion OpenAPI spec. Reports missing params, coverage percentages, and gaps per endpoint.
disable-model-invocation: true
allowed-tools: Read Bash(gh *) Grep Glob Agent WebFetch
argument-hint: [--fix]
---

# OpenAPI Spec vs Implementation Audit

Perform a systematic audit of every Plerion API endpoint comparing the OpenAPI spec parameters against both the endpoint layer (`src/api/endpoints/*.rs`) and the CLI layer (`src/cli/*.rs`).

If `$ARGUMENTS` contains `--fix`, also implement the missing params. Otherwise, only report.

## Procedure

### Step 1: Gather the OpenAPI spec

Fetch the canonical spec using:

```!
gh api repos/plerionhq/docs/contents/api-reference/openapi.yaml -H "Accept: application/vnd.github.raw+json" 2>/dev/null | head -c 100
```

If the above snippet shows YAML content, fetch the full file:
```
gh api repos/plerionhq/docs/contents/api-reference/openapi.yaml -H "Accept: application/vnd.github.raw+json" > /private/tmp/openapi.yaml
```

Otherwise fall back to cloning or use a cached copy.

### Step 2: Extract all spec parameters

For every path+method in the spec, extract:
- Path and HTTP method
- operationId
- ALL query parameters (name, type, required, enum values)
- ALL path parameters
- Request body schema fields (for POST/PATCH/PUT)

### Step 3: Read the implementation

Read ALL files in:
- `src/api/endpoints/*.rs` -- endpoint param structs and function signatures
- `src/cli/*.rs` -- clap Args structs (the CLI flags users see)

Use parallel agents to speed this up.

### Step 4: Compare systematically

For each endpoint, check three things:

1. **Endpoint layer**: Does the params struct in `src/api/endpoints/<resource>.rs` have a field for every spec query parameter? Does the function body actually send it as a query param?
2. **CLI layer**: Does `src/cli/<resource>.rs` have a clap `#[arg]` for every endpoint param? Does the handler actually pass it through to the endpoint params struct?
3. **Request bodies**: For POST/PATCH endpoints, does the CLI expose all required and optional body fields?

Also check for:
- Boolean flags defined as bare `bool` instead of `Option<bool>` (prevents filtering for `false`)
- Pagination params: does `--all` + `--per-page` exist for paginated endpoints?
- Param naming mismatches between CLI flag, endpoint struct field, and API query param name

### Step 5: Produce the report

Output the report in this exact format:

#### Per-Endpoint Scorecard Table

```
| Endpoint | Spec Params | Endpoint Layer | CLI Layer |
|---|---|---|---|
| Findings | 19 | 19 (100%) | 17 (89%) |
| ... | ... | ... | ... |
| TOTAL | N | N (X%) | N (Y%) |
```

#### Gap Tables (grouped by priority)

**HIGH -- Missing from both endpoint + CLI:**
Table with: Endpoint, Missing Param, Notes

**MEDIUM -- In endpoint but not in CLI:**
Table with: Command, Missing CLI Flag, Endpoint Field

**MEDIUM -- Request body field gaps:**
Table with: Command, Missing Field, Notes (mark required fields)

**LOW -- UX issues:**
Boolean flags that can't filter for `false`, naming mismatches, etc.

#### Fully Covered Endpoints
List all endpoints with 0 gaps.

### Step 6: (If --fix) Implement fixes

If `--fix` was passed:
1. Start with HIGH priority gaps (missing from both layers)
2. Then MEDIUM (CLI-only gaps)
3. For each gap:
   - Add the field to the endpoint params struct
   - Add the query param serialization in the endpoint function
   - Add the clap `#[arg]` in the CLI handler
   - Wire the CLI arg through to the endpoint param
   - Add a test in `tests/<resource>_test.rs`
4. Run `cargo test` after each resource to catch issues early
5. Update docs if flags were added

## Notes

- Count pagination params (cursor, page, perPage, limit) in the total but consider them "covered" if `--all` + `--per-page` exist
- The vuln exemptions endpoint uses non-standard pagination: `limit`/`cursor` params and `hasNext`/`nextCursor` response fields
- Some API params accept both strings and integers for numeric meta values -- the custom deserializer `deserialize_option_u32_or_string` handles this
- Sort order enum casing varies: some endpoints use `ASC`/`DESC`, others use `asc`/`desc`
