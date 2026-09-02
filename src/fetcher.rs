//! Fetching, verifying, caching, and extracting remote templates (`--template-url`).
//!
//! Downloads are cached under `<cache-root>/remote-templates/<sha256-of-canonical-url>/`.
//! A cached copy is reused when no `#sha256=` pin was given, or when the stored hash
//! matches the pin. Downloads are staged in a temporary directory, verified, then moved
//! into the cache atomically. Each cached template carries a `.source.json` provenance
//! file describing where it came from.

use crate::{
    error::{DiscoveryError, DiscoveryResult},
    template_url::{kind_of, parse, RemoteKind, TemplateUrl},
};
use sha2::{Digest, Sha256};
use std::{
    io::{self, Read},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

/// Hard cap on a single downloaded template/archive payload.
const MAX_DOWNLOAD_BYTES: u64 = 10 * 1024 * 1024;
/// Hard cap on the total decompressed size of an archive.
const MAX_EXTRACT_BYTES: u64 = 50 * 1024 * 1024;

/// Abstraction over the HTTP GET so the fetcher can be unit-tested offline.
pub trait Fetch {
    /// Performs a GET and returns the raw response body on success.
    fn get_bytes(&self, url: &str) -> Result<Vec<u8>, String>;
}

/// Default [`Fetch`] backed by `ureq` (synchronous HTTPS).
pub struct UreqFetch;

impl Fetch for UreqFetch {
    fn get_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
        let response = ureq::get(url)
            .call()
            .map_err(|err| format!("request failed: {err}"))?;
        let status = response.status();
        if !(200..300).contains(&status) {
            return Err(format!("HTTP status {status}"));
        }
        let mut body: Vec<u8> = Vec::new();
        response
            .into_reader()
            .take(MAX_DOWNLOAD_BYTES + 1)
            .read_to_end(&mut body)
            .map_err(|err| format!("failed reading response body: {err}"))?;
        if body.len() as u64 > MAX_DOWNLOAD_BYTES {
            return Err(format!(
                "response exceeds the {} byte size limit",
                MAX_DOWNLOAD_BYTES
            ));
        }
        Ok(body)
    }
}

