use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::info;
// DÜZELTME: Sadece kullandığımız modülleri import ediyoruz.
use trust_dns_server::authority::MessageResponseBuilder;
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ServerFuture};

pub struct DnsHandler;

// DÜZELTME: `async_trait` macro'sunu doğru şekilde kullanıyoruz.
#[async_trait::async_trait]
impl RequestHandler for DnsHandler {
    // DÜZELTME: Fonksiyon imzasını, trait'in beklediği tam formata getiriyoruz.
    async fn handle_request<H: ResponseHandler>(
        &self,
        request: &Request,
        response_handle: H,
    ) -> std::io::Result<()> {
        info!("Received DNS request: {:?}", request.query().name());
        
        let builder = MessageResponseBuilder::from_message_request(request);
        let response = builder.error_msg(request.header(), trust_dns_server::proto::op::ResponseCode::NXDomain);
        
        // DÜZELTME: `send_response`'un sonucunu `map` ile doğru tipe dönüştürüyoruz.
        response_handle.send_response(response).await.map(|_| ())
    }
}

pub async fn run_server(addr: SocketAddr) -> Result<()> {
    info!("🚀 DNS server listening on udp://{}", addr);

    let handler = DnsHandler;

    let mut server = ServerFuture::new(handler);
    server.register_socket(UdpSocket::bind(addr).await?);

    server.block_until_done().await?;
    
    Ok(())
}