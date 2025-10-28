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
use rust_embed::RustEmbed;
use std::borrow::Cow;
use warp::http::header::HeaderValue;

// DÜZELTME: Yol, projenin kök dizinine göre doğru olmalıdır.
#[derive(RustEmbed)]
#[folder = "web/dist/"]
struct WebAssets;

// Koşullu derlemeye artık gerek yok, çünkü `Test and Lint` adımı bu dosyayı build etmiyor.
// Asıl build işlemi, 'web/dist' klasörünün zaten var olduğu `Integration Test` adımında yapılıyor.
// Bu, önceki yaklaşımı basitleştirir.

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum WsEvent {
    StatsUpdated(Stats),
}

lazy_static::lazy_static! {
    pub static ref EVENT_BROADCASTER: Sender<WsEvent> = broadcast::channel(128).0;
}

/// Starts the management web server.
pub async fn run_server(addr: SocketAddr, cache: Arc<CacheManager>) -> Result<()> {
    info!("🚀 Management server listening on http://{}", addr);

    let cache_filter = warp::any().map(move || cache.clone());

    let stats_route = warp::path!("api" / "stats")
        .and(cache_filter.clone())
        .and_then(handle_stats);

    let events_route = warp::path!("api" / "events")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_websocket_connection));
    
    let api_routes = stats_route.or(events_route);

    let static_files = warp::get().and(warp::path::tail()).and_then(serve_static_files);
    
    let routes = api_routes.or(static_files);

    warp::serve(routes).run(addr).await;

    Ok(())
}

async fn serve_static_files(path: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let path = if path.as_str().is_empty() { "index.html" } else { path.as_str() };
    
    match WebAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            let mut res = warp::reply::Response::new(Cow::from(content.data).into());
            res.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
            Ok(res)
        }
        None => match WebAssets::get("index.html") {
            Some(content) => {
                let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                let mut res = warp::reply::Response::new(Cow::from(content.data).into());
                res.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
                Ok(res)
            }
            None => Err(warp::reject::not_found()),
        },
    }
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