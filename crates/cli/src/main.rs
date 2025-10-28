//! The CLI Runner Crate
//!
//! This is the main entry point for the headless/server version of the
//! application.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // RUST_LOG=debug gibi environment variable'ları ayarlamak için
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    
    sentiric_service::run().await
}