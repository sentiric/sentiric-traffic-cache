// File: crates/service/src/management.rs

use crate::cache::CacheManager;
use crate::config;
use anyhow::Result;
use futures_util::{StreamExt, SinkExt};
use sentiric_core::{Stats, FlowEntry};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
use warp::ws::{Message, WebSocket};
use warp::Filter;
use serde::Serialize;
use tracing::{info, warn};

// ======================== DÜZELTME BAŞLANGICI ========================
// Enum'ı tuple variant'tan struct variant'a dönüştürüyoruz.
// Bu, serde_json'un ürettiği JSON alan adlarını ("stats", "flow")
// frontend'in beklentileriyle eşleşecek şekilde açıkça kontrol etmemizi sağlar.
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum WsEvent {
    StatsUpdated { stats: Stats },
    FlowUpdated { flow: FlowEntry },
}
// ========================= DÜZELTME BİTİŞİ =========================

lazy_static::lazy_static! {
    pub static ref EVENT_BROADCASTER: Sender<WsEvent> = broadcast::channel(128).0;
}

pub async fn run_server(addr: SocketAddr, cache: Arc<CacheManager>) -> Result<()> {
    info!("🚀 Management server listening on http://{}", addr);

    let cache_filter = warp::any().map(move || cache.clone());

    let stats_route = warp::path!("api" / "stats")
        .and(warp::get())
        .and(cache_filter.clone())
        .and_then(handle_stats);

    let entries_route = warp::path!("api" / "entries")
        .and(warp::get())
        .and(cache_filter.clone())
        .and_then(handle_list_entries);

    let clear_route = warp::path!("api" / "clear")
        .and(warp::post())
        .and(cache_filter.clone())
        .and_then(handle_clear_cache);
    
    let rules_route = warp::path!("api" / "rules")
        .and(warp::get())
        .and_then(handle_list_rules);

    let events_route = warp::path!("api" / "events")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_websocket_connection));
    
    let api_routes = stats_route.or(entries_route).or(clear_route).or(rules_route).or(events_route);

    let pac_route = warp::path!("proxy.pac")
        .and(warp::get())
        .map(handle_pac_file);

    let static_files = warp::fs::dir("web/dist")
        .or(warp::fs::file("web/dist/index.html"));

    let routes = api_routes.or(pac_route).or(static_files);

    warp::serve(routes).run(addr).await;

    Ok(())
}

async fn handle_list_rules() -> Result<impl warp::Reply, warp::Rejection> {
    let rules = &config::get().rules;
    Ok(warp::reply::json(rules))
}

fn handle_pac_file() -> impl warp::Reply {
    let content = "function FindProxyForURL(url, host) {\n  return 'PROXY 127.0.0.1:3128';\n}";
    warp::reply::with_header(content, "Content-Type", "application/x-ns-proxy-config")
}

async fn handle_stats(cache: Arc<CacheManager>) -> Result<impl warp::Reply, warp::Rejection> {
    let stats = cache.get_stats().await;
    Ok(warp::reply::json(&stats))
}

async fn handle_list_entries(cache: Arc<CacheManager>) -> Result<impl warp::Reply, warp::Rejection> {
    match cache.list_entries().await {
        Ok(entries) => Ok(warp::reply::json(&entries)),
        Err(e) => {
            warn!("Failed to list cache entries: {}", e);
            Err(warp::reject::not_found())
        }
    }
}

async fn handle_clear_cache(cache: Arc<CacheManager>) -> Result<impl warp::Reply, warp::Rejection> {
    match cache.clear_cache().await {
        Ok(_) => Ok(warp::reply::with_status("Cache cleared", http::StatusCode::OK)),
        Err(e) => {
            warn!("Failed to clear cache: {}", e);
            Err(warp::reject::not_found())
        }
    }
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