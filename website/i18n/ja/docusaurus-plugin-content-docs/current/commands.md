# コマンド

## ルートオプション

以下のオプションはルートの `bl` コマンドにのみ指定できます。サブコマンドには指定できません。

| オプション | 説明 |
| --- | --- |
| `--banner` | Backlog CLI バナーを表示して終了 |

## グローバルオプション

以下のオプションはすべてのサブコマンドで指定できます。

| オプション | 説明 |
| --- | --- |
| `--no-color` | カラー出力を無効化 |
| `--space <SPACE_KEY>` | このコマンドのみ対象スペースを上書き |
| `--verbose` / `-v` | 詳細なログを stderr に出力 |

## `bl auth`

| コマンド | 説明 |
| --- | --- |
| `bl auth login` | Backlog API キーで認証（スペースの追加または更新）。`--no-banner` でバナーをスキップ |
| `bl auth login-oauth` | ブラウザ経由の OAuth 2.0 で認証。`--port <port>` でデフォルトのコールバックポート（54321）を変更。`--no-banner` でバナーをスキップ |
| `bl auth status` | 現在の認証状態を表示して認証情報を検証 |
| `bl auth list` | 設定済みスペースを一覧表示 |
| `bl auth use <space-key>` | カレントスペースを切り替え |
| `bl auth keyring` | システムキーリングが利用可能か確認 |
| `bl auth logout [<space-key>]` | カレントまたは指定スペースの認証情報を削除 |
| `bl auth logout --all` | すべてのスペースを削除してすべての設定ファイルを削除 |

## `bl space`

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

## `bl space activities`

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

## `bl space disk-usage`

Backlog スペースのディスク使用量を表示します。
スペース管理者権限が必要です。管理者以外のユーザーには `403 Forbidden` が返されます。

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

## `bl space notification`

Backlog スペースに設定された通知メッセージを表示します。

```bash
bl space notification
bl space notification --json
```

出力例:

```text
Updated: 2024-06-18T07:55:37Z

Scheduled maintenance on 2024-07-01.
```

通知が設定されていない場合:

```text
Updated: (not set)

(no notification set)
```

## `bl project list`

アクセス可能なプロジェクトを一覧表示します。

```bash
bl project list
bl project list --json
```

出力例:

```text
[TEST] Test Project
[PROD] Production [archived]
```

## `bl project show`

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

## `bl project activities`

特定のプロジェクトの最近のアクティビティを表示します。

```bash
bl project activities <id-or-key>
bl project activities <id-or-key> --json
```

出力例:

```text
[123] type=1 project=TEST user=John Doe created=2024-06-01T00:00:00Z
```

## `bl project disk-usage`

特定のプロジェクトのディスク使用量を表示します。
スペース管理者権限が必要です。管理者以外のユーザーには `403 Forbidden` が返されます。

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

## `bl project user list`

特定のプロジェクトのメンバーを一覧表示します。

```bash
bl project user list <id-or-key>
bl project user list <id-or-key> --json
```

出力例:

```text
[john] John Doe
[jane] Jane Smith
```

## `bl project status list`

特定のプロジェクトの課題ステータスを一覧表示します。

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

## `bl project issue-type list`

特定のプロジェクトの課題種別を一覧表示します。

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

## `bl project category list`

特定のプロジェクトのカテゴリーを一覧表示します。

```bash
bl project category list <id-or-key>
bl project category list <id-or-key> --json
```

出力例:

```text
[11] Development
[12] Design
```

## `bl project version list`

特定のプロジェクトのバージョン（マイルストーン）を一覧表示します。

```bash
bl project version list <id-or-key>
bl project version list <id-or-key> --json
```

出力例:

```text
[3] Version 0.1 (2024-01-01T00:00:00Z → 2024-01-31T00:00:00Z)
[4] Version 0.2 [archived]
```

## `bl issue list`

オプションのフィルターで課題を一覧表示します。

```bash
bl issue list
bl issue list --project-id 1 --status-id 1
bl issue list --issue-type-id 1 --category-id 2 --milestone-id 3
bl issue list --parent-child not-child --keyword "login" --count 50
bl issue list --json
```

`--parent-child` の値: `all`、`not-child`、`child`、`standalone`、`parent`

出力例:

```text
[TEST-1] Fix login issue (Open, Normal, -)
[TEST-2] Add dark mode (In Progress, Normal, John Doe)
```

## `bl issue count`

オプションのフィルターで課題数をカウントします。`bl issue list` と同じフィルターを使用できます。

```bash
bl issue count
bl issue count --project-id 1 --issue-type-id 1 --parent-child not-child --json
```

出力例:

```text
42
```

## `bl issue show`

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

## `bl issue create`

新しい課題を作成します。`--project-id`、`--summary`、`--issue-type-id`、`--priority-id` が必須です。

```bash
bl issue create --project-id 1 --summary "Fix login" --issue-type-id 1 --priority-id 2
bl issue create --project-id 1 --summary "Bug" --issue-type-id 1 --priority-id 2 \
  --description "Details..." --assignee-id 123 --due-date 2024-12-31 --json
```

優先度 ID: `1` = 高、`2` = 中、`3` = 低

