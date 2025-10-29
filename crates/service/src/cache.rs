use anyhow::{Context, Result};
use sentiric_core::{Stats, CacheEntryInfo};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::{fs, io::AsyncWriteExt};
use tracing::{debug, info, instrument, warn};
use hyper::body::{Body, Sender};
use futures_util::StreamExt;

pub struct CacheManager {
    disk_path: PathBuf,
    pub stats: Arc<CacheStatsInternal>,
}

#[derive(Default)]
pub struct CacheStatsInternal {
    pub hits: AtomicU64,
    pub misses: AtomicU64,
    pub disk_items: AtomicU64,
    pub total_disk_size_bytes: AtomicU64,
    pub bytes_saved: AtomicU64, // <-- YENİ
}

impl CacheManager {
    pub fn new(path: &str) -> Result<Self> {
        let disk_path = Path::new(path).to_path_buf();
        std::fs::create_dir_all(&disk_path).context("Failed to create cache directory")?;
        info!("Disk cache enabled at: {:?}", disk_path);
        Ok(Self {
            disk_path,
            stats: Arc::new(CacheStatsInternal::default()),
        })
    }

    pub async fn get_stats(&self) -> Stats {
        let hits = self.stats.hits.load(Ordering::Relaxed);
        let misses = self.stats.misses.load(Ordering::Relaxed);
        Stats {
            hits,
            misses,
            total_requests: hits + misses,
            disk_items: self.stats.disk_items.load(Ordering::Relaxed),
            total_disk_size_bytes: self.stats.total_disk_size_bytes.load(Ordering::Relaxed),
            bytes_saved: self.stats.bytes_saved.load(Ordering::Relaxed), // <-- YENİ
        }
    }

    fn key_to_path(&self, key: &str) -> PathBuf {
        let hash = format!("{:x}", md5::compute(key));
        self.disk_path.join(hash)
    }

    #[instrument(skip(self))]
    pub async fn get(&self, key: &str) -> Option<Body> {
        let path = self.key_to_path(key);
        if path.exists() {
            debug!("CACHE HIT (disk): {}", key);
            self.stats.hits.fetch_add(1, Ordering::Relaxed);
            
            // Dosya boyutunu al ve bytes_saved'a ekle
            if let Ok(metadata) = fs::metadata(&path).await {
                self.stats.bytes_saved.fetch_add(metadata.len(), Ordering::Relaxed);
            }

            let file = fs::File::open(path).await.ok()?;
            let stream = tokio_util::io::ReaderStream::new(file);
            return Some(Body::wrap_stream(stream));
        }
        debug!("CACHE MISS: {}", key);
        None
    }
    
    // ... (put_stream ve diğer fonksiyonlar aynı kalıyor)
    #[instrument(skip(self, body_stream))]
    pub async fn put_stream(&self, key: String, body_stream: Body) -> Result<Body> {
        let (tx, body_for_client) = Body::channel();
        let path = self.key_to_path(&key);
        let stats_clone = self.stats.clone();
        let meta_path = path.with_extension("meta");
        tokio::spawn(async move {
            if let Err(e) = fs::write(meta_path, key.clone()).await {
                 warn!("Failed to write meta file for cache key {}: {}", key, e);
            }
            if let Err(e) =
                Self::stream_to_disk_and_client(body_stream, tx, path, key, stats_clone).await
            {
                warn!("Failed to cache response: {}", e);
            }
        });
        Ok(body_for_client)
    }
    
    async fn stream_to_disk_and_client(
        mut body_stream: Body,
        mut tx: Sender,
        path: PathBuf,
        key: String,
        stats: Arc<CacheStatsInternal>,
    ) -> Result<()> {
        let mut file = fs::File::create(&path).await.context("Failed to create cache file")?;
        let mut total_bytes = 0;
        while let Some(chunk_result) = body_stream.next().await {
            let chunk = chunk_result.context("Error reading response stream")?;
            file.write_all(&chunk).await.context("Failed to write to cache file")?;
            total_bytes += chunk.len() as u64;
            let _ = tx.send_data(chunk).await;
        }
        stats.disk_items.fetch_add(1, Ordering::Relaxed);
        stats.total_disk_size_bytes.fetch_add(total_bytes, Ordering::Relaxed);
        info!("CACHE PUT: {} ({} bytes)", key, total_bytes);
        Ok(())
    }

    pub async fn list_entries(&self) -> Result<Vec<CacheEntryInfo>> {
        let mut entries = Vec::new();
        let mut read_dir = fs::read_dir(&self.disk_path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some() && path.extension().unwrap() == "meta" { continue; }
            if path.is_file() {
                let metadata = entry.metadata().await?;
                let meta_path = path.with_extension("meta");
                let key = match fs::read_to_string(meta_path).await {
                    Ok(url) => url,
                    Err(_) => path.file_name().unwrap().to_string_lossy().to_string(),
                };
                entries.push(CacheEntryInfo { key, size_bytes: metadata.len(), });
            }
        }
        Ok(entries)
    }

    pub async fn clear_cache(&self) -> Result<()> {
        let mut read_dir = fs::read_dir(&self.disk_path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            if entry.file_type().await?.is_file() {
                fs::remove_file(entry.path()).await?;
            }
        }
        self.stats.disk_items.store(0, Ordering::Relaxed);
        self.stats.total_disk_size_bytes.store(0, Ordering::Relaxed);
        self.stats.bytes_saved.store(0, Ordering::Relaxed); // <-- YENİ
        info!("Cache cleared successfully.");
        Ok(())
    }
}