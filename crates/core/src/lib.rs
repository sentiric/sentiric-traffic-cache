//! The Core Crate
//!
//! This crate contains the shared business logic, data structures, and traits
//! for the `sentiric-traffic-cache`.

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

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
    #[serde(default)] 
    pub dns: Dns,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Proxy {
    pub port: u16,
    pub bind_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Management {
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

#[derive(Debug, Deserialize, Clone)]
pub struct Dns {
    pub enabled: bool,
    pub port: u16,
    pub bind_address: String,
    pub response_ip: IpAddr,
}

impl Default for Dns {
    fn default() -> Self {
        Self {
            enabled: false,
            port: 53,
            bind_address: "0.0.0.0".to_string(),
            response_ip: "127.0.0.1".parse().unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntryInfo {
    pub key: String,
    pub size_bytes: u64,
}