use anyhow::Result;
use async_trait::async_trait;
use sentiric_core::Settings;
use std::net::{IpAddr, SocketAddr};
use tokio::net::UdpSocket;
use tracing::{info, warn};

use trust_dns_server::authority::MessageResponseBuilder;
use trust_dns_server::proto::op::{Header, ResponseCode};
use trust_dns_server::proto::rr::{rdata, Record, RecordType, RData};
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ResponseInfo, ServerFuture};

#[derive(Clone)]
pub struct DnsHandler {
    response_ip: IpAddr,
}

impl DnsHandler {
    pub fn new(response_ip: IpAddr) -> Self {
        Self { response_ip }
    }
}

#[async_trait]
impl RequestHandler for DnsHandler {
    async fn handle_request<H: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: H,
    ) -> ResponseInfo {
        let response_builder = MessageResponseBuilder::from_message_request(request);
        
        let query = request.query();
        info!("Intercepting DNS lookup for: {} ({})", query.name(), query.query_type());

        let answers: Vec<Record> = match (query.query_type(), self.response_ip) {
            (RecordType::A, IpAddr::V4(ipv4)) => {
                let rdata = rdata::A(ipv4);
                vec![Record::from_rdata(query.name().clone().into(), 60, RData::A(rdata))]
            }
            (RecordType::AAAA, IpAddr::V6(ipv6)) => {
                let rdata = rdata::AAAA(ipv6);
                vec![Record::from_rdata(query.name().clone().into(), 60, RData::AAAA(rdata))]
            }
            _ => Vec::new(),
        };

        let mut header = *request.header();
        header.set_authoritative(true);
        
        // DÜZELTME: `empty_answers`'ı `if` bloğunun dışına taşıyoruz.
        let empty_answers: Vec<Record> = vec![]; 
        let response = if answers.is_empty() {
            header.set_response_code(ResponseCode::NXDomain);
            response_builder.build(header, empty_answers.iter(), None.iter(), None.iter(), None.iter())
        } else {
            response_builder.build(header, answers.iter(), None.iter(), None.iter(), None.iter())
        };

        match response_handle.send_response(response).await {
            Ok(info) => info,
            Err(e) => {
                warn!("error sending DNS response: {}", e);
                let mut error_header = Header::response_from_request(request.header());
                error_header.set_response_code(ResponseCode::ServFail);
                ResponseInfo::from(error_header)
            }
        }
    }
}

pub async fn run_server(settings: &Settings) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", settings.dns.bind_address, settings.dns.port).parse()?;
    info!("🚀 Smart DNS server listening on udp://{}", addr);

    let handler = DnsHandler::new(settings.dns.response_ip);

    let mut server = ServerFuture::new(handler);
    server.register_socket(UdpSocket::bind(addr).await?);

    server.block_until_done().await?;
    
    Ok(())
}