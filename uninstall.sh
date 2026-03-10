#!/bin/sh

set -eu

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
PURGE=0

for arg in "$@"; do
  case "$arg" in
    --purge) PURGE=1 ;;
    *) printf "[ERROR] Unknown option: %s\n" "$arg" >&2; exit 1 ;;
  esac
done

BL_BIN="${INSTALL_DIR}/bl"

if [ "$PURGE" = "1" ]; then
  OS=$(uname -s)
  case "$OS" in
    Linux)   CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/bl" ;;
    Darwin)  CONFIG_DIR="$HOME/Library/Application Support/bl" ;;
    *) printf "[ERROR] Unsupported OS: %s\n" "$OS" >&2; exit 1 ;;
  esac

  if [ -x "$BL_BIN" ]; then
    printf "Removing credentials...\n"
    "$BL_BIN" auth logout --all 2>/dev/null || true
  else
    printf "[WARNING] bl binary not found at %s; credentials were not removed automatically.\n" "$BL_BIN" >&2
    printf "Run 'bl auth logout --all' from the original install location to clear credentials.\n" >&2
  fi

  if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
    printf "Removed %s\n" "$CONFIG_DIR"
  fi
fi

if [ -f "$BL_BIN" ]; then
  rm -f "$BL_BIN"
  printf "Removed %s\n" "$BL_BIN"
else
  printf "bl not found at %s\n" "$BL_BIN"
fi

printf "Done.\n"
