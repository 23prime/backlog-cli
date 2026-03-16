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
bl space activities --count 50 --order asc
bl space activities --activity-type-id 1 --activity-type-id 2 --min-id 100 --json
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

## `bl space licence`

Backlog スペースのライセンス情報を表示します。

```bash
bl space licence
bl space licence --json
```

出力例:

```text
Contract:  premium
Storage:   5242880 / 1073741824 bytes
Start:     2020-01-01
```

## `bl space update-notification`

Backlog スペースの通知メッセージを更新します。

```bash
bl space update-notification --content "メンテナンスのお知らせ"
bl space update-notification --content "Hello." --json
```

出力例:

```text
Updated: 2024-07-01T00:00:00Z

メンテナンスのお知らせ
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
bl project activities <id-or-key> --count 20 --order asc
bl project activities <id-or-key> --activity-type-id 1 --activity-type-id 2
bl project activities <id-or-key> --min-id 100 --max-id 200
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

## `bl project create`

新しいプロジェクトを作成します。

```bash
bl project create --name "My Project" --key MYPRJ
bl project create --name "My Project" --key MYPRJ --chart-enabled --subtasking-enabled
bl project create --name "My Project" --key MYPRJ --text-formatting-rule backlog --json
```

| フラグ | デフォルト | 説明 |
| --- | --- | --- |
| `--name` | — | プロジェクト名（必須） |
| `--key` | — | プロジェクトキー：大文字・数字・アンダースコア、2〜10文字（必須） |
| `--chart-enabled` | `false` | バーンダウンチャート機能を有効化 |
| `--subtasking-enabled` | `false` | サブタスク機能を有効化 |
| `--text-formatting-rule` | `markdown` | `backlog` または `markdown` |
| `--json` | — | JSON 形式で出力 |

出力例:

```text
ID:         1
Key:        MYPRJ
Name:       My Project
Formatting: markdown
Archived:   false
```

## `bl project update`

既存のプロジェクトを更新します。少なくとも 1 つのフィールドを指定する必要があります。

```bash
bl project update <id-or-key> --name "New Name"
bl project update <id-or-key> --key NEWKEY --chart-enabled true
bl project update <id-or-key> --archived true --json
```

| フラグ | 説明 |
| --- | --- |
| `--name` | 新しいプロジェクト名 |
| `--key` | 新しいプロジェクトキー |
| `--chart-enabled` | `true` または `false` |
| `--subtasking-enabled` | `true` または `false` |
| `--text-formatting-rule` | `backlog` または `markdown` |
| `--archived` | `true` または `false` |
| `--json` | JSON 形式で出力 |

出力例:

```text
ID:         1
Key:        MYPRJ
Name:       New Name
Formatting: markdown
Archived:   false
```

## `bl project delete`

プロジェクトを完全に削除します。

```bash
bl project delete <id-or-key>
bl project delete <id-or-key> --json
```

出力例:

```text
Deleted: My Project (MYPRJ)
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

## `bl issue comment count`

課題のコメント数を取得します。

```bash
bl issue comment count TEST-1
bl issue comment count TEST-1 --json
```

## `bl issue comment show`

特定のコメントを表示します。

```bash
bl issue comment show TEST-1 42
bl issue comment show TEST-1 42 --json
```

## `bl issue comment notification list`

コメントの通知一覧を取得します。

```bash
bl issue comment notification list TEST-1 42
bl issue comment notification list TEST-1 42 --json
```

## `bl issue comment notification add`

コメントに通知を追加します。

