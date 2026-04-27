use anyhow::{Context, Result};
use search_cache::SearchCache;
use search_cancel::CancellationToken;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crate::output::{FileResult, SearchOutput};

const IGNORE_PATH: &str = "/System/Volumes/Data";
static NEVER_STOPPED: AtomicBool = AtomicBool::new(false);

pub struct SearchRunner {
    cache: Option<SearchCache>,
}

impl SearchRunner {
    pub fn new(root: &Path, refresh: bool) -> Result<Self> {
        let ignore_paths = vec![PathBuf::from(IGNORE_PATH)];
        let cache_path = default_cache_path()?;

        let cache = if refresh {
            SearchCache::walk_fs_with_ignore(root, &ignore_paths)
        } else {
            SearchCache::try_read_persistent_cache(
                root,
                &cache_path,
                &ignore_paths,
                &NEVER_STOPPED,
            )
            .unwrap_or_else(|_| SearchCache::walk_fs_with_ignore(root, &ignore_paths))
        };

        Ok(Self {
            cache: Some(cache),
        })
    }

    pub fn search(
        &mut self,
        query: &str,
        limit: usize,
        _case_sensitive: bool,
    ) -> Result<SearchOutput> {
        let start = Instant::now();
        let cache = self
            .cache
            .as_mut()
            .context("cache not initialized")?;

        let opt_results = cache
            .query_files(query, CancellationToken::noop())
            .context("search query failed")?;

        let results = opt_results.unwrap_or_default();
        let total = results.len();
        let returned = total.min(limit);
        let file_results: Vec<FileResult> = results
            .into_iter()
            .take(limit)
            .map(FileResult::from_search_result)
            .collect();

        Ok(SearchOutput {
            results: file_results,
            total,
            returned,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    pub fn save_cache(self) -> Result<()> {
        let cache_path = default_cache_path()?;
        if let Some(cache) = self.cache {
            cache.flush_to_file(&cache_path).context("failed to write cache")?;
        }
        Ok(())
    }
}

pub fn build_index(root: &Path, refresh: bool) -> Result<SearchCache> {
    let ignore_paths = vec![PathBuf::from(IGNORE_PATH)];
    let cache_path = default_cache_path()?;

    if refresh {
        println!("Walking filesystem from {}...", root.display());
        Ok(SearchCache::walk_fs_with_ignore(root, &ignore_paths))
    } else {
        let cache =
            SearchCache::try_read_persistent_cache(root, &cache_path, &ignore_paths, &NEVER_STOPPED)
                .unwrap_or_else(|_| {
                    println!("Cache miss. Walking filesystem from {}...", root.display());
                    SearchCache::walk_fs_with_ignore(root, &ignore_paths)
                });
        Ok(cache)
    }
}

fn default_cache_path() -> Result<PathBuf> {
    if let Ok(custom) = std::env::var("CARDINAL_CACHE_PATH") {
        return Ok(PathBuf::from(custom));
    }
    let home = std::env::var("HOME").context("HOME not set")?;
    Ok(PathBuf::from(format!(
        "{home}/Library/Caches/cardinal/cache.zstd"
    )))
}
