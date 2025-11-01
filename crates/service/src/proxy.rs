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
use tracing::{error, info, warn, Instrument};
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
    
    // Cache kontrolü
    if let Some(cached_body) = cache.get(&cache_key).await {
        info!("[HIT] {}", uri_string);
        
        // Header'ları al ve response'u oluştur
        let mut response = Response::new(cached_body);
        
        // Cache'ten header bilgilerini al
        if let Some((content_encoding, content_type)) = cache.get_headers(&cache_key).await {
            if !content_encoding.is_empty() {
                response.headers_mut().insert("content-encoding", content_encoding.parse().unwrap());
            }
            if !content_type.is_empty() {
                response.headers_mut().insert("content-type", content_type.parse().unwrap());
            }
        }
        
        // Vary header'ını garanti et
        response.headers_mut().insert("vary", "accept-encoding".parse().unwrap());
        
        let _ = EVENT_BROADCASTER.send(WsEvent::FlowUpdated {
            flow: FlowEntry {
                id: Uuid::new_v4().to_string(),
                method: req.method().to_string(),
                uri: uri_string,
                status_code: 200,
                response_size_bytes: 0,
                is_hit: true,
            }
        });
        return Ok(response);
    }

    info!("[MISS] {}", uri_string);

    match downloader::forward_request(req).await {
        Ok(mut response) => {
            let status_code = response.status().as_u16();
            let content_length = response.headers()
                .get(hyper::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            
            let _ = EVENT_BROADCASTER.send(WsEvent::FlowUpdated {
                flow: FlowEntry {
                    id: Uuid::new_v4().to_string(),
                    method: "GET".to_string(),
                    uri: uri_string.clone(),
                    status_code,
                    response_size_bytes: content_length,
                    is_hit: false,
                }
            });

            // Header'ları al
            let content_encoding = response.headers()
                .get("content-encoding")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());
                
            let content_type = response.headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());

            let body_stream = std::mem::replace(response.body_mut(), Body::empty());
            if let Ok(body_for_client) = cache.put_stream(cache_key, body_stream, content_encoding, content_type).await {
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

    Http::new()
        .serve_connection(stream, service)
        .await
        .context("Error serving HTTPS connection")
}