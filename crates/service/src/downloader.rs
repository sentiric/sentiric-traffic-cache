use hyper::{client::HttpConnector, Body, Client, Request, Response};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use lazy_static::lazy_static;
use tracing::{debug, instrument};

// Hata Düzeltmesi: Client'ı HTTP/2'yi destekleyecek şekilde yapılandırıyoruz.
type HttpsClient = Client<HttpsConnector<HttpConnector>>;

lazy_static! {
    pub static ref HTTP_CLIENT: HttpsClient = {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        
        Client::builder()
            .http2_only(false) // Hem HTTP/1 hem HTTP/2'ye izin ver
            .http1_title_case_headers(true) // Bazı sunucularla uyumluluk için
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
    
    // Host başlığını ayarla, bu özellikle HTTP/1.1 için önemlidir.
    let host = req.uri().host().unwrap();
    let port = req.uri().port_u16().unwrap_or(if req.uri().scheme_str() == Some("https") { 443 } else { 80 });
    let host_header = format!("{}:{}", host, port);
    req.headers_mut().insert(hyper::header::HOST, host_header.parse()?);

    let response = HTTP_CLIENT.request(req).await?;
    Ok(response)
}