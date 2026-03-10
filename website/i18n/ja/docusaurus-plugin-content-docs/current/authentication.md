# 認証

## API キーでログイン

### API キーの発行

1. Backlog スペースにログイン
2. **個人設定** → **API** へ移動
3. メモを入力して **Submit** をクリック
4. 生成された API キーをコピー

### ログインコマンドの実行

```bash
bl auth login
```

以下の入力を求められます。

- **スペースキー** — Backlog スペースのサブドメイン。
  `mycompany.backlog.com` の場合は `mycompany` を入力
- **API キー** — 上記で発行したキー（入力は非表示）

別のスペースキーで `bl auth login` を再実行すると、そのスペースが追加されます。
最後にログインしたスペースがカレント（アクティブ）スペースになります。

## OAuth 2.0 でログイン

API キーの代替として、ブラウザ経由の OAuth 2.0 ログインを利用できます。

### ステップ 1 — Backlog に OAuth アプリケーションを登録

1. [https://backlog.com/developer/applications/oauth2Clients/add](https://backlog.com/developer/applications/oauth2Clients/add) を開く
2. 新しいアプリケーションを作成します。
   - **Application type**: Confidential Client
   - **Redirect URI**: `http://127.0.0.1:54321/callback`
     （`--port <port>` を指定する場合は `http://127.0.0.1:<port>/callback`）
3. **Client ID** と **Client Secret** を控えておく

### ステップ 2 — OAuth ログインコマンドの実行

```bash
bl auth login-oauth
```

以下の入力を求められます。

- **スペースキー** — Backlog スペースのサブドメイン
- **Client ID** — 登録したアプリケーションのもの
- **Client Secret** — 登録したアプリケーションのもの（入力は非表示）

コマンドがブラウザで Backlog の認可ページを開きます。
承認後、ブラウザは `http://127.0.0.1:54321/callback` にリダイレクトされ、アクセストークンが自動保存されます。

カスタムポートを使用する場合（Backlog に登録した Redirect URI と一致させる必要があります）:

```bash
bl auth login-oauth --port 8080
```

## 複数スペースの管理

```bash
# 設定済みスペースを一覧表示（* がカレントスペース）
bl auth list

# カレントスペースを切り替え
bl auth use another-company

# 1 コマンドだけ別のスペースを使用
bl --space another-company project list

# または BL_SPACE 環境変数で指定
export BL_SPACE=another-company
bl project list

# 環境変数で認証情報を注入（CI/CD で便利）
export BL_SPACE=mycompany
export BL_API_KEY=your-api-key
bl project list
```

## 認証状態の確認

```bash
bl auth status
```

Backlog API に対して認証情報を検証し、以下を表示します。

```text
Space: mycompany.backlog.com
  - Auth method: API key
  - API key: abcd...
  - Stored in: System keyring
  - Logged in as Your Name (your-id)
```

OAuth 認証の場合:

```text
Space: mycompany.backlog.com
  - Auth method: OAuth 2.0
  - Client ID: abc123
  - Client Secret: abcd...
  - Access token: abcd...
  - Stored in: System keyring
  - Logged in as Your Name (your-id)
```

API キー認証で `BL_API_KEY` が設定されている場合、`Stored in` は `Environment variable` と表示されます。

## ログアウト

```bash
# カレントスペースからログアウト
bl auth logout

# 特定のスペースからログアウト
bl auth logout another-company

# すべてのスペースからログアウトして設定ファイルをすべて削除
bl auth logout --all
```
