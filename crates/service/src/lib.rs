//! The Service Crate
//!
//! This crate implements the actual services that power the application.

use anyhow::Result;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{prelude::*, EnvFilter, FmtSubscriber};

// Modülleri tanımlıyoruz
pub mod certs;
pub mod config;

pub async fn run() -> Result<()> {
    // Tracing (logging) altyapısını kur
    // TODO: Bu daha sonra arayüze log gönderecek şekilde geliştirilecek
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Konfigürasyonu başlat
    config::init()?;
    let settings = config::get();
    info!("Configuration loaded successfully.");

    // Sertifika Otoritesini (CA) oluştur veya yükle
    let ca = Arc::new(certs::CertificateAuthority::new(&settings.certs.path)?);
    info!("Certificate Authority is ready.");

    println!("Service layer starting on port {}...", settings.proxy.port);
    
    // TODO: Proxy sunucusunu burada başlatacağız.

    println!("Service running. Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}