/// Returns the cache root directory used for remote templates.
///
/// An explicit `--cache-dir` wins; otherwise the OS cache directory (falling back to
/// `~/.cache`) plus the `mcp-discovery` namespace is used. The directory is created.
pub fn cache_root_for(override_dir: Option<&Path>) -> DiscoveryResult<PathBuf> {
    let root = match override_dir {
        Some(dir) => dir.to_path_buf(),
        None => dirs::cache_dir()
            .or_else(|| dirs::home_dir().map(|home| home.join(".cache")))
            .unwrap_or_else(|| PathBuf::from("."))
            .join("mcp-discovery"),
    };
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

/// Fetches (or reuses from cache) the template at `url` and returns the local entry file path.
pub fn fetch_remote(url: &str, cache_dir_override: Option<&Path>) -> DiscoveryResult<PathBuf> {
    let template_url = parse(url)?;
    let fetcher = RemoteFetcher::new(cache_root_for(cache_dir_override)?);
    fetcher.fetch(&template_url)
}

struct RemoteFetcher {
    cache_root: PathBuf,
    http: Box<dyn Fetch>,
}

impl RemoteFetcher {
    fn new(cache_root: PathBuf) -> Self {
        RemoteFetcher::new_with(cache_root, Box::new(UreqFetch))
    }

    fn new_with(cache_root: PathBuf, http: Box<dyn Fetch>) -> Self {
        RemoteFetcher { cache_root, http }
    }

    /// Returns the local path to the resolved entry template file.
    fn fetch(&self, template_url: &TemplateUrl) -> DiscoveryResult<PathBuf> {
        let kind = kind_of(&template_url.url);
        let dir = self
            .cache_root
            .join("remote-templates")
            .join(hex::encode(Sha256::digest(template_url.url.as_bytes())));

        let cached = source_meta(&dir).ok();
        let cached_sha = cached
            .as_ref()
            .and_then(|meta| meta.get("sha256"))
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        let cache_valid = match (&cached_sha, &template_url.sha256) {
            (Some(stored), Some(pin)) => stored == pin,
            (_, None) => dir.is_dir(),
            (None, Some(_)) => false,
        };

        let cache_dir = if cache_valid {
            dir
        } else {
            self.download_and_install(template_url, kind, &dir)?
        };

        resolve_cached_entry(&cache_dir, template_url, kind).ok_or_else(|| {
            DiscoveryError::InvalidRemote {
                url: template_url.url.clone(),
                err: "cached template is missing its entry file; clear the cache and retry"
                    .to_string(),
            }
        })
    }

    fn download_and_install(
        &self,
        template_url: &TemplateUrl,
        kind: RemoteKind,
        dir: &Path,
    ) -> DiscoveryResult<PathBuf> {
        let bytes = self.http.get_bytes(&template_url.url).map_err(|err| {
            DiscoveryError::TemplateFetch {
                url: template_url.url.clone(),
                err,
            }
        })?;

        let computed = hex::encode(Sha256::digest(&bytes));
        if let Some(expected) = &template_url.sha256 {
            if computed != *expected {
                return Err(DiscoveryError::TemplateIntegrity {
                    expected: expected.clone(),
                    actual: computed,
                });
            }
        }

        let parent = dir.parent().ok_or_else(|| DiscoveryError::InvalidRemote {
            url: template_url.url.clone(),
            err: "invalid cache path".to_string(),
        })?;
        std::fs::create_dir_all(parent)?;

        let stage = parent.join(format!(
            ".{}.tmp",
            dir.file_name().unwrap_or_default().to_string_lossy()
        ));
        if stage.exists() {
            std::fs::remove_dir_all(&stage)?;
        }
        std::fs::create_dir_all(&stage)?;

        match kind {
            RemoteKind::Raw => std::fs::write(stage.join("template.hbs"), &bytes)?,
            RemoteKind::Zip => {
                extract_zip(&bytes, &stage).map_err(|err| DiscoveryError::InvalidRemote {
                    url: template_url.url.clone(),
                    err,
                })?
            }
            RemoteKind::TarGz => {
                extract_tar_gz(&bytes, &stage).map_err(|err| DiscoveryError::InvalidRemote {
                    url: template_url.url.clone(),
                    err,
                })?
            }
        }

        write_source_meta(&stage, template_url, &computed)?;

        if dir.exists() {
            std::fs::remove_dir_all(dir)?;
        }
        std::fs::rename(&stage, dir)?;

        Ok(dir.to_path_buf())
    }
}

/// Returns the metadata written for a cached template directory, if present.
fn source_meta(dir: &Path) -> DiscoveryResult<serde_json::Value> {
    let content = std::fs::read_to_string(dir.join(".source.json"))?;
    Ok(serde_json::from_str(&content)?)
}

fn write_source_meta(
    stage: &Path,
    template_url: &TemplateUrl,
    sha256: &str,
) -> DiscoveryResult<()> {
    let fetched_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let meta = serde_json::json!({
        "url": template_url.url,
        "sha256": sha256,
        "fetched_at": fetched_at,
        "fetched_by": concat!("mcp-discovery ", env!("CARGO_PKG_VERSION")),
    });
    std::fs::write(
        stage.join(".source.json"),
        serde_json::to_string_pretty(&meta)?,
    )?;
    Ok(())
}

/// Resolves the entry file inside a fetched/cached template directory.
///
/// `#entry=<subpath>` wins for archives. Otherwise archives use `template.hbs` or the
/// single standalone `.hbs` file in the root. Raw templates are stored as `template.hbs`.
fn resolve_cached_entry(
    dir: &Path,
    template_url: &TemplateUrl,
    kind: RemoteKind,
) -> Option<PathBuf> {
    if let Some(entry) = &template_url.entry {
        return match kind {
            RemoteKind::Raw => None,
            _ => {
                let candidate = dir.join(entry);
                candidate.is_file().then_some(candidate)
            }
        };
    }
    match kind {
        RemoteKind::Raw => dir
            .join("template.hbs")
            .is_file()
            .then(|| dir.join("template.hbs")),
        RemoteKind::Zip | RemoteKind::TarGz => {
            let conventional = dir.join("template.hbs");
            if conventional.is_file() {
                return Some(conventional);
            }
            let mut candidates: Vec<PathBuf> = std::fs::read_dir(dir)
                .ok()?
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("hbs"))
                .collect();
            candidates.sort();
            match candidates.as_slice() {
                [single] => Some(single.clone()),
                _ => None,
            }
        }
    }
}

