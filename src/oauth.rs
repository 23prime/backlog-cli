use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

pub const DEFAULT_OAUTH_PORT: u16 = 54321;

/// Tokens and client credentials for OAuth 2.0 authentication.
#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl std::fmt::Debug for OAuthTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthTokens")
            .field("client_id", &self.client_id)
            .field("client_secret", &"<redacted>")
            .field("access_token", &"<redacted>")
            .field("refresh_token", &"<redacted>")
            .finish()
    }
}

/// Response body from Backlog's `/api/v2/oauth2/token` endpoint.
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

/// Run the full OAuth 2.0 authorization code flow.
///
/// 1. Bind the local callback listener first (fail fast on port conflicts).
/// 2. Open the browser to the Backlog authorization page.
/// 3. Wait for the callback and exchange the code for tokens.
pub fn run_oauth_flow(
    space_key: &str,
    client_id: &str,
    client_secret: &str,
    port: u16,
) -> Result<OAuthTokens> {
    // Bind before opening the browser so a port conflict is caught immediately.
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).with_context(|| {
        format!(
            "Failed to bind to port {port}. \
             Is the port already in use? Try a different port with --port."
        )
    })?;

    let redirect_uri = format!("http://127.0.0.1:{port}/callback");
    let state = generate_state()?;
    let auth_url = format!(
        "https://{space_key}.backlog.com/OAuth2AccessRequest.action\
         ?response_type=code\
         &client_id={client_id}\
         &redirect_uri={}\
         &state={state}",
        percent_encode(&redirect_uri),
    );

    anstream::eprintln!(
        "Opening browser for authorization...\n\
         If the browser does not open, visit:\n  {auth_url}"
    );
    let _ = open::that(&auth_url);

    anstream::eprintln!(
        "Waiting for authorization at http://127.0.0.1:{port}/callback (Ctrl+C to cancel)..."
    );
    let code = wait_for_callback(listener, &state)?;

    let tokens = exchange_code(space_key, client_id, client_secret, &code, &redirect_uri)?;
    Ok(tokens)
}

/// Exchange an authorization code for access and refresh tokens.
pub fn exchange_code(
    space_key: &str,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<OAuthTokens> {
    let token_url = format!("https://{space_key}.backlog.com/api/v2/oauth2/token");
    let client = Client::builder()
        .build()
        .context("Failed to build HTTP client")?;

    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = client
        .post(&token_url)
        .form(&params)
        .send()
        .context("Failed to request OAuth token")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        anyhow::bail!("OAuth token request failed ({}): {}", status, body);
    }

    let token_resp: TokenResponse = response.json().context("Failed to parse token response")?;
    Ok(OAuthTokens {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
    })
}

/// Use a refresh token to obtain a new access token.
pub fn refresh_access_token(space_key: &str, tokens: &OAuthTokens) -> Result<OAuthTokens> {
    let token_url = format!("https://{space_key}.backlog.com/api/v2/oauth2/token");
    let client = Client::builder()
        .build()
        .context("Failed to build HTTP client")?;

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", tokens.refresh_token.as_str()),
        ("client_id", tokens.client_id.as_str()),
        ("client_secret", tokens.client_secret.as_str()),
    ];

    let response = client
        .post(&token_url)
        .form(&params)
        .send()
        .context("Failed to refresh OAuth token")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        anyhow::bail!("OAuth token refresh failed ({}): {}", status, body);
    }

    let token_resp: TokenResponse = response.json().context("Failed to parse token response")?;
    Ok(OAuthTokens {
        client_id: tokens.client_id.clone(),
        client_secret: tokens.client_secret.clone(),
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
    })
}

