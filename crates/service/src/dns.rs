use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::info;
use trust_dns_server::authority::MessageResponseBuilder;
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ServerFuture};

pub struct DnsHandler;

#[async_trait::async_trait]
impl RequestHandler for DnsHandler {
    async fn handle_request<H: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: H, // <--- DÜZELTME: 'mut' eklendi
    ) -> std::io::Result<()> {
        info!("Received DNS request for: {:?}", request.query().name());

        let builder = MessageResponseBuilder::from_message_request(request);
        let response = builder.error_msg(request.header(), trust_dns_server::proto::op::ResponseCode::NXDomain);

        // DÜZELTME: Kütüphane artık doğrudan Result<(), io::Error> bekliyor.
        response_handle.send_response(response).await
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