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
pub mod dns;
pub mod management;
pub mod proxy;

// DÜZELTME: web_server modülünü 'web' feature'ı arkasına geri koyuyoruz.
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
    let mgmt_cache_clone = cache_manager.clone();
    let mgmt_task = tokio::spawn(async move {
        info!("🚀 Management server listening on http://{}", mgmt_addr);
        let api = management::api_routes(mgmt_cache_clone);

        #[cfg(feature = "web")]
        {
            use warp::Filter;
            let routes = api.or(web_server::static_files());
            warp::serve(routes).run(mgmt_addr).await;
        }

        #[cfg(not(feature = "web"))]
        {
            warp::serve(api).run(mgmt_addr).await;
        }
    });

    let dns_task = if settings.dns.enabled {
        let dns_addr: SocketAddr = format!("{}:{}", settings.dns.bind_address, settings.dns.port).parse()?;
        Some(tokio::spawn(dns::run_server(dns_addr)))
    } else {
        info!("DNS server is disabled in config.");
        None
    };

    let stats_broadcaster_task = tokio::spawn(async move {
        loop {
            let stats = cache_manager.get_stats().await;
            let _ = EVENT_BROADCASTER.send(management::WsEvent::StatsUpdated(stats));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    info!("All services running. Press Ctrl+C to exit.");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => { info!("Shutdown signal received."); }
        res = proxy_task => { if let Err(e) = res? { error!("Proxy server exited: {}", e); } }
        _res = mgmt_task => { error!("Management server exited."); }
        res = async { if let Some(task) = dns_task { task.await.unwrap() } else { futures_util::future::pending().await } } => {
            if let Err(e) = res { error!("DNS server exited: {}", e); }
        }
        _ = stats_broadcaster_task => { info!("Stats broadcaster exited."); }
    }
    Ok(())
}