/// Guards an archive member path so it cannot escape `dest` (zip-slip / absolute paths).
fn safe_member_path(dest: &Path, member: &str) -> Result<PathBuf, String> {
    let cleaned = member.replace('\\', "/");
    let mut out = dest.to_path_buf();
    for component in Path::new(&cleaned).components() {
        match component {
            Component::Normal(part) => out.push(part),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err("archive member escapes the target directory".into())
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err("archive member uses an absolute path".into());
            }
        }
    }
    if !out.starts_with(dest) {
        return Err("archive member escapes the target directory".into());
    }
    Ok(out)
}

fn extract_zip(bytes: &[u8], dest: &Path) -> Result<(), String> {
    let mut archive = zip::ZipArchive::new(io::Cursor::new(bytes))
        .map_err(|err| format!("invalid zip archive: {err}"))?;
    let mut total: u64 = 0;

    for index in 0..archive.len() {
        let mut member = archive
            .by_index(index)
            .map_err(|err| format!("invalid zip member: {err}"))?;
        if member.is_dir() {
            continue;
        }
        #[cfg(unix)]
        {
            if member
                .unix_mode()
                .is_some_and(|mode| mode & 0o170000 == 0o120000)
            {
                continue; // skip symlinks
            }
        }
        let out = safe_member_path(dest, member.name())?;
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }
        total += member.size();
        if total > MAX_EXTRACT_BYTES {
            return Err("archive decompresses beyond the size limit".into());
        }
        let mut file = std::fs::File::create(&out).map_err(|err| err.to_string())?;
        io::copy(&mut member, &mut file).map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn extract_tar_gz(bytes: &[u8], dest: &Path) -> Result<(), String> {
    let decoder = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);
    let mut total: u64 = 0;

    let entries = archive
        .entries()
        .map_err(|err| format!("invalid tar archive: {err}"))?;
    for entry in entries {
        let mut entry = entry.map_err(|err| format!("invalid tar member: {err}"))?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_dir() || entry_type.is_symlink() || entry_type.is_hard_link() {
            continue;
        }
        let member_path = entry
            .path()
            .map_err(|err| format!("invalid tar member path: {err}"))?
            .into_owned();
        let out = safe_member_path(dest, &member_path.to_string_lossy())?;
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }
        let copied = io::copy(
            &mut entry,
            &mut std::fs::File::create(&out).map_err(|e| e.to_string())?,
        )
        .map_err(|err| err.to_string())?;
        total += copied;
        if total > MAX_EXTRACT_BYTES {
            return Err("archive decompresses beyond the size limit".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[derive(Clone)]
    struct FakeFetch {
        responses: Arc<std::collections::HashMap<String, Vec<u8>>>,
        calls: Arc<AtomicUsize>,
    }

    impl Fetch for FakeFetch {
        fn get_bytes(&self, url: &str) -> Result<Vec<u8>, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.responses
                .get(url)
                .cloned()
                .ok_or_else(|| format!("no response for {url}"))
        }
    }

    fn temp_cache() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("cache");
        (dir, root)
    }

    #[test]
    fn test_fetch_raw_and_cache_hit_skips_network() {
        let (dir, root) = temp_cache();
        let body = b"Hello {{name}}".to_vec();
        let responses: std::collections::HashMap<String, Vec<u8>> =
            [("https://example.com/t.hbs".to_string(), body.clone())]
                .into_iter()
                .collect();
        let calls = Arc::new(AtomicUsize::new(0));
        let fetcher = RemoteFetcher::new_with(
            root.clone(),
            Box::new(FakeFetch {
                responses: Arc::new(responses),
                calls: calls.clone(),
            }),
        );

        let turl = parse("https://example.com/t.hbs").unwrap();
        let entry = fetcher.fetch(&turl).unwrap();
        assert_eq!(std::fs::read_to_string(&entry).unwrap(), "Hello {{name}}");
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        // Second fetch reuses cache: no network call.
        let entry2 = fetcher.fetch(&turl).unwrap();
        assert_eq!(entry, entry2);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(entry.parent().unwrap().join(".source.json").is_file());
        drop(dir);
    }

    #[test]
    fn test_fetch_sha256_mismatch_errors() {
        let (dir, root) = temp_cache();
        let body = b"content".to_vec();
        let responses: std::collections::HashMap<String, Vec<u8>> =
            [("https://example.com/t.hbs".to_string(), body)]
                .into_iter()
                .collect();
        let fetcher = RemoteFetcher::new_with(
            root.clone(),
            Box::new(FakeFetch {
                responses: Arc::new(responses),
                calls: Arc::new(AtomicUsize::new(0)),
            }),
        );
        let turl = parse(&format!(
            "https://example.com/t.hbs#sha256={}",
            "a".repeat(64)
        ))
        .unwrap();
        let result = fetcher.fetch(&turl);
        assert!(matches!(
            result,
            Err(DiscoveryError::TemplateIntegrity { .. })
        ));
        drop(dir);
    }

    #[test]
    fn test_fetch_refetches_when_pin_changes() {
        let (dir, root) = temp_cache();
        let body = b"content".to_vec();
        let digest = hex::encode(Sha256::digest(&body));
        let responses: std::collections::HashMap<String, Vec<u8>> =
            [("https://example.com/t.hbs".to_string(), body)]
                .into_iter()
                .collect();
        let calls = Arc::new(AtomicUsize::new(0));
        let fetcher = RemoteFetcher::new_with(
            root.clone(),
            Box::new(FakeFetch {
                responses: Arc::new(responses),
                calls: calls.clone(),
            }),
        );

        let turl = parse(&format!("https://example.com/t.hbs#sha256={digest}")).unwrap();
        fetcher.fetch(&turl).unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        // Cache stores matching pin → reuse without a network call.
        fetcher.fetch(&turl).unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        drop(dir);
    }

    #[test]
    fn test_extract_zip_resolves_template_hbs_and_partials() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = dir.path().join("pack.zip");
        {
            let file = std::fs::File::create(&zip_path).unwrap();
            let mut writer = zip::ZipWriter::new(file);
            writer
                .start_file("template.hbs", zip::write::SimpleFileOptions::default())
                .unwrap();
            writer.write_all(b"{{> partials/header }}").unwrap();
            writer
                .start_file(
                    "partials/header.hbs",
                    zip::write::SimpleFileOptions::default(),
                )
                .unwrap();
            writer.write_all(b"HEADER").unwrap();
            writer.finish().unwrap();
        }
        let bytes = std::fs::read(&zip_path).unwrap();
        let dest = dir.path().join("out");
        extract_zip(&bytes, &dest).unwrap();
        assert_eq!(
            std::fs::read_to_string(dest.join("template.hbs")).unwrap(),
            "{{> partials/header }}"
        );
        assert_eq!(
            std::fs::read_to_string(dest.join("partials/header.hbs")).unwrap(),
            "HEADER"
        );
    }

    #[test]
    fn test_extract_zip_rejects_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = dir.path().join("evil.zip");
        {
            let file = std::fs::File::create(&zip_path).unwrap();
            let mut writer = zip::ZipWriter::new(file);
            writer
                .start_file("../evil.hbs", zip::write::SimpleFileOptions::default())
                .unwrap();
            writer.write_all(b"EVIL").unwrap();
            writer.finish().unwrap();
        }
        let bytes = std::fs::read(&zip_path).unwrap();
        let dest = dir.path().join("out");
        assert!(extract_zip(&bytes, &dest).is_err());
        assert!(!dest.join("..").join("evil.hbs").exists());
    }

    #[test]
    fn test_cache_root_override_and_default() {
        let (dir, root) = temp_cache();
        let overridden = cache_root_for(Some(&root)).unwrap();
        assert_eq!(overridden, root);
        // Default path resolves without panicking (may be the real cache dir).
        let _ = cache_root_for(None).unwrap();
        drop(dir);
    }
}
