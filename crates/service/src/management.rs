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

lazy_static::lazy_static! {
    pub static ref EVENT_BROADCASTER: Sender<WsEvent> = broadcast::channel(128).0;
}

// DÜZELTME: Bu fonksiyonun 'pub' olması gerekiyor ki 'lib.rs' onu görebilsin.
pub fn api_routes(cache: Arc<CacheManager>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let cache_filter = warp::any().map(move || cache.clone());

    let stats_route = warp::path!("api" / "stats")
        .and(cache_filter.clone())
        .and_then(handle_stats);

    let events_route = warp::path!("api" / "events")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_websocket_connection));
    
    stats_route.or(events_route)
}

async fn handle_stats(cache: Arc<CacheManager>) -> Result<impl warp::Reply, warp::Rejection> {
    let stats = cache.get_stats().await;
    Ok(warp::reply::json(&stats))
}

async fn handle_websocket_connection(websocket: WebSocket) {
    info!("New WebSocket client connected");
    let (mut client_tx, _) = websocket.split();
    let mut rx = EVENT_BROADCASTER.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&event) {
                if client_tx.send(Message::text(json)).await.is_err() {
                    break;
                }
            }
        }
        info!("WebSocket client disconnected");
    });
}