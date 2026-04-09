#!/bin/sh
# Plerion CLI installer — auto-detects OS and architecture.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/plerionhq/plerion-cli/main/install.sh | sh
#   VERSION=v0.1.1 curl -fsSL ... | sh    # pin a specific version
#   INSTALL_DIR=~/.local/bin curl -fsSL ... | sh
set -e

REPO="plerionhq/plerion-cli"
VERSION="${VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

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

if [ "$VERSION" = "latest" ]; then
  URL="https://github.com/$REPO/releases/latest/download/$BINARY"
else
  URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY"
fi

DEST="$INSTALL_DIR/plerion"

echo "Detected: $OS/$ARCH"
echo "Downloading $BINARY ($VERSION)..."
curl -fsSL "$URL" -o "$DEST"
chmod +x "$DEST"

# Remove macOS quarantine attribute so Gatekeeper doesn't block the binary.
if [ "$OS" = "Darwin" ]; then
  xattr -d com.apple.quarantine "$DEST" 2>/dev/null || true
fi

echo ""
echo "Installed: $DEST"
"$DEST" --version
