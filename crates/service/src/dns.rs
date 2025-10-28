use anyhow::Result;
use async_trait::async_trait;
use sentiric_core::Settings;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{debug, info};

// DÜZELTME: Kütüphanenin yeni versiyonu için doğru ve tam modül yolları
use trust_dns_server::authority::{
    Authority, BoxAuthority, Catalog, LookupError, LookupObject, LookupOptions,
};
use trust_dns_server::proto::op::ResponseCode;
use trust_dns_server::proto::rr::{rdata, LowerName, Name, Record, RecordSet, RecordType};
use trust_dns_server::server::{Request, RequestHandler, RequestInfo, ResponseHandler, ServerFuture};

/// Tüm DNS sorgularını yakalayan ve sabit bir IP adresiyle yanıtlayan
/// bir DNS yetkili sunucusu.
#[derive(Clone, Debug)]
pub struct InterceptAuthority {
    response_ip: IpAddr,
    origin: LowerName,
}

impl InterceptAuthority {
    pub fn new(response_ip: IpAddr) -> Self {
        Self {
            response_ip,
            origin: LowerName::new(&Name::root()),
        }
    }
}

#[async_trait]
impl Authority for InterceptAuthority {
    type Lookup = LookupObject;

    fn zone_type(&self) -> ZoneType {
        ZoneType::Primary
    }

    fn is_axfr_allowed(&self) -> bool {
        false
    }

    async fn update(
        &self,
        _update: &trust_dns_server::authority::MessageRequest,
    ) -> Result<bool, ResponseCode> {
        Ok(false)
    }

    fn origin(&self) -> &LowerName {
        &self.origin
    }

    // DÜZELTME: `search`'ün yerini alan yeni `lookup` fonksiyonu
    async fn lookup(
        &self,
        name: &LowerName,
        query_type: RecordType,
        _lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        info!("Intercepting DNS lookup for: {} ({})", name, query_type);

        let mut record_set = RecordSet::new(name.into(), query_type, 0);

        match (query_type, self.response_ip) {
            (RecordType::A, IpAddr::V4(ipv4)) => {
                let rdata = rdata::A(ipv4);
                let mut record = Record::with(name.into(), RecordType::A, 60);
                record.set_data(Some(rdata.into()));
                record_set.insert(record, 0);
            }
            (RecordType::AAAA, IpAddr::V6(ipv6)) => {
                let rdata = rdata::AAAA(ipv6);
                let mut record = Record::with(name.into(), RecordType::AAAA, 60);
                record.set_data(Some(rdata.into()));
                record_set.insert(record, 0);
            }
            _ => (),
        }

        if record_set.is_empty() {
            return Err(LookupError::from(ResponseCode::NXDomain));
        }

        Ok(LookupObject::new(Arc::new(record_set)))
    }

    // DÜZELTME: Diğer gerekli trait metodları için varsayılan implementasyonlar
    async fn search(
        &self,
        request_info: RequestInfo<'_>,
        lookup_options: LookupOptions,
    ) -> Result<Self::Lookup, LookupError> {
        self.lookup(
            request_info.query.name(),
            request_info.query.query_type(),
            lookup_options,
        )
        .await
    }
    
    async fn get_nsec_records(
        &self,
        _name: &LowerName,
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
    async fn handle_request<H: ResponseHandler>(&self, request: &Request, response_handle: H) {
        let result = self.catalog.handle_request(request, Default::default()).await;
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
        LowerName::new(&Name::root()),
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