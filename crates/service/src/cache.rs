use anyhow::{Context, Result};
use bytes::Bytes;
use futures_util::StreamExt;
use hyper::body::{Body, Sender};
use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncWriteExt};
use tracing::{debug, info, instrument, warn};

/// Manages disk-based caching.
pub struct CacheManager {
    disk_path: PathBuf,
}

impl CacheManager {
    pub fn new(path: &str) -> Result<Self> {
        let disk_path = Path::new(path).to_path_buf();
        std::fs::create_dir_all(&disk_path).context("Failed to create cache directory")?;
        info!("Disk cache enabled at: {:?}", disk_path);
        Ok(Self { disk_path })
    }

    fn key_to_path(&self, key: &str) -> PathBuf {
        let hash = format!("{:x}", md5::compute(key));
        self.disk_path.join(hash)
    }

    /// Retrieves a response from the disk cache if it exists.
    #[instrument(skip(self))]
    pub async fn get(&self, key: &str) -> Option<Body> {
        let path = self.key_to_path(key);
        if path.exists() {
            debug!("CACHE HIT (disk): {}", key);
            let file = fs::File::open(path).await.ok()?;
            let stream = tokio_util::io::ReaderStream::new(file);
            return Some(Body::wrap_stream(stream));
        }
        debug!("CACHE MISS: {}", key);
        None
    }

    /// Caches a response body to disk while streaming it to the client.
    #[instrument(skip(self, body_stream))]
    pub async fn put_stream(&self, key: String, body_stream: Body) -> Result<Body> {
        let (mut tx, body_for_client) = Body::channel();
        let path = self.key_to_path(&key);

        tokio::spawn(async move {
            if let Err(e) =
                Self::stream_to_disk_and_client(body_stream, tx, path, key).await
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
    ) -> Result<()> {
        let mut file = fs::File::create(&path).await.context("Failed to create cache file")?;
        let mut total_bytes = 0;

        while let Some(chunk_result) = body_stream.next().await {
            let chunk = chunk_result.context("Error reading response stream")?;
            file.write_all(&chunk).await.context("Failed to write to cache file")?;
            total_bytes += chunk.len();
            
            let _ = tx.send_data(Bytes::from(chunk)).await;
        }

        info!("CACHE PUT: {} ({} bytes)", key, total_bytes);
        Ok(())
    }
}