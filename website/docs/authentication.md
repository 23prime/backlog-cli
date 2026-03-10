# Authentication

## Logging in with an API key

### Issuing an API key

1. Log in to your Backlog space
2. Go to **Personal settings** → **API**
3. Enter a memo and click **Submit**
4. Copy the generated API key

### Running the login command

```bash
bl auth login
```

You will be prompted for:

- **Space key** — the subdomain of your Backlog space.
  For `mycompany.backlog.com`, enter `mycompany`.
- **API key** — the key issued in the step above (input is hidden)

Running `bl auth login` again with a different space key adds another space.
The most recently logged-in space becomes the current (active) space.

## Logging in with OAuth 2.0

`bl` supports browser-based OAuth 2.0 login as an alternative to API keys.

### Step 1 — Register an OAuth application in Backlog

1. Open [https://backlog.com/developer/applications/oauth2Clients/add](https://backlog.com/developer/applications/oauth2Clients/add)
2. Create a new application:
   - **Application type**: Confidential Client
   - **Redirect URI**: `http://127.0.0.1:54321/callback`
     (use `http://127.0.0.1:<port>/callback` if you will pass `--port <port>`)
3. Note the **Client ID** and **Client Secret**

### Step 2 — Run the OAuth login command

```bash
bl auth login-oauth
```

You will be prompted for:

- **Space key** — the subdomain of your Backlog space
- **Client ID** — from the registered application
- **Client Secret** — from the registered application (input is hidden)

The command opens your browser to the Backlog authorization page.
After you approve, the browser is redirected to `http://127.0.0.1:54321/callback`
and the access token is stored automatically.

To use a custom port (must match the Redirect URI registered in Backlog):

```bash
bl auth login-oauth --port 8080
```

## Managing multiple spaces

```bash
# List all configured spaces (* marks the current space)
bl auth list

# Switch the current space
bl auth use another-company

# Use a different space for a single command
bl --space another-company project list

# Or set the BL_SPACE environment variable
export BL_SPACE=another-company
bl project list

# Inject credentials via environment variables (useful in CI/CD)
export BL_SPACE=mycompany
export BL_API_KEY=your-api-key
bl project list
```

## Checking auth status

```bash
bl auth status
```

This verifies your credentials against the Backlog API and shows:

```text
Space: mycompany.backlog.com
  - Auth method: API key
  - API key: abcd...
  - Stored in: System keyring
  - Logged in as Your Name (your-id)
```

When authenticated via OAuth:

```text
Space: mycompany.backlog.com
  - Auth method: OAuth 2.0
  - Client ID: abc123
  - Client Secret: abcd...
  - Access token: abcd...
  - Stored in: System keyring
  - Logged in as Your Name (your-id)
```

When using API key auth with `BL_API_KEY` set, `Stored in` shows `Environment variable` instead.

## Logging out

```bash
# Logout from the current space
bl auth logout

# Logout from a specific space
bl auth logout another-company

# Logout from all spaces and remove all config files (useful before uninstalling)
bl auth logout --all
```
