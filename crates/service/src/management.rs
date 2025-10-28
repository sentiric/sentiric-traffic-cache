use anyhow::Result;
use std::net::SocketAddr;
use warp::Filter;
use serde::Serialize;
use tracing::info;

// Şimdilik basit bir Stats yapısı. Daha sonra CacheManager'dan gelen
// gerçek verilerle dolduracağız.
#[derive(Serialize)]
struct Stats {
    hits: u64,
    misses: u64,
}

/// Starts the management web server.
pub async fn run_server(addr: SocketAddr) -> Result<()> {
    info!("🚀 Management server listening on http://{}", addr);

    // GET /api/stats endpoint'i
    let stats_route = warp::path!("api" / "stats")
        .map(|| {
            let stats = Stats { hits: 10, misses: 5 };
            warp::reply::json(&stats)
        });
    
    // TODO: WebSocket /api/events endpoint'ini ekleyeceğiz.
    // TODO: Web arayüzünü sunacak olan static file server'ı ekleyeceğiz.

    let routes = stats_route;

    warp::serve(routes).run(addr).await;

    Ok(())
}