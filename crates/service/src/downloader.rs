// File: crates/service/src/downloader.rs

use hyper::{client::HttpConnector, Body, Client, Request, Response, Version};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use lazy_static::lazy_static;
use tracing::{debug, instrument};

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

lazy_static! {
    pub static ref HTTP_CLIENT: HttpsClient = {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        
        Client::builder()
            .http2_only(false)
            .build(https)
    };
}

/// Forwards a request to the internet using the shared HTTP client.
#[instrument(skip(req))]
pub async fn forward_request(
    mut req: Request<Body>,
) -> anyhow::Result<Response<Body>> {
    debug!("Forwarding request to the internet");

    // ======================== NİHAİ DÜZELTME BAŞLANGICI ========================
    // Sürdürülebilirlik: "Hop-by-hop" başlıkları sadece tek bir bağlantı için
    // geçerlidir ve proxy tarafından asla ileriye taşınmamalıdır. Bu başlıkları
    // temizlemek, hem protokol standartlarına uymamızı sağlar hem de hedef
    // sunucuların isteği reddetmesini önler.
    const HOP_BY_HOP_HEADERS: &[&str] = &[
        "connection", "keep-alive", "proxy-authenticate", "proxy-authorization",
        "te", "trailers", "transfer-encoding", "upgrade", "proxy-connection",
    ];

    let headers = req.headers_mut();
    for header in HOP_BY_HOP_HEADERS {
        if headers.remove(*header).is_some() {
            debug!("Stripped hop-by-hop header before forwarding: {}", header);
        }
    }
    // ========================= NİHAİ DÜZELTME BİTİŞİ =========================
    
    let host = req.uri().host().unwrap();
    let port = req.uri().port_u16().unwrap_or(if req.uri().scheme_str() == Some("https") { 443 } else { 80 });
    let host_header = format!("{}:{}", host, port);
    req.headers_mut().insert(hyper::header::HOST, host_header.parse()?);

    // Bu satır, dış dünyaya giden isteklerin kararlılığını artırmak için
    // HTTP/1.1 kullanmaya zorlar. Bu, birçok sunucuyla uyumluluğu garanti eder.
    *req.version_mut() = Version::HTTP_11;

    let response = HTTP_CLIENT.request(req).await?;
    Ok(response)
}