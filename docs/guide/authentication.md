# Authentication

`mcp-discovery` can connect to protected MCP servers over the **Streamable HTTP** transport. Authentication is selected with flags that are only valid together with `--url` (auth does not apply to stdio servers).

## Flags

| Flag | Description |
| --- | --- |
| `--header "Name: Value"` | Static HTTP header, repeatable. Applied to every request (e.g. a bearer token or API key). |
| `--client-id <ID>` | Pre-registered OAuth client id. When omitted, Dynamic Client Registration (DCR) is used. |
| `--client-secret <SECRET>` | Pre-registered OAuth client secret. |
| `--scope <SCOPE>` | OAuth scope(s) to request, e.g. `"mcp tools"`. |
| `--redirect-uri <URI>` | Redirect URI used by the `authorization-code` grant (required for it). |
| `--grant <GRANT>` | OAuth grant: `client-credentials` (default) or `authorization-code`. |

> All authentication flags require `--url`. `mcp-discovery` also validates this at runtime and errors clearly if authentication options are set without a URL.

## 1. No authentication

```bash
mcp-discovery print --url https://gateway.mcpservers.org/yahoo-finance/mcp
```

## 2. Static bearer token / API key

When the server accepts a pre-issued token, add it as a header. No OAuth handshake is performed.

```bash
mcp-discovery print --url https://mcp.example.com/mcp \
  --header "Authorization: Bearer <token>"
```

You can repeat `--header` to send multiple headers:

```bash
mcp-discovery print --url https://mcp.example.com/mcp \
  --header "Authorization: Bearer <token>" \
  --header "X-Api-Key: <key>"
```

## 3. OAuth `client_credentials` (machine-to-machine)

For pre-registered clients (no user interaction, the default grant):

```bash
mcp-discovery print --url https://mcp.example.com/mcp \
  --grant client-credentials \
  --client-id my-client \
  --client-secret my-secret \
  --scope "mcp tools"
```

`mcp-discovery` performs OAuth metadata discovery, token exchange, and attaches the `Authorization: Bearer <token>` header automatically. The token is refreshed automatically if it expires during the run.

## 4. OAuth `client_credentials` with Dynamic Client Registration (DCR)

If the server supports [RFC 7591 Dynamic Client Registration](https://datatracker.ietf.org/doc/html/rfc7591) and you do **not** have a pre-registered `client_id`, omit `--client-id`/`--client-secret`:

```bash
mcp-discovery print --url https://mcp.example.com/mcp \
  --grant client-credentials \
  --scope "mcp tools"
```

`mcp-discovery` will discover the registration endpoint, register the client, then exchange credentials for a token.

## 5. OAuth `authorization_code` + PKCE (interactive)

For user-facing flows (e.g. login via a browser). This is interactive — `mcp-discovery` prints an authorization URL for you to open, then captures the returned code:

```bash
mcp-discovery print --url https://mcp.example.com/mcp \
  --grant authorization-code \
  --client-id my-client \
  --redirect-uri http://127.0.0.1:8080/callback \
  --scope "mcp tools"
```

`mcp-discovery` will:

1. Generate a PKCE code verifier/challenge.
2. Print the authorization URL.
3. Wait for you to paste the redirect URL (or just the `?code=...` value) back into the terminal.
4. Exchange the code (with PKCE) for a token and attach the `Authorization: Bearer <token>` header.

> **Note:** The code is currently captured by manual paste. Automatic loopback capture is a planned extension.
