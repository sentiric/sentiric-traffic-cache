// File: crates/service/src/proxy.rs

use crate::certs::CertificateAuthority;
use crate::cache::CacheManager;
use crate::downloader;
use crate::management::{EVENT_BROADCASTER, WsEvent};
use crate::rules::RuleEngine;
use anyhow::{Context, Result};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{upgrade, Body, Method, Request, Response, Uri};
use sentiric_core::{Action, FlowEntry};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, error, info, warn, Instrument};
use uuid::Uuid;

pub async fn run_server(
    addr: SocketAddr,
    ca: Arc<CertificateAuthority>,
    cache: Arc<CacheManager>,
) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("🚀 Proxy server listening on http://{}", addr);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        let ca_clone = ca.clone();
        let cache_clone = cache.clone();

        let service = service_fn(move |req| {
            let ca = ca_clone.clone();
            let cache = cache_clone.clone();
            proxy_service(req, ca, cache)
        });

        tokio::spawn(
            async move {
                if let Err(err) = Http::new()
                    .http1_only(true)
                    .http1_keep_alive(true)
                    .serve_connection(stream, service)
                    .with_upgrades()
                    .await
                {
                    if !err.to_string().contains("connection reset") && !err.to_string().contains("unexpected end of file") {
                        warn!(cause = ?err, "Connection error");
                    }
                }
            }
            .instrument(tracing::info_span!("client", %client_addr)),
        );
    }
}

async fn proxy_service(
    req: Request<Body>,
    ca: Arc<CertificateAuthority>,
    cache: Arc<CacheManager>,
) -> Result<Response<Body>, hyper::Error> {
    if Method::CONNECT == req.method() {
        if let Some(host) = req.uri().authority().map(|auth| auth.to_string()) {
            tokio::spawn(async move {
                match upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = serve_https(upgraded, host, ca, cache).await {
                             if !e.to_string().contains("TLS handshake failed") {
                                error!(cause = ?e, "HTTPS tunnel error");
                            }
                        }
                    }
                    Err(e) => error!(cause = ?e, "Upgrade error"),
                }
            });
            Ok(Response::new(Body::empty()))
        } else {
            let mut resp = Response::new(Body::from("CONNECT must be to a socket address"));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;
            Ok(resp)
        }
    } else {
        serve_http(req, cache, false).await
    }
}

async fn serve_http(
    mut req: Request<Body>,
    cache: Arc<CacheManager>,
    is_https: bool,
) -> Result<Response<Body>, hyper::Error> {
    let uri_string = if is_https {
        req.uri().to_string()
    } else {
        let host = req.headers().get(hyper::header::HOST).and_then(|h| h.to_str().ok()).unwrap_or_default();
        format!("http://{}{}", host, req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("/"))
    };
    
    *req.uri_mut() = uri_string.parse().unwrap();

    let rule_engine = RuleEngine::new(crate::config::get().rules.clone());
    let action = rule_engine.match_action(&uri_string);

    if action == Action::Block {
        info!("[BLOCK] {}", uri_string);
        let mut resp = Response::new(Body::from("Blocked by Sentiric Traffic Cache rule."));
        *resp.status_mut() = http::StatusCode::FORBIDDEN;
        return Ok(resp);
    }

    if action == Action::BypassCache {
        info!("[BYPASS] {}", uri_string);
        return match downloader::forward_request(req).await {
            Ok(resp) => Ok(resp),
            Err(e) => {
                error!("Bypass forward error: {}", e);
                let mut resp = Response::new(Body::from("Upstream request failed"));
                *resp.status_mut() = http::StatusCode::BAD_GATEWAY;
                Ok(resp)
            }
        };
    }

    // Action::Allow
    let cache_key = uri_string.clone();
    if let Some(cached_body) = cache.get(&cache_key).await {
        info!("[HIT] {}", uri_string);
        let _ = EVENT_BROADCASTER.send(WsEvent::FlowUpdated(FlowEntry {
            id: Uuid::new_v4().to_string(),
            method: req.method().to_string(),
            uri: uri_string,
            status_code: 200,
            response_size_bytes: 0,
            is_hit: true,
        }));
        return Ok(Response::new(cached_body));
    }

    info!("[MISS] {}", uri_string);
    cache.stats.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    match downloader::forward_request(req).await {
        Ok(mut response) => {
             let _ = EVENT_BROADCASTER.send(WsEvent::FlowUpdated(FlowEntry {
                id: Uuid::new_v4().to_string(),
                method: "GET".to_string(),
                uri: uri_string.clone(),
                status_code: response.status().as_u16(),
                response_size_bytes: response.headers().get(hyper::header::CONTENT_LENGTH).and_then(|v| v.to_str().ok()).and_then(|s| s.parse().ok()).unwrap_or(0),
                is_hit: false,
            }));

            let body_stream = std::mem::replace(response.body_mut(), Body::empty());
            if let Ok(body_for_client) = cache.put_stream(cache_key, body_stream).await {
                *response.body_mut() = body_for_client;
            }
            Ok(response)
        }
        Err(e) => {
            error!("Forward error: {}", e);
            let mut resp = Response::new(Body::from("Upstream request failed"));
            *resp.status_mut() = http::StatusCode::BAD_GATEWAY;
            Ok(resp)
        }
    }
}

async fn serve_https(
    upgraded: upgrade::Upgraded,
    host: String,
    ca: Arc<CertificateAuthority>,
    cache: Arc<CacheManager>,
) -> Result<()> {
    let server_config = ca.get_server_config(host.split(':').next().unwrap_or(&host))?;
    let stream = TlsAcceptor::from(server_config).accept(upgraded).await.context("TLS handshake failed")?;

    let service = service_fn(move |mut req: Request<Body>| {
        let cache = cache.clone();
        let host = host.clone();
        async move {
            // ======================== NİHAİ DÜZELTME BAŞLANGICI ========================
            // Sürdürülebilirlik: Bu iki işlemin neden gerekli olduğunu açıkla.
            // 1. HTTP/2'ye izin veriyoruz (aşağıdaki Http::new() çağrısında).
            // 2. Bu nedenle, istemci HTTP/2'ye yükselirse protokol hatası vermemesi
            //    için HTTP/1.1'e özgü "hop-by-hop" başlıklarını temizlemeliyiz.
            const HOP_BY_HOP_HEADERS: &[&str] = &[
                "connection", "keep-alive", "proxy-authenticate", "proxy-authorization",
                "te", "trailers", "transfer-encoding", "upgrade", "proxy-connection",
            ];
            
            let headers = req.headers_mut();
            for header in HOP_BY_HOP_HEADERS {
                if headers.remove(*header).is_some() {
                    debug!("Stripped hop-by-hop header for tunneled request: {}", header);
                }
            }
            // ========================= NİHAİ DÜZELTME BİTİŞİ =========================

            let authority = host.parse::<http::uri::Authority>().unwrap();
            let uri = Uri::builder()
                .scheme("https")
                .authority(authority)
                .path_and_query(req.uri().path_and_query().unwrap().clone())
                .build()
                .unwrap();
            *req.uri_mut() = uri;
            serve_http(req, cache, true).await
        }
    });

    // Düzeltme: `.http1_only(true)` kaldırıldı.
    // Artık sunucu, istemcinin tercihine göre HTTP/1.1 veya HTTP/2 konuşabilir (ALPN).
    Http::new()
        .serve_connection(stream, service)
        .await
        .context("Error serving HTTPS connection")
}