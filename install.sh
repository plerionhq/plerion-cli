#!/bin/sh
# Plerion CLI installer — auto-detects OS and architecture.
#
# Requires the GitHub CLI (gh) to be installed and authenticated:
#   https://cli.github.com
#
# Usage:
#   ./install.sh
#   VERSION=v0.1.1 ./install.sh    # pin a specific version
#   INSTALL_DIR=~/.local/bin ./install.sh
set -e

REPO="plerionhq/plerion-cli"
VERSION="${VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Require gh CLI
if ! command -v gh >/dev/null 2>&1; then
  echo "error: the GitHub CLI (gh) is required to install plerion." >&2
  echo "Install it from https://cli.github.com, then run: gh auth login" >&2
  exit 1
fi

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64)  BINARY="plerion-macos-arm64" ;;
      x86_64) BINARY="plerion-macos-x86_64" ;;
      *) echo "error: unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64|amd64)  BINARY="plerion-linux-x86_64" ;;
      aarch64|arm64) BINARY="plerion-linux-arm64" ;;
      *) echo "error: unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "error: unsupported OS: $OS" >&2
    echo "For Windows, download from: https://github.com/$REPO/releases/latest" >&2
    exit 1
    ;;
esac

DEST="$INSTALL_DIR/plerion"
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Detected: $OS/$ARCH"
echo "Downloading $BINARY ($VERSION)..."

if [ "$VERSION" = "latest" ]; then
  gh release download --repo "$REPO" --pattern "$BINARY" --dir "$TMPDIR"
else
  gh release download "$VERSION" --repo "$REPO" --pattern "$BINARY" --dir "$TMPDIR"
fi

mv "$TMPDIR/$BINARY" "$DEST"
chmod +x "$DEST"

# Remove macOS quarantine attribute so Gatekeeper doesn't block the binary.
if [ "$OS" = "Darwin" ]; then
  xattr -d com.apple.quarantine "$DEST" 2>/dev/null || true
fi

echo ""
echo "Installed: $DEST"
"$DEST" --version
