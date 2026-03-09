# backlog-cli

[Nulab Backlog](https://backlog.com) 向け非公式 CLI ツールです。

## 特徴

- 🌐 **クロスプラットフォーム** — Linux・macOS・Windows（x86\_64 / aarch64 / Apple Silicon）に対応
- 🔐 **柔軟な認証** — API キーまたはブラウザ経由の OAuth 2.0 に対応。認証情報はシステムキーリング（GNOME Keyring・macOS Keychain・Windows 資格情報マネージャー）に保存し、利用できない場合はファイルにフォールバック
- 🏢 **マルチスペース対応** — 複数の Backlog スペースを管理し、`bl auth use` で切り替え可能
- 🔧 **JSON 出力** — 全コマンドで `--json` オプションをサポートし、機械可読な出力を提供
- 🤖 **CI/CD フレンドリー** — `BL_API_KEY` / `BL_SPACE` 環境変数で認証情報を注入可能。対話的な操作不要
- 📦 **シングルバイナリ** — ダウンロードしてすぐ実行可能。追加のセットアップ不要
- ⚡ **簡単インストール** — シェルスクリプトまたは PowerShell でワンコマンド導入

## インストール

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

### Windows

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1 | iex
```

その他のインストール方法（ソースからのビルドなど）は [ユーザーガイド](docs/user-guide.ja.md#インストール) を参照してください。

## 使い方

1. 認証します — どちらかの方法を選んでください。

    ```bash
    bl auth login         # API キー
    bl auth login-oauth   # OAuth 2.0（ブラウザ認証）
    ```

2. コマンドを実行します。
   例）スペース情報を表示する。

    ```bash
    bl space
    ```

詳細は [ユーザーガイド](docs/user-guide.ja.md) を参照してください。

## 開発

### 前提ツール

- [mise](https://mise.jdx.dev)
- [rustup](https://rustup.rs)

### コマンド

```bash
mise run setup   # ツールのインストール
mise run check   # リント / フォーマット / テスト
mise run fix     # 自動修正
```

### リリース

```bash
mise run release -- patch   # バージョンバンプ（patch / minor / major）とタグ作成
mise run tag-push           # タグをプッシュして CI リリースをトリガー
```
