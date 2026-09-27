use std::{cell::RefCell, collections::HashMap, time::Duration};

use anyhow::Context;
use crates_index::{http, Crate, SparseIndex};

pub struct Index {
    inner: SparseIndex,
    http: reqwest::blocking::Client,
    /// None reads only Cargo's cache; Some refreshes each queried crate once.
    fetched_crates: RefCell<Option<HashMap<String, Option<Crate>>>>,
}

impl Index {
    /// Open Cargo's sparse cache without fetching anything.
    pub fn new_cargo_default() -> anyhow::Result<Index> {
        Ok(Index {
            // crates-index defaults to the cache layout from before Cargo 1.85.
            inner: SparseIndex::from_url_with_hash_kind(crates_index::sparse::URL, &crates_index::HashKind::Stable)?,
            http: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()?,
            fetched_crates: RefCell::new(None),
        })
    }

    /// Refresh each queried crate over HTTP on its next lookup.
    pub fn update(&self) {
        *self.fetched_crates.borrow_mut() = Some(HashMap::new());
    }

    pub fn crate_(&self, name: &str) -> anyhow::Result<Option<Crate>> {
        let mut fetched_crates = self.fetched_crates.borrow_mut();
        let Some(crates) = fetched_crates.as_mut() else {
            return Ok(self.inner.crate_from_cache(name).ok());
        };
        match crates.entry(name.to_owned()) {
            std::collections::hash_map::Entry::Occupied(entry) => Ok(entry.get().clone()),
            std::collections::hash_map::Entry::Vacant(entry) => Ok(entry.insert(self.fetch_crate(name)?).clone()),
        }
    }

