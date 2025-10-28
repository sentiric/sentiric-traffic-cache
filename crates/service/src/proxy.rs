use crate::certs::CertificateAuthority;
use anyhow::{Context, Result};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{upgrade, Body, Method, Request, Response};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::{error, info, instrument, warn};

/// Ana proxy sunucusunu çalıştırır.
pub async fn run_server(addr: SocketAddr, ca: Arc<CertificateAuthority>) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("🚀 Proxy server listening on http://{}", addr);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        let ca_clone = ca.clone();

        let service = service_fn(move |req| {
            let ca = ca_clone.clone();
            proxy_service(req, ca)
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
                    // Sık karşılaşılan ve genellikle zararsız olan hataları görmezden gel
                    if !err.to_string().contains("connection reset")
                        && !err.to_string().contains("unexpected end of file")
                    {
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
) -> Result<Response<Body>, hyper::Error> {
    if Method::CONNECT == req.method() {
        // HTTPS isteği için bir tünel talebi
        if let Some(host) = req.uri().authority().map(|auth| auth.to_string()) {
            tokio::spawn(async move {
                match upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = serve_https(upgraded, host, ca).await {
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
        // Normal HTTP isteği
        serve_http(req).await
    }
}

/// Gelen HTTP isteklerini işler.
#[instrument(skip_all, fields(uri = %req.uri()))]
async fn serve_http(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    info!("Handling HTTP request");
    // TODO: İsteği cache'e veya internete yönlendir.
    Ok(Response::new(Body::from(format!(
        "HTTP Request Received for {}",
        req.uri()
    ))))
}

/// Bir CONNECT tünelini sonlandırır ve şifreli trafiği işlemeye başlar.
#[instrument(skip_all, fields(host = %host))]
async fn serve_https(
    upgraded: upgrade::Upgraded,
    host: String,
    ca: Arc<CertificateAuthority>,
) -> Result<()> {
    info!("Handling CONNECT request, performing TLS handshake");
    // Domain için anında bir sertifika oluştur
    let server_config = ca
        .get_server_config(&host.split(':').next().unwrap_or(&host))
        .context("Failed to get server config for domain")?;
    
    // TLS el sıkışmasını gerçekleştir
    let acceptor = TlsAcceptor::from(server_config);
    let stream = acceptor.accept(upgraded).await.context("TLS handshake failed")?;

    // Artık şifresi çözülmüş akış üzerinden hizmet ver
    let service = service_fn(move |mut req| {
        // Şifresi çözülmüş isteğin URI'sini yeniden oluştur
        let host_clone = host.clone();
        async move {
            let authority = host_clone.parse::<http::uri::Authority>().unwrap();
            let uri = http::Uri::builder()
                .scheme("https")
                .authority(authority)
                .path_and_query(req.uri().path_and_query().unwrap().clone())
                .build()
                .unwrap();
            *req.uri_mut() = uri;
            serve_http(req).await
        }
    });

    Http::new()
        .http1_only(true) // Şimdilik sadece HTTP/1.1
        .serve_connection(stream, service)
        .await
        .context("Error serving HTTPS connection")?;

    Ok(())
}