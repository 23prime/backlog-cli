# Installation

## Prerequisites

- A [Backlog](https://backlog.com) account with access to at least one space
- A Backlog API key or OAuth 2.0 client credentials (see [Authentication](authentication.md))

## Supported platforms

| OS | Architecture |
| --- | --- |
| Linux | x86\_64, aarch64 |
| macOS | x86\_64 (Intel), arm64 (Apple Silicon) |
| Windows | x86\_64 |

## Using the install script (Linux, macOS)

Requires `curl` and `tar`. The script auto-detects your OS and architecture, selects the matching binary,
and verifies its SHA-256 checksum before installing.

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

The binary is installed to `~/.local/bin/bl` by default.
To install to a different location, set the `INSTALL_DIR` environment variable:

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | INSTALL_DIR=/usr/local/bin sh
```

## Using the install script (Windows)

Requires PowerShell 5.1 or later (built-in on Windows 10/11).

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1 | iex
```

The binary is installed to `%USERPROFILE%\.local\bin\bl.exe` by default.
To install to a different location:

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1))) -InstallDir 'C:\Tools'
```

## Building from source

```bash
git clone https://github.com/23prime/backlog-cli.git
cd backlog-cli
cargo install --path .
```

## Uninstallation

### Uninstall script (Linux, macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh
```

To also remove stored credentials and configuration files, pass `--purge`:

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh -s -- --purge
```

### Uninstall script (Windows)

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1 | iex
```

To also remove stored credentials and configuration files, pass `-Purge`:

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1))) -Purge
```

> **Note:** With `--purge` / `-Purge`, the uninstall script first runs `bl auth logout --all`,
> which clears all API keys from the system keyring and removes all configuration files,
> and then deletes the Backlog CLI configuration directory along with the binary.
> Without this flag, only the binary is removed and credentials are left intact
> (useful if you plan to reinstall later).
>
> You can also clean up credentials manually at any time with `bl auth logout --all`.