## `bl issue update`

既存の課題を更新します。すべてのフィールドはオプションです。

```bash
bl issue update TEST-1 --summary "Updated summary"
bl issue update TEST-1 --status-id 2 --comment "Fixed in v1.2" --json
```

## `bl issue delete`

課題を削除します。

```bash
bl issue delete TEST-1
bl issue delete TEST-1 --json
```

出力例:

```text
Deleted: TEST-1
```

## `bl issue comment list`

課題のコメントを一覧表示します。

```bash
bl issue comment list TEST-1
bl issue comment list TEST-1 --json
```

出力例:

```text
[1] John Doe (2024-01-01T00:00:00Z): Fixed the issue.
[2] Jane Smith (2024-01-02T00:00:00Z): Confirmed.
```

## `bl issue comment add`

課題にコメントを追加します。

```bash
bl issue comment add TEST-1 --content "This is a comment"
bl issue comment add TEST-1 --content "Done" --json
```

## `bl issue comment update`

既存のコメントを更新します。

```bash
bl issue comment update TEST-1 42 --content "Updated comment"
bl issue comment update TEST-1 42 --content "Fixed" --json
```

## `bl issue comment delete`

コメントを削除します。

```bash
bl issue comment delete TEST-1 42
bl issue comment delete TEST-1 42 --json
```

## `bl issue attachment list`

課題の添付ファイルを一覧表示します。

```bash
bl issue attachment list TEST-1
bl issue attachment list TEST-1 --json
```

出力例:

```text
[1] screenshot.png (204800 bytes)
[2] log.txt (1024 bytes)
```

## `bl wiki list`

プロジェクトの Wiki ページを一覧表示します。

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

## `bl wiki show`

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

## `bl wiki create`

新しい Wiki ページを作成します。

```bash
bl wiki create --project-id 1 --name "Setup" --content "# Setup\nSee README."
bl wiki create --project-id 1 --name "Setup" --content "# Setup" --mail-notify --json
```

## `bl wiki update`

既存の Wiki ページを更新します。`--name` または `--content` のいずれかが必須です。

```bash
bl wiki update 12345 --content "# Updated content"
bl wiki update 12345 --name "New Title" --content "New content" --mail-notify
bl wiki update 12345 --name "Renamed" --json
```

## `bl wiki delete`

Wiki ページを削除します。

```bash
bl wiki delete 12345
bl wiki delete 12345 --mail-notify --json
```

## `bl wiki history`

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

## `bl wiki attachment list`

Wiki ページの添付ファイルを一覧表示します。

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

Backlog API v2 エンドポイントと `bl` コマンドの対応表です。

### Space

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl space` | `GET /api/v2/space` | ✅ 実装済み |
| `bl space activities` | `GET /api/v2/space/activities` | ✅ 実装済み |
| `bl space disk-usage` | `GET /api/v2/space/diskUsage` | ✅ 実装済み |
| `bl space notification` | `GET /api/v2/space/notification` | ✅ 実装済み |

### Projects

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

### Issues

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

### Pull Requests

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl pr list` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests` | 計画中 |
| `bl pr show <number>` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests/{number}` | 計画中 |
| `bl pr create` | `POST /api/v2/projects/{projectIdOrKey}/pullRequests` | 計画中 |
| `bl pr update <number>` | `PUT /api/v2/projects/{projectIdOrKey}/pullRequests/{number}` | 計画中 |
| `bl pr comment list <number>` | `GET /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments` | 計画中 |
| `bl pr comment add <number>` | `POST /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments` | 計画中 |
| `bl pr comment update <number> <comment-id>` | `PUT /api/v2/projects/{projectIdOrKey}/pullRequests/{number}/comments/{commentId}` | 計画中 |

### Git Repositories

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl git repo list` | `GET /api/v2/projects/{projectIdOrKey}/repositories` | 計画中 |
| `bl git repo show <repo>` | `GET /api/v2/projects/{projectIdOrKey}/repositories/{repoId}` | 計画中 |

### Users

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl auth status` | `GET /api/v2/users/myself` | ✅ 実装済み（内部） |
| `bl user list` | `GET /api/v2/users` | 計画中 |
| `bl user show <id>` | `GET /api/v2/users/{userId}` | 計画中 |
| `bl user activities <id>` | `GET /api/v2/users/{userId}/activities` | 計画中 |
| `bl user recently-viewed <id>` | `GET /api/v2/users/{userId}/recentlyViewedIssues` | 計画中 |

### Notifications

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl notification list` | `GET /api/v2/notifications` | 計画中 |
| `bl notification read <id>` | `PUT /api/v2/notifications/{notificationId}` | 計画中 |
| `bl notification read-all` | `DELETE /api/v2/notifications/unread` | 計画中 |

### Watching

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl watch list` | `GET /api/v2/watching` | 計画中 |
| `bl watch add` | `POST /api/v2/watching` | 計画中 |
| `bl watch delete <id>` | `DELETE /api/v2/watching/{watchingId}` | 計画中 |

### Teams

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl team list` | `GET /api/v2/teams` | 計画中 |
| `bl team show <id>` | `GET /api/v2/teams/{teamId}` | 計画中 |
