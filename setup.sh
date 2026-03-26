#!/usr/bin/env bash
set -euo pipefail

TEMPLATE_REPO="${TEMPLATE_REPO:-https://github.com/23prime/mise-template.git}"

usage() {
  echo "Usage: setup.sh <remote-url> [repo-name]"
  echo ""
  echo "  <remote-url>  Remote URL of the pre-created repository to register as origin"
  echo "  [repo-name]   Repository name (default: derived from <remote-url>)"
  exit 1
}

# ----------------------------------------------------------------
# Argument parsing
# ----------------------------------------------------------------
if [[ $# -lt 1 ]]; then
  usage
fi

REMOTE_URL="$1"
REPO_NAME="${2:-}"

if [[ -z "$REPO_NAME" ]]; then
  REPO_NAME="$(basename "$REMOTE_URL" .git)"
fi

# ----------------------------------------------------------------
# Prerequisite checks
# ----------------------------------------------------------------
for cmd in git gh; do
  if ! command -v "$cmd" &>/dev/null; then
    echo "Error: '$cmd' is not installed or not in PATH." >&2
    exit 1
  fi
done

echo "Checking remote repository '$REMOTE_URL'..."
if ! git ls-remote "$REMOTE_URL" &>/dev/null; then
  echo "Error: Cannot access remote repository '$REMOTE_URL'." >&2
  echo "Make sure the repository exists and you have access to it." >&2
  exit 1
fi

# ----------------------------------------------------------------
# Setup
# ----------------------------------------------------------------
SETUP_TMPDIR="$(mktemp -d "${TMPDIR:-/tmp}/setup.XXXXXXXXXX")"
if [[ -z "${SETUP_TMPDIR:-}" || ! -d "$SETUP_TMPDIR" ]]; then
  echo "Error: Failed to create temporary directory." >&2
  exit 1
fi
trap 'rm -rf "$SETUP_TMPDIR"' EXIT

echo "Cloning template repository..."
git clone "$TEMPLATE_REPO" "$SETUP_TMPDIR/mise-template"

echo "Copying to '$REPO_NAME'..."
if [[ -e "$REPO_NAME" ]]; then
  echo "Error: Target path '$REPO_NAME' already exists. Please remove it or choose a different repo name." >&2
  exit 1
fi
cp -ar "$SETUP_TMPDIR/mise-template" "$REPO_NAME"

cd "$REPO_NAME"

echo "Renaming 'origin' to 'upstream'..."
git remote rename origin upstream

echo "Adding '$REMOTE_URL' as 'origin'..."
git remote add origin "$REMOTE_URL"

echo "Pushing to origin..."
git push -u origin main

echo "Setting default repository for gh CLI..."
gh repo set-default "$REPO_NAME"

echo ""
echo "Done! Your repository '$REPO_NAME' is ready."
echo "Run 'cd $REPO_NAME' to enter the repository."
