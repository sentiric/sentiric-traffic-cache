//! The Service Crate
//!
//! This crate implements the actual services that power the application.

use crate::cache::CacheManager;
use crate::management::EVENT_BROADCASTER;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// Modülleri tanımlıyoruz
pub mod cache;
pub mod certs;
pub mod config;
pub mod downloader;
pub mod management;
pub mod proxy;

pub async fn run() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    config::init()?;
    let settings = config::get();
    info!("Configuration loaded successfully.");

    let ca = Arc::new(certs::CertificateAuthority::new(&settings.certs.path)?);
    info!("Certificate Authority is ready.");

    let cache_manager = Arc::new(CacheManager::new(&settings.cache.path)?);
    info!("Cache Manager is ready.");

    // --- GÖREVLERİ OLUŞTUR ---

    let proxy_addr: SocketAddr =
        format!("{}:{}", settings.proxy.bind_address, settings.proxy.port).parse()?;
    let proxy_task = tokio::spawn(proxy::run_server(proxy_addr, ca, cache_manager.clone()));

    let mgmt_addr: SocketAddr =
        format!("{}:{}", settings.management.bind_address, settings.management.port).parse()?;
    let mgmt_task = tokio::spawn(management::run_server(mgmt_addr, cache_manager.clone()));

    // İstatistikleri periyodik olarak yayınlayan görev
    let stats_broadcaster_task = tokio::spawn(async move {
        loop {
            let stats = cache_manager.get_stats().await;
            let _ = EVENT_BROADCASTER.send(management::WsEvent::StatsUpdated(stats));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    info!("All services running. Press Ctrl+C to exit.");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received.");
        }
        res = proxy_task => { if let Err(e) = res? { error!("Proxy server exited: {}", e); } }
        res = mgmt_task => { if let Err(e) = res? { error!("Management server exited: {}", e); } }
        _ = stats_broadcaster_task => { info!("Stats broadcaster exited."); }
    }

    Ok(())
}