```bash
bl issue comment notification add TEST-1 42 --notified-user-id 1
bl issue comment notification add TEST-1 42 --notified-user-id 1 --notified-user-id 2 --json
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

## `bl issue attachment get`

課題の添付ファイルをダウンロードします。

```bash
bl issue attachment get TEST-1 1
bl issue attachment get TEST-1 1 --output ./downloaded.png
```

`--output` で指定したパス、または現在のディレクトリに元のファイル名で保存します。

出力例:

```text
Saved: screenshot.png (204800 bytes)
```

## `bl issue attachment delete`

課題の添付ファイルを削除します。

```bash
bl issue attachment delete TEST-1 1
bl issue attachment delete TEST-1 1 --json
```

## `bl issue participant list`

課題の参加者一覧を取得します。

```bash
bl issue participant list TEST-1
bl issue participant list TEST-1 --json
```

## `bl issue shared-file list`

課題にリンクされた共有ファイルの一覧を取得します。

```bash
bl issue shared-file list TEST-1
bl issue shared-file list TEST-1 --json
```

## `bl issue shared-file link`

課題に共有ファイルをリンクします。

```bash
bl issue shared-file link TEST-1 --shared-file-id 1
bl issue shared-file link TEST-1 --shared-file-id 1 --shared-file-id 2 --json
```

## `bl issue shared-file unlink`

課題から共有ファイルのリンクを解除します。

```bash
bl issue shared-file unlink TEST-1 1
bl issue shared-file unlink TEST-1 1 --json
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

## `bl team list`

スペース内のチームを一覧表示します。

```bash
bl team list
bl team list --json
bl team list --count 20 --order asc
bl team list --offset 10
```

出力例:

```text
[1] dev-team (3 members)
[2] design-team (2 members)
```

## `bl team show`

特定のチームの詳細を表示します。
スペースの設定によっては `403 Forbidden` が返ることがあります。

```bash
bl team show <id>
bl team show <id> --json
```

出力例:

```text
ID:      1
Name:    dev-team
Created: 2024-01-01T00:00:00Z
Updated: 2024-06-01T00:00:00Z
Members:
    [2] Developer
    [3] Engineer
```

## `bl user activities`

特定のユーザーの最近のアクティビティを表示します。

```bash
bl user activities <id>
bl user activities <id> --json
bl user activities <id> --count 20 --order asc
bl user activities <id> --activity-type-id 1 --activity-type-id 2
bl user activities <id> --min-id 100 --max-id 200
```

出力例:

```text
[3153] type=2 project=SUB user=admin created=2024-06-01T00:00:00Z
```

## `bl user recently-viewed`

認証ユーザーが最近閲覧した課題を表示します。

```bash
bl user recently-viewed
bl user recently-viewed --json
bl user recently-viewed --count 20 --order asc
bl user recently-viewed --offset 10
```

出力例:

```text
[BLG-1] Fix login (Open, -)
[BLG-2] Add dark mode (In Progress, John Doe)
```

## `bl user add`

新しいユーザーを追加します。スペース管理者権限が必要です。

```bash
bl user add --user-id john --password secret --name "John Doe" --mail-address john@example.com --role-type normal
bl user add --user-id john --password secret --name "John Doe" --mail-address john@example.com --role-type normal --json
```

ロールタイプ: `administrator`, `normal`, `reporter`, `viewer`, `guest-reporter`, `guest-viewer`

出力例:

```text
Added: john (John Doe) [roleType: 2]
```

## `bl user update`

既存のユーザーを更新します。スペース管理者権限が必要です。

```bash
bl user update <id> --name "New Name"
bl user update <id> --mail-address new@example.com --role-type viewer --json
```

出力例:

```text
Updated: john (New Name) [roleType: 4]
```

## `bl user delete`

ユーザーを削除します。スペース管理者権限が必要です。

```bash
bl user delete <id>
bl user delete <id> --json
```

出力例:

```text
Deleted: john (John Doe)
```

## `bl user recently-viewed-projects`

認証ユーザーが最近閲覧したプロジェクトを表示します。

```bash
bl user recently-viewed-projects
bl user recently-viewed-projects --count 50 --offset 20 --order asc
bl user recently-viewed-projects --json
```

出力例:

```text
[MYPRJ] My Project
[TEST] Test Project
```

## `bl user recently-viewed-wikis`

