# Plerion CLI

[![Release](https://img.shields.io/badge/release-v0.1.7-blue?style=flat-square)](https://github.com/plerionhq/plerion-cli/releases)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen?style=flat-square)](https://github.com/plerionhq/plerion-cli/actions)
[![Coverage](https://img.shields.io/badge/coverage-92%25-brightgreen?style=flat-square)](https://github.com/plerionhq/plerion-cli)
[![API Version](https://img.shields.io/badge/Plerion%20API-v1-blue?style=flat-square)](https://docs.plerion.com/api-reference)
[![Docs](https://img.shields.io/badge/docs-plerion.com-blue?style=flat-square)](https://docs.plerion.com/cli-reference/)

Cross-platform CLI for the [Plerion](https://plerion.com) cloud security platform.

## Install

### macOS / Linux (auto-detects OS and architecture)

```bash
curl -fsSL https://raw.githubusercontent.com/plerionhq/plerion-cli/main/install.sh | sh
```

To pin a version or change the install directory:

```bash
VERSION=v0.1.5 INSTALL_DIR=~/.local/bin \
  curl -fsSL https://raw.githubusercontent.com/plerionhq/plerion-cli/main/install.sh | sh
```

### macOS — "cannot verify" Gatekeeper warning

If macOS blocks the binary with a security warning, run:

```bash
xattr -d com.apple.quarantine /usr/local/bin/plerion
```

The install script does this automatically. It only affects manually downloaded binaries.

### macOS / Linux (manual — pick the right binary)

| Platform | Binary |
|---|---|
| macOS (Apple Silicon) | `plerion-macos-arm64` |
| macOS (Intel) | `plerion-macos-x86_64` |
| Linux (x86_64) | `plerion-linux-x86_64` |
| Linux (ARM64) | `plerion-linux-arm64` |

```bash
curl -fsSL https://github.com/plerionhq/plerion-cli/releases/latest/download/plerion-macos-arm64 \
  -o /usr/local/bin/plerion
chmod +x /usr/local/bin/plerion
```

### Windows

Download `plerion-windows-x86_64.exe` from the [latest release](https://github.com/plerionhq/plerion-cli/releases/latest), rename it to `plerion.exe`, and add it to your `PATH`.

### From source

```bash
git clone git@github.com:plerionhq/plerion-cli.git
cd plerion-cli
cargo build --release
cp target/release/plerion /usr/local/bin/
```

## Configuration

### Interactive setup

```bash
plerion configure
```

Creates `~/.plerion/credentials` and `~/.plerion/config`.

### Getting an API key

1. Sign in to the [Plerion dashboard](https://app.plerion.com)
2. Navigate to **Settings > API Keys**
3. Create a key and copy it

Verify with: `plerion tenant get`

### Manual configuration

**~/.plerion/credentials** (sections use bare profile names)
```ini
[default]
api_key = pk_live_your_api_key_here

[prod]
api_key = pk_live_prod_key_here
```

**~/.plerion/config** (same section names as credentials)
```ini
[default]
region = au
output = table

[prod]
region = us1
output = json
endpoint_url = https://us1.api.plerion.com
```

### Environment variables

| Variable | Description |
|---|---|
| `PLERION_API_KEY` | API key |
| `PLERION_REGION` | Region (`au`, `sg1`, `in1`, `us1`) |
| `PLERION_PROFILE` | Profile name |
| `PLERION_ENDPOINT_URL` | Custom API base URL (bypasses region validation) |
| `NO_COLOR` | Disable colour output |

### Config precedence (highest wins)

1. CLI flags (`--api-key`, `--region`, `--endpoint-url`)
2. Environment variables
3. Named profile from config files (`--profile` or `PLERION_PROFILE`)
4. `[default]` profile

## Usage

```bash
plerion [global flags] <command> <subcommand> [flags]
```

### Global flags

| Flag | Description |
|---|---|
| `--profile <name>` | Use a named profile |
| `--region <r>` | API region (`au`, `sg1`, `in1`, `us1`) |
| `--api-key <key>` | Override API key |
| `--endpoint-url <url>` | Custom API base URL |
| `--output <format>` | `table`, `json`, `yaml`, `text` |
| `--query <jmespath>` | JMESPath filter (output always JSON) |
| `--no-color` | Disable colour output |

### Examples

```bash
plerion findings list --severity CRITICAL --output table
plerion tenant get --output json
plerion findings list --output json --query 'data[0].detectionId'
plerion assets list --is-publicly-exposed
plerion vulnerabilities list --severity CRITICAL,HIGH --all
plerion --profile prod findings list
```

## Commands

| Command | Description |
|---|---|
| `configure` | Interactive setup wizard |
| `configure list` | List configured profiles |
| `tenant get` | Tenant details |
| `tenant get-usage` | Tenant usage |
| `findings list` | Security findings (supports `--all`) |
| `assets list` | Cloud assets (supports `--all`) |
| `assets get --asset-id <id>` | Asset details |
| `assets get-sbom --asset-id <id>` | Asset SBOM |
| `asset-groups list/get/create/update/delete` | Manage asset groups (list supports `--all`) |
| `alerts list` | Risk-based alerts (supports `--all`) |
| `audit-logs list` | Audit logs (supports `--all`) |
| `integrations list` | Cloud integrations (supports `--all`) |
| `risks list` | Security risks (supports `--all`) |
| `vulnerabilities list` | Vulnerabilities (supports `--all`) |
| `vulnerabilities exemptions list/get/create/update/delete` | Vulnerability exemptions (list supports `--all`) |
| `compliance-frameworks list` | Compliance frameworks |
| `compliance-frameworks request-report --integration-id <id> --framework-id <id>` | Request compliance report |
| `compliance-frameworks download --integration-id <id> --framework-id <id> [--output-file <path>]` | Download compliance report |
| `well-architected-frameworks list` | Well-Architected frameworks |
| `well-architected-frameworks request-report --integration-id <id> --framework-id <id>` | Request WAF report |
| `well-architected-frameworks download --integration-id <id> --framework-id <id> [--output-file <path>]` | Download WAF report |
| `iac scan --file <file.zip> --name <name>` | Upload IaC for scanning (max 4.4 MB) |
| `iac list-scans` | List IaC scans (supports filtering and `--all`) |
| `iac get-findings --scan-id <id>` | IaC scan findings (supports `--status`, `--severity`, `--all`) |
| `iac get-vulnerabilities --scan-id <id>` | IaC scan vulnerabilities (supports `--severity`, `--all`) |
| `aws get-external-id` | AWS external ID for role trust |
| `aws get-cloudformation-template` | CloudFormation setup template |
| `aws generate-token --integration-id <id>` | Temporary auth token |

Most list commands accept `--per-page <n>` (default 50) and `--sort-by`/`--sort-order`. Run `plerion <command> <subcommand> --help` for all options.

> **Note:** The global `--region` flag selects the API endpoint region. Some commands like `findings list` also have a `--region` flag that filters by cloud resource region (e.g. `us-east-1`). These are independent.

## Output formats

| Format | Flag | Description |
|---|---|---|
| Table | `--output table` | Coloured, human-readable (default). All API fields shown. |
| JSON | `--output json` | Pretty-printed JSON |
| YAML | `--output yaml` | YAML output |
| Text | `--output text` | Tab-separated for scripting |

Table output shows every field returned by the API — no columns are hidden. Use `--query` (JMESPath) to extract specific fields from JSON output, or pipe `--output text` to `cut`/`awk` for scripting.

## Regions

| Region | Code | Base URL |
|---|---|---|
| Australia | `au` | `https://au.api.plerion.com` |
| Singapore | `sg1` | `https://sg1.api.plerion.com` |
| India | `in1` | `https://in1.api.plerion.com` |
| United States | `us1` | `https://us1.api.plerion.com` |

Use `--endpoint-url` to override with a custom URL (bypasses region validation).

## License

MIT
