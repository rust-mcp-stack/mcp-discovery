//! Authentication helpers for connecting to protected streamable HTTP MCP servers.

use std::collections::HashMap;

use rust_mcp_sdk::auth::{generate_pkce_params, McpAuthClient, McpAuthConfig};

use crate::{
    error::DiscoveryResult,
    types::{Grant, McpAuthOptions},
};

/// Parses `"Name: Value"` header entries into a map, splitting on the first `:`.
pub fn parse_headers(entries: &[String]) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for entry in entries {
        if let Some((name, value)) = entry.split_once(':') {
            let name = name.trim().to_string();
            let value = value.trim().to_string();
            if !name.is_empty() {
                headers.insert(name, value);
            }
        }
    }
    headers
}

/// Strategy for capturing the authorization `code` in the interactive PKCE flow.
///
/// `Manual` is implemented today (the user pastes the code / redirect URL into stdin).
/// `Loopback` is a reserved extension point for future automatic capture via a local
/// HTTP listener and is not yet implemented.
pub enum CodeCapture {
    Manual,
    Loopback(u16),
}

impl CodeCapture {
    /// Captures the authorization code according to the configured strategy.
    pub fn capture(self) -> std::io::Result<String> {
        match self {
            CodeCapture::Manual => capture_manually(),
            CodeCapture::Loopback(port) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("loopback code capture on port {port} is not implemented yet"),
            )),
        }
    }
}

/// Reads the authorization code from standard input.
///
/// The user may paste either the full redirect URL (including `?code=...`) or just the code.
fn capture_manually() -> std::io::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(extract_code_from_redirect(input.trim()))
}

/// Extracts the `code` query parameter from a redirect URL or returns the raw value.
fn extract_code_from_redirect(value: &str) -> String {
    if let Some(query_start) = value.find('?') {
        let query = &value[query_start + 1..];
        for pair in query.split('&') {
            if let Some((key, val)) = pair.split_once('=') {
                if key == "code" {
                    return val.to_string();
                }
            }
        }
    }
    value.to_string()
}

/// Resolves HTTP headers for the streamable HTTP transport.
///
/// Returns `None` when no static headers or OAuth flow is configured.
pub async fn resolve_headers(
    options: &McpAuthOptions,
    url: &str,
) -> DiscoveryResult<Option<HashMap<String, String>>> {
    let mut headers = parse_headers(&options.headers);

    let has_oauth = options.client_id.is_some()
        || options.client_secret.is_some()
        || options.redirect_uri.is_some()
        || options.grant != Grant::ClientCredentials;

    if !has_oauth {
        return if headers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(headers))
        };
    }

    let mut builder = McpAuthConfig::builder().server_url(url.to_string());
    if let Some(client_id) = &options.client_id {
        builder = builder.client_id(client_id.clone());
    }
    if let Some(client_secret) = &options.client_secret {
        builder = builder.client_secret(client_secret.clone());
    }
    if let Some(scope) = &options.scope {
        builder = builder.scope(scope.clone());
    }
    if let Some(redirect_uri) = &options.redirect_uri {
        builder = builder.redirect_uri(redirect_uri.clone());
    }
    let auth_client = builder.build()?;

    let oauth_headers = obtain_headers(&auth_client, options).await?;
    headers.extend(oauth_headers);

    Ok(Some(headers))
}

async fn obtain_headers(
    auth_client: &McpAuthClient,
    options: &McpAuthOptions,
) -> DiscoveryResult<HashMap<String, String>> {
    match options.grant {
        Grant::ClientCredentials => {
            if options.client_id.is_none() {
                auth_client.register().await?;
            }
            auth_client.authenticate().await?;
            Ok(auth_client.get_auth_headers().await?)
        }
        Grant::AuthorizationCode => {
            let redirect_uri = options
                .redirect_uri
                .clone()
                .ok_or_else(|| crate::error::DiscoveryError::InvalidSchema(
                    "--redirect-uri is required for the authorization-code grant".to_string(),
                ))?;
            tracing::debug!("using redirect_uri: {redirect_uri}");

            if options.client_id.is_none() {
                auth_client.register().await?;
            }

            let pkce = generate_pkce_params();
            let state = None;
            let scope = options.scope.as_deref();
            let auth_url = auth_client
                .build_authorization_url(&pkce, scope, state)
                .await?;

            println!("Open the following URL in your browser to authorize:");
            println!("{auth_url}");
            println!("Then paste the redirect URL (or just the code) below:");

            let code = CodeCapture::Manual.capture()?;
            auth_client
                .complete_authorization_code_flow(code, pkce.code_verifier)
                .await?;

            Ok(auth_client.get_auth_headers().await?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_headers() {
        let headers = parse_headers(&[
            "Authorization: Bearer abc123".to_string(),
            "X-Api-Key:  secret ".to_string(),
        ]);
        assert_eq!(headers.get("Authorization").unwrap(), "Bearer abc123");
        assert_eq!(headers.get("X-Api-Key").unwrap(), "secret");
    }

    #[test]
    fn test_parse_headers_empty() {
        assert!(parse_headers(&[]).is_empty());
    }

    #[test]
    fn test_extract_code_from_redirect() {
        assert_eq!(
            extract_code_from_redirect("http://127.0.0.1:8080/callback?code=abc&state=xyz"),
            "abc"
        );
        assert_eq!(extract_code_from_redirect("http://127.0.0.1:8080/callback?state=xyz"), "http://127.0.0.1:8080/callback?state=xyz");
        assert_eq!(extract_code_from_redirect("raw-code"), "raw-code");
    }
}