    /// Always contact the index, including when polling for a newly published version.
    pub fn fetch_crate(&self, name: &str) -> anyhow::Result<Option<Crate>> {
        let request = self
            .inner
            .make_cache_request(name)?
            // Let reqwest negotiate HTTP/2 instead of requiring it.
            .version(http::Version::HTTP_11)
            .body(Vec::<u8>::new())?
            .try_into()?;
        let response = self
            .http
            .execute(request)
            .with_context(|| format!("Failed to query the sparse index for '{name}'"))?;
        let mut cached_response = http::Response::new(Vec::new());
        *cached_response.status_mut() = response.status();
        *cached_response.version_mut() = response.version();
        *cached_response.headers_mut() = response.headers().clone();
        *cached_response.body_mut() = response.bytes()?.to_vec();
        self.inner
            .parse_cache_response(name, cached_response, true)
            .with_context(|| format!("Failed to read the sparse index entry for '{name}'"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRY: &str = r#"{"name":"sparse-test","vers":"1.2.3","deps":[],"cksum":"0000000000000000000000000000000000000000000000000000000000000000","features":{},"yanked":false}"#;

    #[test]
    fn reads_cargo_sparse_cache_without_a_git_index() -> gix_testtools::Result {
        if gix_testtools::run_in_isolated_process()? {
            return Ok(());
        }

        let cargo_home = gix_testtools::tempfile::tempdir()?;
        // Only this test runs in the child, so Cargo's environment can be changed safely.
        std::env::set_var("CARGO_HOME", cargo_home.path());
        std::env::set_var("CARGO_REGISTRIES_CRATES_IO_PROTOCOL", "git");
        let (index_path, _) = crates_index::local_path_and_canonical_url_with_hash_kind(
            crates_index::sparse::URL,
            Some(cargo_home.path()),
            &crates_index::HashKind::Stable,
        )?;
        let cache_path = index_path.join(".cache/sp/ar/sparse-test");
        std::fs::create_dir_all(cache_path.parent().unwrap())?;
        std::fs::write(
            cache_path,
            [
                b"\x03\x02\0\0\0etag: \"test\"\0".as_slice(),
                b"1.2.3\0",
                ENTRY.as_bytes(),
                b"\0",
            ]
            .concat(),
        )?;

        assert_eq!(
            Index::new_cargo_default()?
                .crate_("sparse-test")?
                .map(|krate| krate.highest_version().version().to_owned()),
            Some("1.2.3".into())
        );
        Ok(())
    }

    #[test]
    fn sparse_http_refresh_and_polling() -> anyhow::Result<()> {
        use std::io::{BufRead, BufReader, Write};

        let cargo_home = gix_testtools::tempfile::tempdir()?;
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let index = Index {
            inner: SparseIndex::with_path(cargo_home.path(), format!("sparse+http://{}/", listener.local_addr()?))?,
            http: reqwest::blocking::Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(5))
                .build()?,
            fetched_crates: RefCell::new(None),
        };
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let server = std::thread::spawn(move || -> anyhow::Result<()> {
            let published = ENTRY.replace("sparse-test", "missing").replace("1.2.3", "1.2.4");
            for (path, status, expected_etag, body) in [
                ("/sp/ar/sparse-test", "302 Found", None, "redirect body"),
                ("/redirected", "200 OK", None, ENTRY),
                ("/sp/ar/sparse-test", "304 Not Modified", Some("\"test\""), ""),
                ("/mi/ss/missing", "404 Not Found", None, ""),
                ("/sp/ar/sparse-test", "503 Service Unavailable", Some("\"test\""), ""),
                ("/sp/ar/sparse-test", "200 OK", Some("\"test\""), "invalid json"),
                ("/mi/ss/missing", "200 OK", None, published.as_str()),
            ] {
                let (mut stream, _) = listener.accept()?;
                stream.set_read_timeout(Some(Duration::from_secs(5)))?;
                let mut reader = BufReader::new(&stream);
                let mut line = String::new();
                reader.read_line(&mut line)?;
                assert_eq!(line, format!("GET {path} HTTP/1.1\r\n"));
                let mut etag = None;
                loop {
                    line.clear();
                    assert_ne!(reader.read_line(&mut line)?, 0, "request headers must be complete");
                    if line == "\r\n" {
                        break;
                    }
                    if let Some((name, value)) = line.split_once(':') {
                        if name.eq_ignore_ascii_case("if-none-match") {
                            etag = Some(value.trim().to_owned());
                        }
                    }
                }
                assert_eq!(etag.as_deref(), expected_etag);
                let headers = if status == "302 Found" {
                    "Location: /redirected\r\nETag: \"redirect\"\r\n"
                } else {
                    "ETag: \"test\"\r\n"
                };
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\n{headers}Connection: close\r\n\r\n{body}",
                    body.len()
                )?;
            }
            done_tx.send(())?;
            Ok(())
        });

        assert!(index.crate_("sparse-test")?.is_none(), "cached lookups don't fetch");
        index.update();
        for _ in 0..2 {
            assert_eq!(
                index.crate_("sparse-test")?.unwrap().highest_version().version(),
                "1.2.3"
            );
        }
        assert_eq!(
            index.inner.crate_from_cache("sparse-test")?.highest_version().version(),
            "1.2.3"
        );
        index.update();
        assert_eq!(
            index.crate_("sparse-test")?.unwrap().highest_version().version(),
            "1.2.3"
        );
        for _ in 0..2 {
            assert!(index.crate_("missing")?.is_none(), "404s are cached for this traversal");
        }
        let err = index.fetch_crate("sparse-test").unwrap_err();
        assert!(
            format!("{err:#}").contains("503"),
            "server errors must not use stale data"
        );
        assert!(index.fetch_crate("sparse-test").is_err(), "malformed entries must fail");
        assert_eq!(
            index.fetch_crate("missing")?.unwrap().highest_version().version(),
            "1.2.4"
        );

        done_rx.recv_timeout(Duration::from_secs(5))?;
        server.join().expect("HTTP server must not panic")?;
        Ok(())
    }
}
