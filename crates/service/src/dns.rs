use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::{info, warn};
use trust_dns_server::authority::{Authority, MessageResponseBuilder, ZoneType};
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ServerFuture};
use trust_dns_server::store::in_memory::InMemoryAuthority;
use trust_dns_server::Name;

/// Sadece "Merhaba" diyen basit bir DNS sunucusu.
struct DnsHandler;

#[async_trait::async_trait]
impl RequestHandler for DnsHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> std::io::Result<()> {
        info!("Received DNS request: {:?}", request.query().name());
        
        // Şimdilik, her isteğe "Böyle bir domain yok" (NXDOMAIN) cevabı verelim.
        // Bu, sunucunun çalıştığını doğrulamamızı sağlar.
        let builder = MessageResponseBuilder::from_message_request(request);
        let response = builder.error_msg(request.header(), trust_dns_server::proto::op::ResponseCode::NXDomain);
        
        response_handle.send_response(response).await
    }
}

/// DNS sunucusunu başlatan ve çalıştıran ana fonksiyon.
pub async fn run_server(addr: SocketAddr) -> Result<()> {
    info!("🚀 DNS server listening on udp://{}", addr);

    let handler = DnsHandler;

    // UDP soketi üzerinde bir sunucu oluştur
    let mut server = ServerFuture::new(handler);
    server.register_socket(UdpSocket::bind(addr).await?);

    // Sunucuyu çalıştır
    server.block_until_done().await?;
    
    Ok(())
}