//! Parsing and classification of `--template-url` values.
//!
//! A template URL is an `https://` URL that may carry client-side directives in its
//! URL fragment (never sent to the server):
//! - `#sha256=<hex>` — integrity pin over the raw downloaded bytes.
//! - `#entry=<subpath>` — select a file inside an archive (`.zip`/`.tar.gz`) template.

use crate::error::{DiscoveryError, DiscoveryResult};
use url::Url;

/// A parsed `--template-url` value.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateUrl {
    /// Canonical URL without the fragment (the fetch key used for caching).
    pub url: String,
    /// Expected SHA-256 of the raw downloaded bytes, when pinned via `#sha256=<hex>`.
    pub sha256: Option<String>,
    /// File to select inside an archive, when given via `#entry=<subpath>`.
    pub entry: Option<String>,
}

/// The kind of content a template URL points at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RemoteKind {
    /// A single Handlebars template served as a raw file.
    Raw,
    /// A `.zip` archive.
    Zip,
    /// A `.tar.gz` archive.
    TarGz,
}

/// Classifies a canonical URL (without fragment) by its path extension.
pub fn kind_of(url: &str) -> RemoteKind {
    let path = Url::parse(url)
        .ok()
        .and_then(|u| {
            u.path_segments()
                .and_then(|mut s| s.next_back().map(str::to_owned))
        })
        .unwrap_or_default()
        .to_lowercase();
    if path.ends_with(".zip") {
        RemoteKind::Zip
    } else if path.ends_with(".tar.gz") || path.ends_with(".tgz") {
        RemoteKind::TarGz
    } else {
        RemoteKind::Raw
    }
}

/// Parses a raw `--template-url` argument into a [`TemplateUrl`].
///
/// Only `https://` URLs are accepted. Fragment keys other than `sha256`/`entry` are
/// logged as warnings and ignored.
pub fn parse(raw: &str) -> DiscoveryResult<TemplateUrl> {
    let parsed = Url::parse(raw).map_err(|err| DiscoveryError::InvalidRemote {
        url: raw.to_string(),
        err: format!("invalid URL: {err}"),
    })?;

    if parsed.scheme() != "https" {
        return Err(DiscoveryError::InvalidRemote {
            url: raw.to_string(),
            err: format!(
                "only https:// URLs are supported (got '{}://')",
                parsed.scheme()
            ),
        });
    }
    if parsed.host_str().map_or(true, |host| host.is_empty()) {
        return Err(DiscoveryError::InvalidRemote {
            url: raw.to_string(),
            err: "URL has no host".to_string(),
        });
    }

    let mut sha256: Option<String> = None;
    let mut entry: Option<String> = None;

    if let Some(fragment) = parsed.fragment() {
        for (key, value) in url::form_urlencoded::parse(fragment.as_bytes()) {
            match key.as_ref() {
                "sha256" => {
                    let value = value.into_owned().to_ascii_lowercase();
                    if value.len() != 64 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
                        return Err(DiscoveryError::InvalidRemote {
                            url: raw.to_string(),
                            err: "'sha256' fragment must be a 64-character hex digest".to_string(),
                        });
                    }
                    sha256 = Some(value);
                }
                "entry" => entry = Some(value.into_owned()),
                other => tracing::warn!(
                    "Ignoring unknown --template-url fragment key '{other}' in '{raw}'"
                ),
            }
        }
    }

    let mut canonical = parsed;
    canonical.set_fragment(None);

    Ok(TemplateUrl {
        url: canonical.to_string(),
        sha256,
        entry,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex64(c: char) -> String {
        c.to_string().repeat(64)
    }

    #[test]
    fn test_parse_sha256() {
        let digest = hex64('a');
        let turl = parse(&format!("https://example.com/t.hbs#sha256={digest}")).unwrap();
        assert_eq!(turl.url, "https://example.com/t.hbs");
        assert_eq!(turl.sha256.as_deref(), Some(digest.as_str()));
        assert_eq!(turl.entry, None);
    }

    #[test]
    fn test_parse_entry_and_sha256() {
        let digest = hex64('b');
        let turl = parse(&format!(
            "https://example.com/pack.zip#sha256={digest}&entry=sub/report.hbs"
        ))
        .unwrap();
        assert_eq!(turl.sha256.as_deref(), Some(digest.as_str()));
        assert_eq!(turl.entry.as_deref(), Some("sub/report.hbs"));
    }

    #[test]
    fn test_parse_no_fragment() {
        let turl = parse("https://example.com/t.hbs").unwrap();
        assert_eq!(turl.url, "https://example.com/t.hbs");
        assert_eq!(turl.sha256, None);
        assert_eq!(turl.entry, None);
    }

    #[test]
    fn test_parse_rejects_invalid_sha256_length() {
        let result = parse("https://example.com/t.hbs#sha256=abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rejects_non_https() {
        assert!(parse("http://example.com/t.hbs").is_err());
        assert!(parse("ftp://example.com/t.hbs").is_err());
    }

    #[test]
    fn test_parse_rejects_missing_host() {
        assert!(parse("https://").is_err());
    }

    #[test]
    fn test_kind_of() {
        assert_eq!(kind_of("https://example.com/t.hbs"), RemoteKind::Raw);
        assert_eq!(kind_of("https://example.com/pack.zip"), RemoteKind::Zip);
        assert_eq!(
            kind_of("https://example.com/pack.tar.gz"),
            RemoteKind::TarGz
        );
        assert_eq!(kind_of("https://example.com/pack.tgz"), RemoteKind::TarGz);
    }
}