認証ユーザーが最近閲覧したWikiページを表示します。

```bash
bl user recently-viewed-wikis
bl user recently-viewed-wikis --count 50 --offset 20 --order asc
bl user recently-viewed-wikis --json
```

出力例:

```text
[1] Home (project: 1)
[2] API Reference (project: 2)
```

## `bl user star list`

ユーザーがスターをつけた一覧を表示します。

```bash
bl user star list <id>
bl user star list <id> --count 50 --order asc --json
bl user star list <id> --min-id 100 --max-id 200 --json
```

出力例:

```text
[1] Fix login bug
[2] Add dark mode feature
```

## `bl user star count`

ユーザーがつけたスターの数を表示します。

```bash
bl user star count <id>
bl user star count <id> --since 2024-01-01 --until 2024-12-31 --json
```

出力例:

```text
42
```

## `bl user list`

スペース内のユーザーを一覧表示します。
Space Administrator 権限が必要です。権限がない場合は `403 Forbidden` が返ります。

```bash
bl user list
bl user list --json
```

出力例:

```text
[john] John Doe
[jane] Jane Smith
[12345] Bot User
```

## `bl user show`

数値 ID でユーザーの詳細を表示します。

```bash
bl user show <id>
bl user show <id> --json
```

出力例:

```text
ID:           123
User ID:      john
Name:         John Doe
Mail:         john@example.com
Role:         1
Lang:         ja
Last login:   2024-06-01T00:00:00Z
```

## `bl notification list`

認証ユーザーの通知一覧を表示します。

```bash
bl notification list
bl notification list --json
bl notification list --count 20 --order asc
bl notification list --unread
bl notification list --sender-id 123
```

出力例:

```text
[101] reason=2 project=TEST issue=TEST-1 read=false created=2024-06-01T00:00:00Z
[102] reason=6 project=TEST issue=TEST-2 read=true  created=2024-06-02T00:00:00Z
```

## `bl notification count`

認証ユーザーの未読通知数を表示します。

```bash
bl notification count
bl notification count --json
```

出力例:

```text
3
```

## `bl notification read <id>`

指定した通知を既読にします。

```bash
bl notification read <id>
```

## `bl notification reset-unread`

未読通知数をリセットします。
個々の通知が既読になるわけではなく、未読カウンターがリセットされます。

```bash
bl notification reset-unread
```

出力例:

