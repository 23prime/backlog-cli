# ユーザーガイド

## 目次

- [前提条件](#前提条件)
- [インストール](#インストール)
- [アンインストール](#アンインストール)
- [認証](#認証)
- [コマンド](#コマンド)
- [コマンドカバレッジ](#コマンドカバレッジ)
- [設定ファイル](#設定ファイル)
- [トラブルシューティング](#トラブルシューティング)

## 前提条件

- 少なくとも 1 つのスペースにアクセスできる [Backlog](https://backlog.com) アカウント
- Backlog API キーまたは OAuth 2.0 クライアント認証情報（[認証](#認証) を参照）

## インストール

### 対応プラットフォーム

| OS | アーキテクチャ |
| --- | --- |
| Linux | x86\_64, aarch64 |
| macOS | x86\_64 (Intel), arm64 (Apple Silicon) |
| Windows | x86\_64 |

### インストールスクリプト（Linux, macOS）

`curl` と `tar` が必要です。OS とアーキテクチャを自動検出し、対応バイナリを選択して SHA-256 チェックサムを検証してからインストールします。

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

デフォルトのインストール先は `~/.local/bin/bl` です。
場所を変更するには `INSTALL_DIR` 環境変数を設定してください。

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.sh | sh
```

### インストールスクリプト（Windows）

PowerShell 5.1 以降（Windows 10/11 に組み込み）が必要です。

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1 | iex
```

デフォルトのインストール先は `%USERPROFILE%\.local\bin\bl.exe` です。
場所を変更するには次のようにします。

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/install.ps1))) -InstallDir 'C:\Tools'
```

### ソースからビルド

```bash
git clone https://github.com/23prime/backlog-cli.git
cd backlog-cli
cargo install --path .
```

## アンインストール

### アンインストールスクリプト（Linux, macOS）

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh
```

認証情報と設定ファイルも削除するには `--purge` を指定します。

```bash
curl -fsSL https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.sh | sh -s -- --purge
```

### アンインストールスクリプト（Windows）

```powershell
irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1 | iex
```

認証情報と設定ファイルも削除するには `-Purge` を指定します。

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/23prime/backlog-cli/latest/uninstall.ps1))) -Purge
```

> **注意:** `--purge` / `-Purge` を指定すると、アンインストールスクリプトはまず `bl auth logout --all` を実行してシステムキーリングのすべての API キーと設定ファイルを削除し、その後 Backlog CLI の設定ディレクトリとバイナリを削除します。
> このフラグを指定しない場合はバイナリのみが削除され、認証情報はそのまま残ります（後で再インストールする予定がある場合に便利です）。
>
> `bl auth logout --all` を実行すれば、いつでも手動で認証情報を削除できます。

## 認証

### API キーの発行

1. Backlog スペースにログインします
2. **個人設定** → **API** に移動します
3. メモを入力して **送信** をクリックします
4. 生成された API キーをコピーします

### API キーでログイン

```bash
bl auth login
```

以下の入力を求められます。

- **スペースキー** — Backlog スペースのサブドメイン。
  `mycompany.backlog.com` の場合は `mycompany` と入力します。
- **API キー** — 上の手順で発行したキー（入力は非表示）

別のスペースキーで `bl auth login` を再実行すると、そのスペースが追加されます。
最後にログインしたスペースがカレント（アクティブ）スペースになります。

### OAuth 2.0 でログイン

API キーの代わりに、ブラウザベースの OAuth 2.0 認証を使用できます。

#### ステップ 1 — Backlog で OAuth アプリケーションを登録する

1. スペース管理者として Backlog にログインします
2. **スペース設定** → **アプリ** に移動します
3. 新しいアプリケーションを作成します:
   - **リダイレクト URI**: `http://localhost:54321/callback`
     （`--port <port>` を使用する場合は `http://localhost:<port>/callback`）
   - **アプリケーション種別**: Confidential Client
4. **クライアント ID** と **クライアントシークレット** を控えておきます

#### ステップ 2 — OAuth ログインコマンドを実行する

```bash
bl auth login oauth
```

以下の入力を求められます。

- **スペースキー** — Backlog スペースのサブドメイン
- **クライアント ID** — 登録したアプリケーションのクライアント ID
- **クライアントシークレット** — 登録したアプリケーションのクライアントシークレット（入力は非表示）

コマンドを実行するとブラウザが開き、Backlog の認証画面が表示されます。
承認後、ブラウザは `http://localhost:54321/callback` にリダイレクトされ、
アクセストークンが自動的に保存されます。

カスタムポートを使用する場合（Backlog に登録したリダイレクト URI と一致させてください）:

```bash
bl auth login oauth --port 8080
```

### 複数スペースの管理

```bash
# 設定済みスペースの一覧表示（* がカレントスペース）
bl auth list

# カレントスペースの切り替え
bl auth use another-company

# 1 コマンドだけ別のスペースを使う
bl --space another-company project list

# BL_SPACE 環境変数で指定する
export BL_SPACE=another-company
bl project list

# 環境変数で認証情報を注入する（CI/CD での使用に最適）
export BL_SPACE=mycompany
export BL_API_KEY=your-api-key
bl project list
```

### 認証状態の確認

```bash
bl auth status
```

Backlog API に対して認証情報を検証し、以下のように表示します。

```text
Space: mycompany.backlog.com
  - API key: abcd...
  - Stored in: System keyring
  - Logged in as Your Name (your-id)
```

OAuth 2.0 認証の場合:

```text
Space: mycompany.backlog.com
  - Auth method: OAuth 2.0
  - Client ID: abc123
  - Logged in as Your Name (your-id)
```

`BL_API_KEY` が設定されている場合、`Stored in` には `Environment variable` と表示されます。

### ログアウト

```bash
# カレントスペースからログアウト
bl auth logout

# 特定のスペースからログアウト
bl auth logout another-company

# すべてのスペースからログアウトし、設定ファイルをすべて削除（アンインストール前に便利）
bl auth logout --all
```

## コマンド

### グローバルオプション

| オプション | 説明 |
| --- | --- |
| `--banner` | Backlog CLI バナーを表示して終了 |
| `--no-color` | カラー出力を無効化 |
| `--space <SPACE_KEY>` | このコマンドのみアクティブスペースを上書き |

### `bl auth`

| コマンド | 説明 |
| --- | --- |
| `bl auth login` | Backlog API キーで認証（スペースを追加または更新）。`--no-banner` でバナーをスキップ |
| `bl auth login oauth` | ブラウザベースの OAuth 2.0 で認証。`--port <port>` でコールバックポートを変更（デフォルト: 54321） |
| `bl auth status` | 現在の認証状態を表示して認証情報を検証 |
| `bl auth list` | 設定済みスペースの一覧を表示 |
| `bl auth use <space-key>` | カレントスペースを切り替え |
| `bl auth logout [<space-key>]` | カレントまたは指定スペースの認証情報を削除 |
| `bl auth logout --all` | すべてのスペースを削除し、設定ファイルをすべて削除 |

### `bl space`

Backlog スペースの情報を表示します。

```bash
bl space
```

出力例:

```text
Space key:  mycompany
Name:       My Company
Language:   ja
Timezone:   Asia/Tokyo
Formatting: markdown
Created:    2020-01-01T00:00:00Z
Updated:    2024-06-01T00:00:00Z
```

### `bl space activities`

Backlog スペースの最近のアクティビティを表示します。

```bash
bl space activities
bl space activities --json
```

出力例:

```text
[123] type=1 project=TEST user=John Doe created=2024-06-01T00:00:00Z
[124] type=2 project=TEST user=Jane Smith created=2024-06-02T00:00:00Z
```

### `bl space disk-usage`

Backlog スペースのディスク使用量を表示します。
スペース管理者権限が必要です。管理者以外のユーザーは `403 Forbidden` が返されます。

```bash
bl space disk-usage
bl space disk-usage --json
```

出力例:

```text
Capacity:   5242880 bytes
Issue:      2048 bytes
Wiki:       512 bytes
File:       1024 bytes
Subversion: 64 bytes
Git:        256 bytes
Git LFS:    128 bytes
Details:    3 project(s) — use --json for breakdown
```

### `bl space notification`

Backlog スペースに設定されたお知らせを表示します。

```bash
bl space notification
bl space notification --json
```

出力例:

```text
Updated: 2024-06-18T07:55:37Z

Scheduled maintenance on 2024-07-01.
```

お知らせが設定されていない場合:

```text
Updated: (not set)

(no notification set)
```

### `bl project list`

アクセス可能なすべてのプロジェクトを一覧表示します。

```bash
bl project list
bl project list --json
```

出力例:

```text
[TEST] Test Project
[PROD] Production [archived]
```

### `bl project show`

特定のプロジェクトの詳細を表示します。

```bash
bl project show <id-or-key>
bl project show <id-or-key> --json
```

出力例:

```text
ID:         1
Key:        TEST
Name:       Test Project
Formatting: markdown
Archived:   false
```

### `bl project activities`

特定のプロジェクトの最近のアクティビティを表示します。

```bash
bl project activities <id-or-key>
bl project activities <id-or-key> --json
```

出力例:

```text
[123] type=1 project=TEST user=John Doe created=2024-06-01T00:00:00Z
```

### `bl project disk-usage`

特定のプロジェクトのディスク使用量を表示します。
スペース管理者権限が必要です。管理者以外のユーザーは `403 Forbidden` が返されます。

```bash
bl project disk-usage <id-or-key>
bl project disk-usage <id-or-key> --json
```

出力例:

```text
Issue:      2048 bytes
Wiki:       512 bytes
Document:   0 bytes
File:       1024 bytes
Subversion: 64 bytes
Git:        256 bytes
Git LFS:    128 bytes
```

### `bl project user list`

特定のプロジェクトのメンバー一覧を表示します。

```bash
bl project user list <id-or-key>
bl project user list <id-or-key> --json
```

出力例:

```text
[john] John Doe
[jane] Jane Smith
```

### `bl project status list`

特定のプロジェクトに定義された課題ステータスの一覧を表示します。

```bash
bl project status list <id-or-key>
bl project status list <id-or-key> --json
```

出力例:

```text
[1] Open
[2] In Progress
[3] Resolved
[4] Closed
```

### `bl project issue-type list`

特定のプロジェクトに定義された課題種別の一覧を表示します。

```bash
bl project issue-type list <id-or-key>
bl project issue-type list <id-or-key> --json
```

出力例:

```text
[1] Bug
[2] Task
[3] Feature Request
```

### `bl project category list`

特定のプロジェクトに定義されたカテゴリの一覧を表示します。

```bash
bl project category list <id-or-key>
bl project category list <id-or-key> --json
```

出力例:

```text
[11] Development
[12] Design
```

### `bl project version list`

特定のプロジェクトに定義されたバージョン（マイルストーン）の一覧を表示します。

```bash
bl project version list <id-or-key>
bl project version list <id-or-key> --json
```

出力例:

```text
[3] Version 0.1 (2024-01-01T00:00:00Z → 2024-01-31T00:00:00Z)
[4] Version 0.2 [archived]
```

### `bl issue list`

課題を絞り込み条件付きで一覧表示します。

```bash
bl issue list
bl issue list --project-id 1 --status-id 1
bl issue list --issue-type-id 1 --category-id 2 --milestone-id 3
bl issue list --parent-child not-child --keyword "login" --count 50
bl issue list --json
```

`--parent-child` の値: `all`, `not-child`, `child`, `standalone`, `parent`

出力例:

```text
[TEST-1] Fix login issue (Open, Normal, -)
[TEST-2] Add dark mode (In Progress, Normal, John Doe)
```

### `bl issue count`

絞り込み条件付きで課題数をカウントします。`bl issue list` と同じフィルターが使えます。

```bash
bl issue count
bl issue count --project-id 1 --issue-type-id 1 --parent-child not-child --json
```

出力例:

```text
42
```

### `bl issue show`

特定の課題の詳細を表示します。

```bash
bl issue show <id-or-key>
bl issue show TEST-1 --json
```

出力例:

```text
TEST-1 Fix login issue
  Status:     Open
  Priority:   Normal
  Type:       Bug
  Assignee:   -
  Created:    2024-01-01T00:00:00Z
  Updated:    2024-06-01T00:00:00Z
```

### `bl issue create`

新しい課題を作成します。`--project-id`、`--summary`、`--issue-type-id`、`--priority-id` は必須です。

```bash
bl issue create --project-id 1 --summary "Fix login" --issue-type-id 1 --priority-id 2
bl issue create --project-id 1 --summary "Bug" --issue-type-id 1 --priority-id 2 \
  --description "Details..." --assignee-id 123 --due-date 2024-12-31 --json
```

優先度 ID: `1` = 高, `2` = 中, `3` = 低

### `bl issue update`

既存の課題を更新します。すべてのフィールドはオプションです。

```bash
bl issue update TEST-1 --summary "Updated summary"
bl issue update TEST-1 --status-id 2 --comment "Fixed in v1.2" --json
```

### `bl issue delete`

課題を削除します。

```bash
bl issue delete TEST-1
bl issue delete TEST-1 --json
```

出力例:

```text
Deleted: TEST-1
```

### `bl issue comment list`

課題のコメント一覧を表示します。

```bash
bl issue comment list TEST-1
bl issue comment list TEST-1 --json
```

出力例:

```text
[1] John Doe (2024-01-01T00:00:00Z): Fixed the issue.
[2] Jane Smith (2024-01-02T00:00:00Z): Confirmed.
```

### `bl issue comment add`

課題にコメントを追加します。

```bash
bl issue comment add TEST-1 --content "This is a comment"
bl issue comment add TEST-1 --content "Done" --json
```

### `bl issue comment update`

既存のコメントを更新します。

```bash
bl issue comment update TEST-1 42 --content "Updated comment"
bl issue comment update TEST-1 42 --content "Fixed" --json
```

### `bl issue comment delete`

コメントを削除します。

```bash
bl issue comment delete TEST-1 42
bl issue comment delete TEST-1 42 --json
```

### `bl issue attachment list`

課題の添付ファイル一覧を表示します。

```bash
bl issue attachment list TEST-1
bl issue attachment list TEST-1 --json
```

出力例:

```text
[1] screenshot.png (204800 bytes)
[2] log.txt (1024 bytes)
```

## Wiki ページ

### `bl wiki list`

プロジェクトの Wiki ページ一覧を表示します。

```bash
bl wiki list TEST
bl wiki list TEST --keyword setup
bl wiki list TEST --json
```

出力例:

```text
Home [guide, onboarding]
Setup
API Reference
```

### `bl wiki show`

Wiki ページの内容を表示します。

```bash
bl wiki show 12345
bl wiki show 12345 --json
```

出力例:

```text
Home
  Tags:    guide, onboarding
  Created: 2024-01-01T00:00:00Z
  Updated: 2024-06-01T00:00:00Z

# Home
Welcome to the project wiki!
```

### `bl wiki create`

新しい Wiki ページを作成します。

```bash
bl wiki create --project-id 1 --name "Setup" --content "# Setup\nSee README."
bl wiki create --project-id 1 --name "Setup" --content "# Setup" --mail-notify --json
```

### `bl wiki update`

既存の Wiki ページを更新します。`--name` または `--content` のどちらか一方は必須です。

```bash
bl wiki update 12345 --content "# Updated content"
bl wiki update 12345 --name "New Title" --content "New content" --mail-notify
bl wiki update 12345 --name "Renamed" --json
```

### `bl wiki delete`

Wiki ページを削除します。

```bash
bl wiki delete 12345
bl wiki delete 12345 --mail-notify --json
```

### `bl wiki history`

Wiki ページの変更履歴を表示します。

```bash
bl wiki history 12345
bl wiki history 12345 --json
```

出力例:

```text
v3 Home — 2024-06-01T00:00:00Z
v2 Home — 2024-03-15T00:00:00Z
v1 Home — 2024-01-01T00:00:00Z
```

### `bl wiki attachment list`

Wiki ページの添付ファイル一覧を表示します。

```bash
bl wiki attachment list 12345
bl wiki attachment list 12345 --json
```

出力例:

```text
[1] diagram.png (204800 bytes)
[2] notes.txt (1024 bytes)
```

## コマンドカバレッジ

以下の表は Backlog API v2 エンドポイントと `bl` コマンドの対応一覧です。

### スペース

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl space` | `GET /api/v2/space` | ✅ 実装済み |
| `bl space activities` | `GET /api/v2/space/activities` | ✅ 実装済み |
| `bl space disk-usage` | `GET /api/v2/space/diskUsage` | ✅ 実装済み |
| `bl space notification` | `GET /api/v2/space/notification` | ✅ 実装済み |

### プロジェクト

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl project list` | `GET /api/v2/projects` | ✅ 実装済み |
| `bl project show <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}` | ✅ 実装済み |
| `bl project activities <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/activities` | ✅ 実装済み |
| `bl project disk-usage <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/diskUsage` | ✅ 実装済み |
| `bl project user list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/users` | ✅ 実装済み |
| `bl project status list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/statuses` | ✅ 実装済み |
| `bl project issue-type list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/issueTypes` | ✅ 実装済み |
| `bl project category list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/categories` | ✅ 実装済み |
| `bl project version list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/versions` | ✅ 実装済み |

### 課題

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl issue list` | `GET /api/v2/issues` | ✅ 実装済み |
| `bl issue count` | `GET /api/v2/issues/count` | ✅ 実装済み |
| `bl issue show <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}` | ✅ 実装済み |
| `bl issue create` | `POST /api/v2/issues` | ✅ 実装済み |
| `bl issue update <id-or-key>` | `PATCH /api/v2/issues/{issueIdOrKey}` | ✅ 実装済み |
| `bl issue delete <id-or-key>` | `DELETE /api/v2/issues/{issueIdOrKey}` | ✅ 実装済み |
| `bl issue comment list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/comments` | ✅ 実装済み |
| `bl issue comment add <id-or-key>` | `POST /api/v2/issues/{issueIdOrKey}/comments` | ✅ 実装済み |
| `bl issue comment update <id-or-key> <comment-id>` | `PATCH /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ 実装済み |
| `bl issue comment delete <id-or-key> <comment-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ 実装済み |
| `bl issue attachment list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/attachments` | ✅ 実装済み |

### Wiki

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl wiki list` | `GET /api/v2/wikis` | ✅ 実装済み |
| `bl wiki show <id>` | `GET /api/v2/wikis/{wikiId}` | ✅ 実装済み |
| `bl wiki create` | `POST /api/v2/wikis` | ✅ 実装済み |
| `bl wiki update <id>` | `PATCH /api/v2/wikis/{wikiId}` | ✅ 実装済み |
| `bl wiki delete <id>` | `DELETE /api/v2/wikis/{wikiId}` | ✅ 実装済み |
| `bl wiki history <id>` | `GET /api/v2/wikis/{wikiId}/history` | ✅ 実装済み |
| `bl wiki attachment list <id>` | `GET /api/v2/wikis/{wikiId}/attachments` | ✅ 実装済み |

### プルリクエスト

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl pr list` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests` | Planned |
| `bl pr show <number>` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests/{number}` | Planned |
| `bl pr create` | `POST /api/v2/projects/{projectIdOrKey}/pullRequests` | Planned |
| `bl pr update <number>` | `PUT /api/v2/projects/{projectIdOrKey}/pullRequests/{number}` | Planned |
| `bl pr comment list <number>` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments` | Planned |
| `bl pr comment add <number>` | `POST /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments` | Planned |
| `bl pr comment update <number> <comment-id>` | `PUT /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments/{commentId}` | Planned |

### Git リポジトリ

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl git repo list` | `GET /api/v2/projects/{projectIdOrKey}/repositories` | Planned |
| `bl git repo show <repo>` | `GET /api/v2/projects/{projectIdOrKey}/repositories/{repoId}` | Planned |

### ユーザー

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl auth status` | `GET /api/v2/users/myself` | ✅ 実装済み（内部利用） |
| `bl user list` | `GET /api/v2/users` | Planned |
| `bl user show <id>` | `GET /api/v2/users/{userId}` | Planned |
| `bl user activities <id>` | `GET /api/v2/users/{userId}/activities` | Planned |
| `bl user recently-viewed <id>` | `GET /api/v2/users/{userId}/recentlyViewedIssues` | Planned |

### お知らせ

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl notification list` | `GET /api/v2/notifications` | Planned |
| `bl notification read <id>` | `PUT /api/v2/notifications/{notificationId}` | Planned |
| `bl notification read-all` | `DELETE /api/v2/notifications/unread` | Planned |

### ウォッチ

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl watch list` | `GET /api/v2/watching` | Planned |
| `bl watch add` | `POST /api/v2/watching` | Planned |
| `bl watch delete <id>` | `DELETE /api/v2/watching/{watchingId}` | Planned |

### チーム

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl team list` | `GET /api/v2/teams` | Planned |
| `bl team show <id>` | `GET /api/v2/teams/{teamId}` | Planned |

## 設定ファイル

### Linux / macOS

| 場所 | 内容 |
| --- | --- |
| `~/.config/bl/config.toml` | スペースキー（非機密メタデータ） |
| システムキーリング | API キーと OAuth トークン（優先; GNOME Keyring / Keychain） |
| `~/.config/bl/credentials.toml` | API キーのフォールバック（mode 0600、キーリングが使えない場合） |
| `~/.config/bl/oauth_tokens.toml` | OAuth トークンのフォールバック（mode 0600、キーリングが使えない場合） |

### Windows

| 場所 | 内容 |
| --- | --- |
| `%APPDATA%\bl\config.toml` | スペースキー（非機密メタデータ） |
| Windows 資格情報マネージャー | API キーと OAuth トークン（優先） |
| `%APPDATA%\bl\credentials.toml` | API キーのフォールバック（資格情報マネージャーが使えない場合） |
| `%APPDATA%\bl\oauth_tokens.toml` | OAuth トークンのフォールバック（資格情報マネージャーが使えない場合） |

### 設定ファイルの形式

```toml
current_space = "mycompany"
spaces = ["mycompany", "another-company"]
```

古い `[auth] space_key` 形式の設定は、初回実行時に自動的に移行されます。

## トラブルシューティング

### `API key not found. Run bl auth login to authenticate.`

API キーがキーリングに見つかりません。`bl auth login` を再実行してください。

### `API error (401 Unauthorized): Authentication failure`

スペースキーまたは API キーが正しくありません。以下を確認してください。

- スペースキーが Backlog URL と一致している（例: `mycompany.backlog.com` の場合は `mycompany`）
- API キーが Backlog の個人設定でまだ有効である

`bl auth login` を実行して認証情報を再入力してください。

### キーリングが利用できない

Linux では、キーリングには Secret Service デーモン（GNOME Keyring または KWallet）が起動している必要があります。
デーモンが利用できない場合（ヘッドレス環境や SSH 経由など）、`bl` は自動的に `~/.config/bl/credentials.toml`（mode 0600）にフォールバックします。

macOS ではシステムの Keychain、Windows では Windows 資格情報マネージャーが使用されます。
資格情報マネージャーが利用できない場合は `%APPDATA%\bl\credentials.toml` にフォールバックします。

`bl auth status` の出力で使用中のバックエンドを確認できます。

```text
  - Stored in: System keyring
```

または

```text
  - Stored in: Credentials file
```
