use hyper::{client::HttpConnector, Body, Client, Request, Response, Version}; // Version eklendi
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
            // .enable_http2() bu builder üzerinde doğru değil, Client::builder() ile yapılmalı
            .build();
        
        // HTTP/2'yi deneyecek ama HTTP/1'e geri dönebilecek şekilde yapılandır
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

    req.headers_mut().remove("proxy-connection");
    req.headers_mut().remove("proxy-authorization");
    
    let host = req.uri().host().unwrap();
    let port = req.uri().port_u16().unwrap_or(if req.uri().scheme_str() == Some("https") { 443 } else { 80 });
    let host_header = format!("{}:{}", host, port);
    req.headers_mut().insert(hyper::header::HOST, host_header.parse()?);

    // --- NIHAI HATA DÜZELTMESİ ---
    // Eğer istemci bize HTTP/2 ile geldiyse bile, dış dünyaya gönderdiğimiz isteği
    // HTTP/1.1'e zorlayarak protokol uyuşmazlığı hatasını gideriyoruz.
    *req.version_mut() = Version::HTTP_11;

    let response = HTTP_CLIENT.request(req).await?;
    Ok(response)
}