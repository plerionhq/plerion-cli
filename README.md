# Plerion CLI

[![Release](https://img.shields.io/github/v/release/plerionhq/plerion-cli?style=flat-square)](https://github.com/plerionhq/plerion-cli/releases)
[![Tests](https://img.shields.io/github/actions/workflow/status/plerionhq/plerion-cli/release.yml?style=flat-square&label=tests)](https://github.com/plerionhq/plerion-cli/actions)
[![Coverage](https://img.shields.io/badge/coverage-92%25-brightgreen?style=flat-square)](https://github.com/plerionhq/plerion-cli)
[![API Version](https://img.shields.io/badge/Plerion%20API-v1-blue?style=flat-square)](https://docs.plerion.com/api-reference)
[![OpenAPI Sync](https://img.shields.io/github/actions/workflow/status/plerionhq/plerion-cli/openapi-monitor.yml?style=flat-square&label=API%20sync)](https://github.com/plerionhq/plerion-cli/actions/workflows/openapi-monitor.yml)

A cross-platform command-line interface for the [Plerion](https://plerion.com) cloud security platform. Modelled on the AWS CLI experience with INI-based profiles, multiple output formats, JMESPath filtering, and coloured table output.

## Quick install

### From GitHub releases (recommended)

Download the latest binary for your platform from [Releases](https://github.com/plerionhq/plerion-cli/releases):

| Platform | Binary |
|---|---|
| macOS (Apple Silicon) | `plerion-aarch64-apple-darwin` |
| macOS (Intel) | `plerion-x86_64-apple-darwin` |
| Linux (x86_64) | `plerion-x86_64-unknown-linux-musl` |
| Linux (ARM64) | `plerion-aarch64-unknown-linux-musl` |
| Windows (x64) | `plerion-x86_64-pc-windows-msvc.exe` |

```bash
# Example: macOS Apple Silicon
curl -L -o plerion https://github.com/plerionhq/plerion-cli/releases/latest/download/plerion-aarch64-apple-darwin
chmod +x plerion
sudo mv plerion /usr/local/bin/
```

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

This creates `~/.plerion/credentials` and `~/.plerion/config` with your API key, region, and output preferences.

### Getting an API key

1. Sign in to the [Plerion dashboard](https://app.plerion.com)
2. Navigate to **Settings > API Keys**
3. Click **Create API Key** and copy the key

### Manual configuration

**~/.plerion/credentials**
```ini
[default]
api_key = pk_live_your_api_key_here
```

**~/.plerion/config**
```ini
[default]
region = au
output = table
```

### Environment variables

```bash
export PLERION_API_KEY=pk_live_your_api_key_here
export PLERION_REGION=au          # au, sg1, in1, us1
export PLERION_PROFILE=prod       # named profile
export PLERION_ENDPOINT_URL=...   # custom API base URL
```

### Config precedence (highest wins)

1. CLI flags (`--api-key`, `--region`, `--endpoint-url`)
2. Environment variables (`PLERION_API_KEY`, `PLERION_REGION`, etc.)
3. Named profile from config files (`--profile` or `PLERION_PROFILE`)
4. `[default]` profile

## Usage

```bash
plerion [--profile <name>] [--region <r>] [--output json|yaml|table|text] [--query <jmespath>] [--no-color] <command> <subcommand> [flags]
```

### Examples

```bash
# List critical findings
plerion findings list --severity CRITICAL --output table

# Get tenant info as JSON
plerion tenant get --output json

# Filter output with JMESPath
plerion findings list --output json --query 'data[0].detectionId'

# List assets that are publicly exposed
plerion assets list --is-publicly-exposed

# Fetch all pages automatically
plerion vulnerabilities list --severity CRITICAL,HIGH --all

# Use a named profile
plerion --profile prod findings list
```

## Commands

| Command | Description |
|---|---|
| `configure` | Interactive setup wizard / `configure list` to show profiles |
| `tenant get` | Get tenant details |
| `tenant get-usage` | Get tenant usage |
| `findings list` | List security findings with filters |
| `assets list` | List cloud assets |
| `assets get <id>` | Get asset details |
| `assets get-sbom <id>` | Get asset SBOM |
| `asset-groups list/get/create/update/delete` | Manage asset groups |
| `alerts list` | List alerts |
| `audit-logs list` | List audit logs |
| `integrations list` | List integrations |
| `risks list` | List risks |
| `vulnerabilities list` | List vulnerabilities |
| `vulnerabilities exemptions list/get/create/update/delete` | Manage vulnerability exemptions |
| `compliance-frameworks list` | List compliance frameworks |
| `compliance-frameworks request-report` | Request a compliance report |
| `compliance-frameworks download` | Download a compliance report |
| `well-architected-frameworks list/request-report/download` | Well-Architected operations |
| `iac scan` | Upload IaC for scanning |
| `iac list-scans` | List IaC scans |
| `iac get-findings <scan-id>` | Get IaC scan findings |
| `iac get-vulnerabilities <scan-id>` | Get IaC scan vulnerabilities |
| `aws get-external-id` | Get AWS external ID |
| `aws get-cloudformation-template` | Get CloudFormation template |
| `aws generate-token` | Generate temporary auth token |

## Output formats

| Format | Flag | Description |
|---|---|---|
| Table | `--output table` | Coloured, human-readable (default for TTY) |
| JSON | `--output json` | Pretty-printed JSON |
| YAML | `--output yaml` | YAML output |
| Text | `--output text` | Tab-separated for scripting |

## Regions

| Region | Code |
|---|---|
| Australia | `au` |
| Singapore | `sg1` |
| India | `in1` |
| United States | `us1` |

## License

MIT
