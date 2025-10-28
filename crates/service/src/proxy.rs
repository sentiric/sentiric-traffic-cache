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

#[instrument(skip(req, cache), fields(uri = %req.uri()))]
async fn serve_http(
    req: Request<Body>,
    cache: Arc<CacheManager>,
) -> Result<Response<Body>, hyper::Error> {
    // 'Host' başlığından gerçek hedefi al.
    let host = req.headers().get(hyper::header::HOST).and_then(|h| h.to_str().ok()).unwrap_or_default();
    let uri_string = if !host.is_empty() && req.uri().authority().is_none() {
        // Bu, şeffaf bir proxy isteğidir. URI'yi yeniden oluşturalım.
        format!("http://{}{}", host, req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("/"))
    } else {
        req.uri().to_string()
    };
    
    info!(target_uri = %uri_string, "Handling HTTP request");
    
    // DÜZELTME: cache_key'i ödünç almak yerine, string'in kendisi olarak tutuyoruz.
    let cache_key = uri_string.clone();

    if let Some(cached_body) = cache.get(&cache_key).await {
        return Ok(Response::new(cached_body));
    }
    
    cache.stats.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // İsteği, potansiyel olarak yeniden oluşturulmuş URI ile klonla.
    let mut builder = Request::builder()
        .method(req.method().clone())
        // DÜZELTME: 'uri_string'i taşımak yerine klonluyoruz.
        .uri(&uri_string) 
        .version(req.version());
    *builder.headers_mut().unwrap() = req.headers().clone();
    
    // .body() metodu sahiplik aldığı için, `req.into_body()` kullanmak en temizidir.
    let new_req = builder.body(req.into_body()).unwrap();

    match downloader::forward_request(new_req).await {
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

#[instrument(skip_all, fields(host = %host))]
async fn serve_https(
    upgraded: upgrade::Upgraded,
    host: String,
    ca: Arc<CertificateAuthority>,
    cache: Arc<CacheManager>,
) -> Result<()> {
    info!("Handling CONNECT request, performing TLS handshake");
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