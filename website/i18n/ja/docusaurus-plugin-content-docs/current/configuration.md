# 設定ファイル

## Linux / macOS

| 場所 | 内容 |
| --- | --- |
| `~/.config/bl/config.toml` | スペースキー（非機密のメタデータ） |
| システムキーリング | API キーと OAuth トークン（優先；GNOME Keyring / Keychain） |
| `~/.config/bl/credentials.toml` | API キーのフォールバック（mode 0600、キーリングが利用できない場合に使用） |
| `~/.config/bl/oauth_tokens.toml` | OAuth トークンのフォールバック（mode 0600、キーリングが利用できない場合に使用） |

## Windows

| 場所 | 内容 |
| --- | --- |
| `%APPDATA%\bl\config.toml` | スペースキー（非機密のメタデータ） |
| Windows 資格情報マネージャー | API キーと OAuth トークン（優先） |
| `%APPDATA%\bl\credentials.toml` | API キーのフォールバック（資格情報マネージャーが利用できない場合に使用） |
| `%APPDATA%\bl\oauth_tokens.toml` | OAuth トークンのフォールバック（資格情報マネージャーが利用できない場合に使用） |

## 設定ファイルの形式

```toml
current_space = "mycompany"
spaces = ["mycompany", "another-company"]
```

旧形式（`[auth] space_key`）の設定ファイルは、初回実行時に自動的に移行されます。
