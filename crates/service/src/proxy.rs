use crate::certs::CertificateAuthority;
use crate::cache::CacheManager;
use crate::downloader;
use anyhow::{Context, Result};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{upgrade, Body, Method, Request, Response};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::{error, info, instrument, warn, Instrument};

/// Ana proxy sunucusunu çalıştırır.
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
                    if !err.to_string().contains("connection reset")
                        && !err.to_string().contains("unexpected end of file") {
                        warn!(%err, "Connection error");
                    }
                }
            }
            .instrument(tracing::info_span!("client", %client_addr)),
        );
    }
}

/// Gelen her bir isteği işleyen ana servis fonksiyonu.
async fn proxy_service(
    req: Request<Body>,
    ca: Arc<CertificateAuthority>,
    cache: Arc<CacheManager>,
) -> Result<Response<Body>, hyper::Error> {
    if Method::CONNECT == req.method() {
        if let Some(host) = req.uri().authority().map(|auth| auth.to_string()) {
            let cache_clone = cache.clone();
            tokio::spawn(async move {
                match upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = serve_https(upgraded, host, ca, cache_clone).await {
                            if !e.to_string().contains("TLS handshake failed") {
                                error!("HTTPS tunnel error: {}", e);
                            }
                        }
                    }
                    Err(e) => error!("Upgrade error: {}", e),
                }
            });
            Ok(Response::new(Body::empty()))
        } else {
            error!("CONNECT request without host");
            let mut resp = Response::new(Body::from("CONNECT must be to a socket address"));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;
            Ok(resp)
        }
    } else {
        serve_http(req, cache).await
    }
}

/// Gelen HTTP isteklerini işler.
#[instrument(skip(req, cache), fields(uri = %req.uri()))]
async fn serve_http(
    req: Request<Body>,
    cache: Arc<CacheManager>,
) -> Result<Response<Body>, hyper::Error> {
    info!("Handling HTTP request");
    
    let cache_key = req.uri().to_string();

    if let Some(cached_body) = cache.get(&cache_key).await {
        return Ok(Response::new(cached_body));
    }

    match downloader::forward_request(req).await {
        Ok(mut response) => {
            let body_stream = std::mem::replace(response.body_mut(), Body::empty());
            match cache.put_stream(cache_key, body_stream).await {
                Ok(body_for_client) => {
                    *response.body_mut() = body_for_client;
                }
                Err(e) => {
                    error!("Failed to initiate cache stream: {}", e);
                }
            }
            Ok(response)
        }
        Err(e) => {
            error!("Failed to forward request: {}", e);
            let mut resp = Response::new(Body::from("Upstream request failed"));
            *resp.status_mut() = http::StatusCode::BAD_GATEWAY;
            Ok(resp)
        }
    }
}

/// Bir CONNECT tünelini sonlandırır ve şifreli trafiği işlemeye başlar.
#[instrument(skip_all, fields(host = %host))]
async fn serve_https(
    upgraded: upgrade::Upgraded,
    host: String,
    ca: Arc<CertificateAuthority>,
    cache: Arc<CacheManager>,
) -> Result<()> {
    info!("Handling CONNECT request, performing TLS handshake");
    // DÜZELTME BURADA: Gereksiz `&` kaldırıldı.
    let server_config = ca
        .get_server_config(host.split(':').next().unwrap_or(&host))
        .context("Failed to get server config for domain")?;
    
    let acceptor = TlsAcceptor::from(server_config);
    let stream = acceptor.accept(upgraded).await.context("TLS handshake failed")?;

    let service = service_fn(move |mut req| {
        let host_clone = host.clone();
        let cache_clone = cache.clone();
        async move {
            let authority = host_clone.parse::<http::uri::Authority>().unwrap();
            let uri = http::Uri::builder()
                .scheme("https")
                .authority(authority)
                .path_and_query(req.uri().path_and_query().unwrap().clone())
                .build()
                .unwrap();
            *req.uri_mut() = uri;
            serve_http(req, cache_clone).await
        }
    });

    Http::new()
        .http1_only(true)
        .serve_connection(stream, service)
        .await
        .context("Error serving HTTPS connection")?;

    Ok(())
}