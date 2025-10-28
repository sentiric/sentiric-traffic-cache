use anyhow::Result;
use std::net::SocketAddr;
use warp::Filter;
use serde::Serialize;
use tracing::info;

// DÜZELTME: Serialize özelliğini ekledik.
#[derive(Serialize)] 
struct Stats {
    hits: u64,
    misses: u64,
}

/// Starts the management web server.
pub async fn run_server(addr: SocketAddr) -> Result<()> {
    info!("🚀 Management server listening on http://{}", addr);

    let stats_route = warp::path!("api" / "stats")
        .map(|| {
            let stats = Stats { hits: 10, misses: 5 };
            warp::reply::json(&stats)
        });
    
    let routes = stats_route;

    warp::serve(routes).run(addr).await;

    Ok(())
}