/// Block on `listener` until the OAuth callback arrives.
/// Returns the authorization code after verifying the state parameter.
///
/// Non-callback requests (e.g. browser favicon fetches) are answered with a
/// minimal 200 response and then ignored so that the loop continues waiting.
fn wait_for_callback(listener: TcpListener, expected_state: &str) -> Result<String> {
    // Accept connections in a loop until a valid OAuth callback arrives.
    // This handles cases where the browser also issues a favicon request.
    const MAX_ATTEMPTS: usize = 10;
    for _ in 0..MAX_ATTEMPTS {
        let (stream, _) = listener
            .accept()
            .context("Failed to accept OAuth callback")?;
        let mut writer = stream.try_clone().context("Failed to clone TCP stream")?;
        let reader = BufReader::new(&stream);

        // Read the request line: "GET /callback?code=...&state=... HTTP/1.1"
        let request_line = match reader.lines().next() {
            Some(Ok(line)) => line,
            _ => continue,
        };

        // Second token is the path+query string.
        let path = match request_line.split_whitespace().nth(1) {
            Some(p) => p.to_string(),
            None => continue,
        };

        // Skip requests that are not the OAuth callback (e.g. /favicon.ico).
        if !path.starts_with("/callback") {
            send_html_response(&mut writer, 200, "");
            continue;
        }

        let (code, state) = parse_callback_params(&path)?;

        if state != expected_state {
            send_html_response(
                &mut writer,
                400,
                "<h1>Authorization failed</h1><p>State mismatch. Please try again.</p>",
            );
            anyhow::bail!("OAuth state mismatch — possible CSRF attempt");
        }

        send_html_response(
            &mut writer,
            200,
            "<h1>Authorization successful!</h1>\
             <p>You can close this tab and return to the terminal.</p>",
        );

        return Ok(code);
    }

    anyhow::bail!("OAuth callback not received after {MAX_ATTEMPTS} connection attempts")
}

fn parse_callback_params(path: &str) -> Result<(String, String)> {
    let query = path.split_once('?').map(|(_, q)| q).unwrap_or("");

    let mut code = None;
    let mut state = None;

    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            match k {
                "code" => code = Some(percent_decode(v)),
                "state" => state = Some(percent_decode(v)),
                _ => {}
            }
        }
    }

    let code = code.context("OAuth callback: 'code' parameter missing")?;
    let state = state.context("OAuth callback: 'state' parameter missing")?;
    Ok((code, state))
}

fn send_html_response(stream: &mut impl Write, status: u16, body: &str) {
    let reason = match status {
        200 => "OK",
        _ => "Bad Request",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n\
         {}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

/// Generate an opaque state value for CSRF protection using OS CSPRNG.
fn generate_state() -> Result<String> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes)
        .map_err(|e| anyhow::anyhow!("Failed to generate random state: {e}"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn percent_decode(s: &str) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            if let Ok(byte) = u8::from_str_radix(&format!("{h1}{h2}"), 16) {
                bytes.push(byte);
            }
        } else {
            let mut buf = [0u8; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encode_plain_ascii() {
        assert_eq!(percent_encode("hello"), "hello");
    }

    #[test]
    fn percent_encode_special_chars() {
        let encoded = percent_encode("http://localhost:54321/callback");
        assert!(encoded.contains("%3A")); // ':'
        assert!(encoded.contains("%2F")); // '/'
    }

    #[test]
    fn percent_decode_roundtrip() {
        let original = "http://localhost:54321/callback";
        assert_eq!(percent_decode(&percent_encode(original)), original);
    }

    #[test]
    fn parse_callback_params_success() {
        let (code, state) = parse_callback_params("/callback?code=abc123&state=deadbeef").unwrap();
        assert_eq!(code, "abc123");
        assert_eq!(state, "deadbeef");
    }

    #[test]
    fn parse_callback_params_missing_code() {
        assert!(parse_callback_params("/callback?state=deadbeef").is_err());
    }

    #[test]
    fn parse_callback_params_missing_state() {
        assert!(parse_callback_params("/callback?code=abc123").is_err());
    }

    #[test]
    fn generate_state_is_nonempty() {
        let s = generate_state().unwrap();
        assert!(!s.is_empty());
    }

    #[test]
    fn generate_state_two_calls_differ() {
        let s1 = generate_state().unwrap();
        let s2 = generate_state().unwrap();
        assert!(!s1.is_empty());
        assert!(!s2.is_empty());
        assert_ne!(s1, s2);
    }
}
