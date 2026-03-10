# トラブルシューティング

## `API key not found. Run bl auth login to authenticate.`

キーリングに API キーが存在しません。`bl auth login` を再実行してください。

## `API error (401 Unauthorized): Authentication failure`

スペースキーまたは API キーが正しくありません。以下を確認してください。

- スペースキーが Backlog URL と一致しているか（例: `mycompany.backlog.com` の場合は `mycompany`）
- API キーが Backlog の個人設定でまだ有効か

`bl auth login` を再実行して認証情報を再入力してください。

## キーリングが利用できない

Linux では、キーリングには Secret Service デーモン（GNOME Keyring または KWallet）の起動が必要です。
デーモンが利用できない場合（ヘッドレス環境や SSH 環境など）、`bl` は自動的に
`~/.config/bl/credentials.toml`（API キー）と `~/.config/bl/oauth_tokens.toml`（OAuth トークン）
へのフォールバックを使用します。どちらも mode 0600 で保存されます。

macOS ではシステム Keychain が使用されます。Windows では Windows 資格情報マネージャーが使用されます。
資格情報マネージャーが利用できない場合、`bl` は `%APPDATA%\bl\credentials.toml`（API キー）と
`%APPDATA%\bl\oauth_tokens.toml`（OAuth トークン）にフォールバックします。

`bl auth status` の出力でどのバックエンドが使用されているか確認できます。

```text
  - Stored in: System keyring
```

または

```text
  - Stored in: Credentials file
```
