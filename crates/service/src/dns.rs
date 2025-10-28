use anyhow::Result;
use sentiric_core::Settings;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{info, warn};
use trust_dns_server::authority::{
    Authority, Catalog, MessageResponseBuilder, ZoneType,
};
use trust_dns_server::proto::rr::{rdata::SOA, RData, Record, RecordType};
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ServerFuture};
use trust_dns_server::store::in_memory::InMemoryAuthority;
use trust_dns_server::Name;

/// Tüm DNS sorgularını yakalayan ve sabit bir IP adresiyle yanıtlayan
/// bir DNS yetkili sunucusu.
pub struct InterceptAuthority {
    response_ip: IpAddr,
    soa: RData,
    ns: RData,
}

impl InterceptAuthority {
    pub fn new(response_ip: IpAddr) -> Self {
        let soa_name = Name::from_str("interceptor.internal.").unwrap();
        let ns_name = Name::from_str("ns.interceptor.internal.").unwrap();
        Self {
            response_ip,
            soa: RData::SOA(SOA::new(
                soa_name.clone(),
                soa_name,
                0,
                3600,
                3600,
                3600,
                3600,
            )),
            ns: RData::NS(ns_name),
        }
    }
}

#[async_trait::async_trait]
impl Authority for InterceptAuthority {
    type Lookup = trust_dns_server::authority::EmptyLookup;

    fn zone_type(&self) -> ZoneType {
        ZoneType::Primary
    }

    fn is_axfr_allowed(&self) -> bool {
        false
    }

    async fn update(&self, _update: &trust_dns_server::proto::op::UpdateMessage) -> std::io::Result<bool> {
        Ok(false)
    }

    fn origin(&self) -> &Name {
        // Bu, tüm sorguları yakalamak için "." (kök) bölgesi gibi davranır.
        Name::root()
    }

    // Bu fonksiyon, asıl sihrin gerçekleştiği yerdir.
    async fn lookup(
        &self,
        name: &Name,
        query_type: RecordType,
        _is_secure: bool,
    ) -> std::io::Result<Self::Lookup> {
        info!("Intercepting DNS lookup for: {} ({})", name, query_type);
        
        let mut records = Vec::new();

        // Gelen sorgu A (IPv4) veya AAAA (IPv6) ise, bizim IP adresimizle cevap ver.
        match (query_type, self.response_ip) {
            (RecordType::A, IpAddr::V4(ipv4)) => {
                let rdata = RData::A(ipv4);
                records.push(Record::from_rdata(name.clone(), 60, rdata));
            }
            (RecordType::AAAA, IpAddr::V6(ipv6)) => {
                let rdata = RData::AAAA(ipv6);
                records.push(Record::from_rdata(name.clone(), 60, rdata));
            }
            // Diğer tüm sorgu tipleri için boş cevap veriyoruz ama hata vermiyoruz.
            _ => (),
        }

        Ok(trust_dns_server::authority::EmptyLookup::new(
            Arc::from(records),
        ))
    }

    // Bu fonksiyonlar, sunucunun "yetkili" gibi davranması için gereklidir.
    async fn soa(&self) -> std::io::Result<Self::Lookup> {
        let mut records = Vec::new();
        records.push(Record::from_rdata(
            self.origin().clone(),
            3600,
            self.soa.clone(),
        ));
        Ok(trust_dns_server::authority::EmptyLookup::new(
            Arc::from(records),
        ))
    }

    async fn ns(&self) -> std::io::Result<Self::Lookup> {
        let mut records = Vec::new();
        records.push(Record::from_rdata(
            self.origin().clone(),
            3600,
            self.ns.clone(),
        ));
        Ok(trust_dns_server::authority::EmptyLookup::new(
            Arc::from(records),
        ))
    }
}

/// DNS sunucusunu başlatan ve çalıştıran ana fonksiyon.
pub async fn run_server(settings: &Settings) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", settings.dns.bind_address, settings.dns.port).parse()?;
    info!("🚀 Smart DNS server listening on udp://{}", addr);

    // Tüm sorguları yakalamak için Catalog'umuza InterceptAuthority'yi ekliyoruz.
    let authority = InterceptAuthority::new(settings.dns.response_ip);
    let mut catalog = Catalog::new();
    catalog.upsert(Name::root().into(), Box::new(Arc::new(authority)));

    // Sunucuyu bu katalog ile başlat
    let mut server = ServerFuture::new(catalog);
    server.register_socket(UdpSocket::bind(addr).await?);

    server.block_until_done().await?;
    
    Ok(())
}