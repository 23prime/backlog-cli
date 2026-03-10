# Backlog CLI

[Nulab Backlog](https://backlog.com) 向け非公式 CLI ツールです。

## 特徴

- 🌐 **クロスプラットフォーム** — Linux・macOS・Windows（x86\_64 / aarch64 / Apple Silicon）に対応
- 🔐 **柔軟な認証** — API キーまたはブラウザ経由の OAuth 2.0 に対応。認証情報はシステムキーリングに保存し、利用できない場合はファイルにフォールバック
- 🏢 **マルチスペース対応** — 複数の Backlog スペースを管理し、`bl auth use` で切り替え可能
- 🔧 **JSON 出力** — 主要なコマンドで `--json` オプションをサポートし、機械可読な出力を提供
- 🤖 **CI/CD フレンドリー** — `BL_API_KEY` / `BL_SPACE` 環境変数で認証情報を注入可能。対話的な操作不要
- 📦 **シングルバイナリ** — ダウンロードしてすぐ実行可能。追加のセットアップ不要
- ⚡ **簡単インストール** — シェルスクリプトまたは PowerShell でワンコマンド導入

## クイックスタート

1. `bl` を[インストール](installation.md)する

2. 認証します — どちらかの方法を選んでください。

    ```bash
    bl auth login         # API キー
    bl auth login-oauth   # OAuth 2.0（ブラウザ認証）
    ```

3. コマンドを実行します。

    ```bash
    bl space
    bl issue list
    ```

詳細は[認証](authentication.md)・[コマンド](commands.md)のページを参照してください。
