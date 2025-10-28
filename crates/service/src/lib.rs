//! The Service Crate
//!
//! This crate implements the actual services that power the application.

use anyhow::Result;

// Modülleri tanımlıyoruz
pub mod config;

pub async fn run() -> Result<()> {
    // Konfigürasyonu başlat
    config::init()?;
    let settings = config::get();

    println!("Service layer starting on port {}...", settings.proxy.port);
    
    // TODO: Proxy sunucusunu burada başlatacağız.

    println!("Service running. Press Ctrl+C to exit.");
    // Sunucunun sonsuza dek çalışmasını sağlamak için
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}