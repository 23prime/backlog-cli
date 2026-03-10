# Troubleshooting

## `API key not found. Run bl auth login to authenticate.`

The API key is missing from the keyring. Run `bl auth login` again.

## `API error (401 Unauthorized): Authentication failure`

The space key or API key is incorrect. Check:

- The space key matches your Backlog URL (e.g. `mycompany` for `mycompany.backlog.com`)
- The API key is still valid in Backlog personal settings

Run `bl auth login` to re-enter your credentials.

## Keyring is unavailable

On Linux, the keyring requires a running Secret Service daemon (GNOME Keyring or KWallet).
If no daemon is available (e.g. headless or SSH environments), `bl` automatically falls back
to storing the API key in `~/.config/bl/credentials.toml` and OAuth tokens in
`~/.config/bl/oauth_tokens.toml`, both with mode 0600.

On macOS, the system Keychain is used. On Windows, the Windows Credential Manager is used.
If the Credential Manager is unavailable, `bl` falls back to `%APPDATA%\bl\credentials.toml`
(API key) and `%APPDATA%\bl\oauth_tokens.toml` (OAuth tokens).

The `bl auth status` output shows which backend is in use:

```text
  - Stored in: System keyring
```

or

```text
  - Stored in: Credentials file
```
