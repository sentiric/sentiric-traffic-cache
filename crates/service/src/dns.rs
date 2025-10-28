use anyhow::Result;
use async_trait::async_trait;
use sentiric_core::Settings;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{debug, info};

// DÜZELTME: Kütüphanenin yeni versiyonu için doğru modül yolları
use trust_dns_server::authority::{
    Authority, BoxAuthority, Catalog, LookupError, LookupObject, LookupOptions, LookupRecords,
    Name, ZoneType,
};
use trust_dns_server::proto::op::ResponseCode;
use trust_dns_server::proto::rr::{rdata, Record, RecordSet, RrKey};
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ServerFuture};

/// Tüm DNS sorgularını yakalayan ve sabit bir IP adresiyle yanıtlayan
/// bir DNS yetkili sunucusu.
#[derive(Clone)]
pub struct InterceptAuthority {
    response_ip: IpAddr,
    origin: Name,
}

impl InterceptAuthority {
    pub fn new(response_ip: IpAddr) -> Self {
        Self {
            response_ip,
            origin: Name::root(),
        }
    }
}

#[async_trait]
impl Authority for InterceptAuthority {
    type Lookup = LookupRecords;

    fn zone_type(&self) -> ZoneType {
        ZoneType::Primary
    }

    fn is_axfr_allowed(&self) -> bool {
        false
    }

    async fn update(
        &self,
        _update: &trust_dns_server::proto::op::Message,
    ) -> Result<bool, LookupError> {
        Ok(false)
    }

    fn origin(&self) -> &Name {
        &self.origin
    }

    // DÜZELTME: Yeni `search` fonksiyonu, `lookup`'ın yerini aldı.
    async fn search(
        &self,
        request_info: trust_dns_server::server::RequestInfo<'_>,
        _lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        let name = request_info.query.name();
        let query_type = request_info.query.query_type();
        debug!("Intercepting DNS lookup for: {} ({})", name, query_type);

        let mut records = RecordSet::new(name, query_type, 0);

        match (query_type, self.response_ip) {
            (RecordType::A, IpAddr::V4(ipv4)) => {
                let rdata = rdata::A(ipv4);
                let mut record = Record::with(name.clone(), RecordType::A, 60);
                record.set_rdata(rdata.into());
                records.add_rdata(record.rdata().clone());
                records.set_ttl(60);
            }
            (RecordType::AAAA, IpAddr::V6(ipv6)) => {
                let rdata = rdata::AAAA(ipv6);
                let mut record = Record::with(name.clone(), RecordType::AAAA, 60);
                record.set_rdata(rdata.into());
                records.add_rdata(record.rdata().clone());
                records.set_ttl(60);
            }
            _ => (),
        }

        if records.is_empty() {
            return Err(LookupError::from(ResponseCode::NXDomain));
        }

        Ok(LookupObject::new(Arc::new(records)).into())
    }

    // DÜZELTME: Diğer gerekli trait metodları için varsayılan implementasyonlar
    async fn get_nsec_records(
        &self,
        _name: &Name,
        _lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        Err(LookupError::from(ResponseCode::Refused))
    }
}

// DÜZELTME: `RequestHandler` artık Authority'yi yöneten Catalog'u kullanıyor.
#[derive(Clone)]
pub struct DnsHandler {
    catalog: Arc<Catalog>,
}

#[async_trait]
impl RequestHandler for DnsHandler {
    async fn handle_request<H: ResponseHandler>(&self, request: &Request, mut response_handle: H) {
        let result = self.catalog.handle_request(request).await;
        if let Err(e) = response_handle.send_response(result).await {
            info!("error sending DNS response: {}", e);
        }
    }
}

/// DNS sunucusunu başlatan ve çalıştıran ana fonksiyon.
pub async fn run_server(settings: &Settings) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", settings.dns.bind_address, settings.dns.port).parse()?;
    info!("🚀 Smart DNS server listening on udp://{}", addr);

    let authority = InterceptAuthority::new(settings.dns.response_ip);
    let mut catalog = Catalog::new();
    catalog.upsert(
        Name::root().into(),
        Box::new(Arc::new(authority)) as BoxAuthority,
    );

    let handler = DnsHandler {
        catalog: Arc::new(catalog),
    };

    let mut server = ServerFuture::new(handler);
    server.register_socket(UdpSocket::bind(addr).await?);

    server.block_until_done().await?;
    
    Ok(())
}