```text
Unread count reset.
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
| `bl space licence` | `GET /api/v2/space/licence` | ✅ 実装済み |
| `bl space update-notification` | `PUT /api/v2/space/notification` | ✅ 実装済み |
| — | `GET /api/v2/space/image` | 計画中 |
| — | `POST /api/v2/space/attachment` | 計画中 |

### Projects

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl project list` | `GET /api/v2/projects` | ✅ 実装済み |
| `bl project show <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}` | ✅ 実装済み |
| `bl project create` | `POST /api/v2/projects` | ✅ 実装済み |
| `bl project update <id-or-key>` | `PATCH /api/v2/projects/{projectIdOrKey}` | ✅ 実装済み |
| `bl project delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}` | ✅ 実装済み |
| `bl project activities <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/activities` | ✅ 実装済み |
| `bl project disk-usage <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/diskUsage` | ✅ 実装済み |
| `bl project user list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/users` | ✅ 実装済み |
| `bl project user add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/users` | 計画中 |
| `bl project user delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/users` | 計画中 |
| `bl project admin list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/administrators` | 計画中 |
| `bl project admin add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/administrators` | 計画中 |
| `bl project admin delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/administrators` | 計画中 |
| — | `GET /api/v2/projects/{projectIdOrKey}/image` | 計画中 |
| `bl project status list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/statuses` | ✅ 実装済み |
| `bl project status add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/statuses` | 計画中 |
| `bl project status update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/statuses/{id}` | 計画中 |
| `bl project status delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/statuses/{id}` | 計画中 |
| `bl project status reorder <id-or-key>` | `PATCH /api/v2/projects/{projectIdOrKey}/statuses/updateDisplayOrder` | 計画中 |
| `bl project issue-type list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/issueTypes` | ✅ 実装済み |
| `bl project issue-type add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/issueTypes` | 計画中 |
| `bl project issue-type update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` | 計画中 |
| `bl project issue-type delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/issueTypes/{id}` | 計画中 |
| `bl project category list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/categories` | ✅ 実装済み |
| `bl project category add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/categories` | 計画中 |
| `bl project category update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/categories/{id}` | 計画中 |
| `bl project category delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/categories/{id}` | 計画中 |
| `bl project version list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/versions` | ✅ 実装済み |
| `bl project version add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/versions` | 計画中 |
| `bl project version update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/versions/{id}` | 計画中 |
| `bl project version delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/versions/{id}` | 計画中 |
| `bl project custom-field list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/customFields` | 計画中 |
| `bl project custom-field add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/customFields` | 計画中 |
| `bl project custom-field update <id-or-key> <id>` | `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}` | 計画中 |
| `bl project custom-field delete <id-or-key> <id>` | `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}` | 計画中 |
| `bl project custom-field item add <id-or-key> <id>` | `POST /api/v2/projects/{projectIdOrKey}/customFields/{id}/items` | 計画中 |
| `bl project custom-field item update <id-or-key> <id> <item-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` | 計画中 |
| `bl project custom-field item delete <id-or-key> <id> <item-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/customFields/{id}/items/{itemId}` | 計画中 |
| `bl project webhook list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/webhooks` | 計画中 |
| `bl project webhook show <id-or-key> <webhook-id>` | `GET /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | 計画中 |
| `bl project webhook add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/webhooks` | 計画中 |
| `bl project webhook update <id-or-key> <webhook-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | 計画中 |
| `bl project webhook delete <id-or-key> <webhook-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/webhooks/{webhookId}` | 計画中 |
| `bl project team list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/teams` | 計画中 |
| `bl project team add <id-or-key>` | `POST /api/v2/projects/{projectIdOrKey}/teams` | 計画中 |
| `bl project team delete <id-or-key>` | `DELETE /api/v2/projects/{projectIdOrKey}/teams` | 計画中 |

### Priorities & Resolutions

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl priority list` | `GET /api/v2/priorities` | 計画中 |
| `bl resolution list` | `GET /api/v2/resolutions` | 計画中 |

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
| `bl issue comment count <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/comments/count` | ✅ 実装済み |
| `bl issue comment show <id-or-key> <comment-id>` | `GET /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ 実装済み |
| `bl issue comment update <id-or-key> <comment-id>` | `PATCH /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ 実装済み |
| `bl issue comment delete <id-or-key> <comment-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/comments/{commentId}` | ✅ 実装済み |
| `bl issue comment notification list <id-or-key> <comment-id>` | `GET /api/v2/issues/{issueIdOrKey}/comments/{commentId}/notifications` | ✅ 実装済み |
| `bl issue comment notification add <id-or-key> <comment-id>` | `POST /api/v2/issues/{issueIdOrKey}/comments/{commentId}/notifications` | ✅ 実装済み |
| `bl issue attachment list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/attachments` | ✅ 実装済み |
| `bl issue attachment get <id-or-key> <attachment-id>` | `GET /api/v2/issues/{issueIdOrKey}/attachments/{attachmentId}` | ✅ 実装済み |
| `bl issue attachment delete <id-or-key> <attachment-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/attachments/{attachmentId}` | ✅ 実装済み |
| `bl issue participant list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/participants` | ✅ 実装済み |
| `bl issue shared-file list <id-or-key>` | `GET /api/v2/issues/{issueIdOrKey}/sharedFiles` | ✅ 実装済み |
| `bl issue shared-file link <id-or-key>` | `POST /api/v2/issues/{issueIdOrKey}/sharedFiles` | ✅ 実装済み |
| `bl issue shared-file unlink <id-or-key> <shared-file-id>` | `DELETE /api/v2/issues/{issueIdOrKey}/sharedFiles/{id}` | ✅ 実装済み |

### Documents

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl document list` | `GET /api/v2/documents` | 計画中 |
| `bl document tree` | `GET /api/v2/documents/tree` | 計画中 |
| `bl document show <id>` | `GET /api/v2/documents/{documentId}` | 計画中 |
| `bl document create` | `POST /api/v2/documents` | 計画中 |
| `bl document delete <id>` | `DELETE /api/v2/documents/{documentId}` | 計画中 |
| `bl document attachment get <id> <attachment-id>` | `GET /api/v2/documents/{documentId}/attachments/{attachmentId}` | 計画中 |

