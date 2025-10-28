use crate::cache::CacheManager;
use anyhow::Result;
use futures_util::{StreamExt, SinkExt};
use sentiric_core::Stats;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
use warp::ws::{Message, WebSocket};
use warp::Filter;
use serde::Serialize;
use tracing::info;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum WsEvent {
    StatsUpdated(Stats),
}

// Tüm sisteme yayın yapacak olan global kanal
lazy_static::lazy_static! {
    pub static ref EVENT_BROADCASTER: Sender<WsEvent> = broadcast::channel(128).0;
}

/// Starts the management web server.
pub async fn run_server(addr: SocketAddr, cache: Arc<CacheManager>) -> Result<()> {
    info!("🚀 Management server listening on http://{}", addr);

    // Warp filter'larında kullanılmak üzere klonla
    let cache_filter = warp::any().map(move || cache.clone());

    // GET /api/stats endpoint'i
    let stats_route = warp::path!("api" / "stats")
        .and(cache_filter.clone())
        .and_then(handle_stats);

    // WS /api/events endpoint'i
    let events_route = warp::path!("api" / "events")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_websocket_connection)
        });

    let routes = stats_route.or(events_route);

    warp::serve(routes).run(addr).await;

    Ok(())
}

// /api/stats isteğini işler
async fn handle_stats(cache: Arc<CacheManager>) -> Result<impl warp::Reply, warp::Rejection> {
    let stats = cache.get_stats().await;
    Ok(warp::reply::json(&stats))
}

// Yeni bir WebSocket bağlantısını işler
async fn handle_websocket_connection(websocket: WebSocket) {
    info!("New WebSocket client connected");
    let (mut client_tx, _) = websocket.split();
    let mut rx = EVENT_BROADCASTER.subscribe();

    // Bu bağlantı için ayrı bir görev (task) başlat
    tokio::spawn(async move {
        // Kanalda yeni bir event olduğunda...
        while let Ok(event) = rx.recv().await {
            // Event'i JSON string'ine çevir
            if let Ok(json) = serde_json::to_string(&event) {
                // Ve WebSocket üzerinden istemciye gönder
                if client_tx.send(Message::text(json)).await.is_err() {
                    // Hata olursa (istemci bağlantıyı kapatmışsa), döngüyü kır
                    break;
                }
            }
        }
        info!("WebSocket client disconnected");
    });
}