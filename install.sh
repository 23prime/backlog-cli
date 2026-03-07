#!/bin/sh

set -eu

REPO="23prime/backlog-cli"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
      aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
      *) printf "[ERROR] Unsupported Linux architecture: %s\n" "$ARCH" >&2; exit 1 ;;
    esac
    ;;
  Darwin)
    case "$ARCH" in
      x86_64)  TARGET="x86_64-apple-darwin" ;;
      arm64)   TARGET="aarch64-apple-darwin" ;;
      *) printf "[ERROR] Unsupported macOS architecture: %s\n" "$ARCH" >&2; exit 1 ;;
    esac
    ;;
  *)
    printf "[ERROR] Unsupported OS: %s\n" "$OS" >&2
    exit 1
    ;;
esac

ASSET_NAME="bl-${TARGET}.tar.gz"

# Check required commands
for cmd in curl tar; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf "[ERROR] Required command '%s' not found. Please install it.\n" "$cmd" >&2
    exit 1
  fi
done

# Detect SHA-256 tool
if command -v sha256sum >/dev/null 2>&1; then
  SHA256_CMD="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  SHA256_CMD="shasum -a 256"
else
  printf "[ERROR] No SHA-256 tool found. Install sha256sum or shasum.\n" >&2
  exit 1
fi

# Get latest version from GitHub API
printf "Fetching latest release...\n"
response=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest") || {
  printf "[ERROR] Failed to fetch release info from GitHub API\n" >&2
  exit 1
}
tag=$(printf '%s' "$response" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p')

if [ -z "$tag" ]; then
  printf "[ERROR] Failed to fetch latest release\n" >&2
  exit 1
fi

printf "Latest version: %s\n" "$tag"

# Download and verify
url="https://github.com/${REPO}/releases/download/${tag}/${ASSET_NAME}"
checksum_url="${url}.sha256"
printf "Downloading %s...\n" "$url"

tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

curl -fsSL "$url" -o "${tmpdir}/${ASSET_NAME}"
curl -fsSL "$checksum_url" -o "${tmpdir}/${ASSET_NAME}.sha256"

printf "Verifying checksum...\n"
(cd "$tmpdir" && $SHA256_CMD -c "${ASSET_NAME}.sha256") || {
  printf "[ERROR] Checksum verification failed\n" >&2
  exit 1
}

tar xzf "${tmpdir}/${ASSET_NAME}" -C "$tmpdir"

# Install
mkdir -p "$INSTALL_DIR"
install -m 755 "${tmpdir}/bl" "${INSTALL_DIR}/bl"

printf "Installed bl to %s/bl\n" "$INSTALL_DIR"

# Check if INSTALL_DIR is in PATH
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) ;;
  *) printf "\nNote: %s is not in your PATH. Add it with:\n  export PATH=\"%s:\$PATH\"\n" "$INSTALL_DIR" "$INSTALL_DIR" ;;
esac