### Wiki

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl wiki list` | `GET /api/v2/wikis` | ✅ 実装済み |
| `bl wiki count` | `GET /api/v2/wikis/count` | 計画中 |
| `bl wiki tag list` | `GET /api/v2/wikis/tags` | 計画中 |
| `bl wiki show <id>` | `GET /api/v2/wikis/{wikiId}` | ✅ 実装済み |
| `bl wiki create` | `POST /api/v2/wikis` | ✅ 実装済み |
| `bl wiki update <id>` | `PATCH /api/v2/wikis/{wikiId}` | ✅ 実装済み |
| `bl wiki delete <id>` | `DELETE /api/v2/wikis/{wikiId}` | ✅ 実装済み |
| `bl wiki history <id>` | `GET /api/v2/wikis/{wikiId}/history` | ✅ 実装済み |
| `bl wiki star list <id>` | `GET /api/v2/wikis/{wikiId}/stars` | 計画中 |
| `bl wiki attachment list <id>` | `GET /api/v2/wikis/{wikiId}/attachments` | ✅ 実装済み |
| `bl wiki attachment add <id>` | `POST /api/v2/wikis/{wikiId}/attachments` | 計画中 |
| `bl wiki attachment get <id> <attachment-id>` | `GET /api/v2/wikis/{wikiId}/attachments/{attachmentId}` | 計画中 |
| `bl wiki attachment delete <id> <attachment-id>` | `DELETE /api/v2/wikis/{wikiId}/attachments/{attachmentId}` | 計画中 |
| `bl wiki shared-file list <id>` | `GET /api/v2/wikis/{wikiId}/sharedFiles` | 計画中 |
| `bl wiki shared-file link <id>` | `POST /api/v2/wikis/{wikiId}/sharedFiles` | 計画中 |
| `bl wiki shared-file unlink <id> <shared-file-id>` | `DELETE /api/v2/wikis/{wikiId}/sharedFiles/{id}` | 計画中 |

### Shared Files

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl shared-file list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/files/metadata/{path}` | 計画中 |
| `bl shared-file get <id-or-key> <id>` | `GET /api/v2/projects/{projectIdOrKey}/files/{sharedFileId}` | 計画中 |

### Stars

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl star add` | `POST /api/v2/stars` | 計画中 |
| `bl star delete <id>` | `DELETE /api/v2/stars/{starId}` | 計画中 |
| `bl user star list <id>` | `GET /api/v2/users/{userId}/stars` | ✅ 実装済み |
| `bl user star count <id>` | `GET /api/v2/users/{userId}/stars/count` | ✅ 実装済み |

### Pull Requests

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl pr list <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` | 計画中 |
| `bl pr count <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/count` | 計画中 |
| `bl pr show <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` | 計画中 |
| `bl pr create <id-or-key> <repo>` | `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests` | 計画中 |
| `bl pr update <id-or-key> <repo> <number>` | `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}` | 計画中 |
| `bl pr comment list <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` | 計画中 |
| `bl pr comment count <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/count` | 計画中 |
| `bl pr comment add <id-or-key> <repo> <number>` | `POST /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments` | 計画中 |
| `bl pr comment update <id-or-key> <repo> <number> <comment-id>` | `PATCH /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/comments/{commentId}` | 計画中 |
| `bl pr attachment list <id-or-key> <repo> <number>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments` | 計画中 |
| `bl pr attachment get <id-or-key> <repo> <number> <attachment-id>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments/{attachmentId}` | 計画中 |
| `bl pr attachment delete <id-or-key> <repo> <number> <attachment-id>` | `DELETE /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}/pullRequests/{number}/attachments/{attachmentId}` | 計画中 |

