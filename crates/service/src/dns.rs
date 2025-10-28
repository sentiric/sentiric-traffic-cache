use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::info;
use trust_dns_server::authority::MessageResponseBuilder;
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ResponseInfo, ServerFuture};

pub struct DnsHandler;

#[async_trait::async_trait]
impl RequestHandler for DnsHandler {
    // DÜZELTME: Fonksiyonun geri dönüş tipini 'ResponseInfo' olarak değiştiriyoruz.
    async fn handle_request<H: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: H,
    ) -> ResponseInfo {
        info!("Received DNS request for: {:?}", request.query().name());

        let builder = MessageResponseBuilder::from_message_request(request);
        let response = builder.error_msg(request.header(), trust_dns_server::proto::op::ResponseCode::NXDomain);
        
        // DÜZELTME: 'send_response' zaten 'ResponseInfo' döndürüyor.
        // `await`'ten sonra gelen `Result`'ı açıp (unwrap) doğrudan döndürüyoruz.
        response_handle.send_response(response).await.unwrap()
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