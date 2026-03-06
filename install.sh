#!/bin/sh

set -eu

REPO="23prime/backlog-cli"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
ASSET_NAME="bl-x86_64-unknown-linux-gnu.tar.gz"

# Check required commands
for cmd in curl tar; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf "[ERROR] Required command '%s' not found. Please install it.\n" "$cmd" >&2
    exit 1
  fi
done

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

# Download and extract
url="https://github.com/${REPO}/releases/download/${tag}/${ASSET_NAME}"
printf "Downloading %s...\n" "$url"

tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

curl -fsSL "$url" -o "${tmpdir}/${ASSET_NAME}"
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
