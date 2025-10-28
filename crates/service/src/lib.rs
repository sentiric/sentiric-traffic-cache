//! The Service Crate
//!
//! This crate implements the actual services that power the application.

use crate::cache::CacheManager;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// Modülleri tanımlıyoruz
pub mod cache;
pub mod certs;
pub mod config;
pub mod downloader;
pub mod proxy;

pub async fn run() -> Result<()> {
    // Tracing (logging) altyapısını kur
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Konfigürasyonu başlat
    config::init()?;
    let settings = config::get();
    info!("Configuration loaded successfully.");

    // Sertifika Otoritesini (CA) oluştur veya yükle
    let ca = Arc::new(certs::CertificateAuthority::new(&settings.certs.path)?);
    info!("Certificate Authority is ready.");

    // Cache Yöneticisini oluştur
    let cache_manager = Arc::new(CacheManager::new(&settings.cache.path)?);
    info!("Cache Manager is ready.");

    // Proxy sunucusunu başlat
    let proxy_addr: SocketAddr =
        format!("{}:{}", settings.proxy.bind_address, settings.proxy.port).parse()?;

    let proxy_task = tokio::spawn(proxy::run_server(proxy_addr, ca, cache_manager));

    info!("Service running. Press Ctrl+C to exit.");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received.");
        }
        res = proxy_task => {
            if let Err(e) = res? {
                tracing::error!("Proxy server exited with an error: {}", e);
            }
        }
    }

    Ok(())
}