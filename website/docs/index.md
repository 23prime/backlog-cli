---
slug: /
sidebar_position: 1
---

# Backlog CLI

An unofficial CLI tool for [Nulab's Backlog](https://backlog.com).

## Features

- 🌐 **Cross-platform** — Runs on Linux, macOS, and Windows (x86\_64 / aarch64 / Apple Silicon)
- 🔐 **Flexible authentication** — API key or browser-based OAuth 2.0; credentials stored in the system keyring with a file fallback
- 🏢 **Multi-space support** — Manage multiple Backlog spaces and switch between them with `bl auth use`
- 🔧 **JSON output** — All primary commands support `--json` for machine-readable output
- 🤖 **CI/CD friendly** — Inject credentials via `BL_API_KEY` and `BL_SPACE` environment variables; no interactive prompts needed
- 📦 **Single binary** — Just download and run; no extra setup required
- ⚡ **Easy install** — Single-command installation via shell script or PowerShell

## Quick Start

1. [Install](installation.md) `bl`

2. Authenticate — choose one method:

    ```bash
    bl auth login         # API key
    bl auth login-oauth   # OAuth 2.0 (browser-based)
    ```

3. Run commands:

    ```bash
    bl space
    bl issue list
    ```

See the [Authentication](authentication.md) and [Commands](commands.md) pages for full documentation.
