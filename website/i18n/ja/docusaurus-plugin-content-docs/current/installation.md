# インストール

## 前提条件

- [Backlog](https://backlog.com) アカウントと、少なくとも 1 つのスペースへのアクセス権
- Backlog API キーまたは OAuth 2.0 クライアント認証情報（[認証](authentication.md)を参照）

## 対応プラットフォーム

| OS | アーキテクチャ |
| --- | --- |
| Linux | x86\_64、aarch64 |
| macOS | x86\_64（Intel）、arm64（Apple Silicon） |
| Windows | x86\_64 |

## インストールスクリプト（Linux / macOS）

`curl` と `tar` が必要です。スクリプトが OS とアーキテクチャを自動検出し、対応するバイナリを選択して SHA-256 チェックサムを検証してからインストールします。

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

デフォルトのインストール先は `~/.local/bin/bl` です。
別の場所にインストールするには `INSTALL_DIR` 環境変数を設定します。

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

## インストールスクリプト（Windows）

PowerShell 5.1 以降が必要です（Windows 10/11 には標準搭載）。

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1 | iex
```

デフォルトのインストール先は `%USERPROFILE%\.local\bin\bl.exe` です。
別の場所にインストールするには次のように実行します。

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1))) -InstallDir 'C:\Tools'
```

## ソースからビルド

```bash
git clone https://github.com/23prime/backlog-cli.git
cd backlog-cli
cargo install --path .
```

## アンインストール

### アンインストールスクリプト（Linux / macOS）

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh
```

認証情報と設定ファイルも削除するには `--purge` を渡します。

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh -s -- --purge
```

### アンインストールスクリプト（Windows）

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1 | iex
```

認証情報と設定ファイルも削除するには `-Purge` を渡します。

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1))) -Purge
```

> **注意:** `--purge` / `-Purge` を指定すると、アンインストールスクリプトはまず `bl auth logout --all` を実行してすべての API キーをキーリングから削除し、すべての設定ファイルを削除してから、バイナリと設定ディレクトリを削除します。
> このフラグを指定しない場合はバイナリのみ削除され、認証情報は保持されます（再インストールを予定している場合に便利です）。
>
> 認証情報はいつでも `bl auth logout --all` で手動削除できます。