### Git Repositories

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl git repo list <id-or-key>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories` | 計画中 |
| `bl git repo show <id-or-key> <repo>` | `GET /api/v2/projects/{projectIdOrKey}/git/repositories/{repoIdOrName}` | 計画中 |

### Users

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl auth status` | `GET /api/v2/users/myself` | ✅ 実装済み（内部） |
| `bl user list` | `GET /api/v2/users` | ✅ 実装済み |
| `bl user show <id>` | `GET /api/v2/users/{userId}` | ✅ 実装済み |
| `bl user add` | `POST /api/v2/users` | ✅ 実装済み |
| `bl user update <id>` | `PATCH /api/v2/users/{userId}` | ✅ 実装済み |
| `bl user delete <id>` | `DELETE /api/v2/users/{userId}` | ✅ 実装済み |
| `bl user activities <id>` | `GET /api/v2/users/{userId}/activities` | ✅ 実装済み |
| `bl user recently-viewed` | `GET /api/v2/users/myself/recentlyViewedIssues` | ✅ 実装済み |
| `bl user recently-viewed-projects` | `GET /api/v2/users/myself/recentlyViewedProjects` | ✅ 実装済み |
| `bl user recently-viewed-wikis` | `GET /api/v2/users/myself/recentlyViewedWikis` | ✅ 実装済み |
| `bl user star list <id>` | `GET /api/v2/users/{userId}/stars` | ✅ 実装済み |
| `bl user star count <id>` | `GET /api/v2/users/{userId}/stars/count` | ✅ 実装済み |
| — | `GET /api/v2/users/{userId}/icon` | 計画中 |

### Notifications

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl notification list` | `GET /api/v2/notifications` | ✅ 実装済み |
| `bl notification count` | `GET /api/v2/notifications/count` | ✅ 実装済み |
| `bl notification read <id>` | `POST /api/v2/notifications/{notificationId}/markAsRead` | ✅ 実装済み |
| `bl notification reset-unread` | `POST /api/v2/notifications/markAsRead` | ✅ 実装済み |

### Watching

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl watch list` | `GET /api/v2/users/{userId}/watchings` | 計画中 |
| `bl watch count` | `GET /api/v2/users/{userId}/watchings/count` | 計画中 |
| `bl watch show <id>` | `GET /api/v2/watchings/{watchingId}` | 計画中 |
| `bl watch add` | `POST /api/v2/watchings` | 計画中 |
| `bl watch update <id>` | `PATCH /api/v2/watchings/{watchingId}` | 計画中 |
| `bl watch delete <id>` | `DELETE /api/v2/watchings/{watchingId}` | 計画中 |
| `bl watch read <id>` | `POST /api/v2/watchings/{watchingId}/markAsRead` | 計画中 |

### Teams

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl team list` | `GET /api/v2/teams` | ✅ 実装済み |
| `bl team show <id>` | `GET /api/v2/teams/{teamId}` | ✅ 実装済み |
| `bl team add` | `POST /api/v2/teams` | 計画中 |
| `bl team update <id>` | `PATCH /api/v2/teams/{teamId}` | 計画中 |
| `bl team delete <id>` | `DELETE /api/v2/teams/{teamId}` | 計画中 |
| — | `GET /api/v2/teams/{teamId}/icon` | 計画中 |

### System

| コマンド | API エンドポイント | 状態 |
| --- | --- | --- |
| `bl rate-limit` | `GET /api/v2/rateLimit` | 計画中 |
