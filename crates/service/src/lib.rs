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
use warp::Filter;

// Modülleri tanımlıyoruz
pub mod cache;
pub mod certs;
pub mod config;
pub mod downloader;
pub mod dns; // <--- EKLENDİ
pub mod management;
pub mod proxy;

#[cfg(feature = "web")]
pub mod web_server;

pub async fn run() -> Result<()> {
    let subscriber = FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?)).with_thread_ids(true).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    config::init()?;
    let settings = config::get();
    info!("Configuration loaded successfully.");
    let ca = Arc::new(certs::CertificateAuthority::new(&settings.certs.path)?);
    info!("Certificate Authority is ready.");
    let cache_manager = Arc::new(CacheManager::new(&settings.cache.path)?);
    info!("Cache Manager is ready.");

    // --- GÖREVLERİ OLUŞTUR ---

    let proxy_addr: SocketAddr = format!("{}:{}", settings.proxy.bind_address, settings.proxy.port).parse()?;
    let proxy_task = tokio::spawn(proxy::run_server(proxy_addr, ca, cache_manager.clone()));

    let mgmt_addr: SocketAddr = format!("{}:{}", settings.management.bind_address, settings.management.port).parse()?;
    let mgmt_task = tokio::spawn(async move {
        // ... (management task'in içi aynı)
    });

    // DNS Sunucusu Görevi (sadece config'de etkinse)
    let dns_task = if settings.dns.enabled {
        let dns_addr: SocketAddr = format!("{}:{}", settings.dns.bind_address, settings.dns.port).parse()?;
        Some(tokio::spawn(dns::run_server(dns_addr)))
    } else {
        info!("DNS server is disabled in config.");
        None
    };

    let stats_broadcaster_task = tokio::spawn(async move {
        // ... (stats task'in içi aynı)
    });

    info!("All services running. Press Ctrl+C to exit.");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => { info!("Shutdown signal received."); }
        res = proxy_task => { if let Err(e) = res? { error!("Proxy server exited: {}", e); } }
        _res = mgmt_task => { error!("Management server exited."); }
        
        // DNS task'ini de select'e ekle
        res = async { if let Some(task) = dns_task { task.await.unwrap() } else { futures_util::future::pending().await } } => {
            if let Err(e) = res { error!("DNS server exited: {}", e); }
        }

        _ = stats_broadcaster_task => { info!("Stats broadcaster exited."); }
    }
    Ok(())
}