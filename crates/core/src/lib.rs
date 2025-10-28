//! The Core Crate
//!
//! This crate contains the shared business logic, data structures, and traits
//! for the `sentiric-traffic-cache`.

use serde::{Deserialize, Serialize}; // Serialize'ı da ekliyoruz

// Stats yapısını core'a taşıyoruz, çünkü hem service hem de management
// tarafından kullanılacak.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub hits: u64,
    pub misses: u64,
    pub total_requests: u64,
    pub disk_items: u64,
    pub total_disk_size_bytes: u64,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub proxy: Proxy,
    pub certs: Certs,
    pub cache: Cache,
    pub management: Management,
    pub dns: Dns, // <-- EKLENDİ
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dns { // <-- EKLENDİ
    pub enabled: bool,
    pub port: u16,
    pub bind_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Proxy {
    pub port: u16,
    pub bind_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Management { // <-- EKLENDİ
    pub port: u16,
    pub bind_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Certs {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cache {
    pub path: String,
}