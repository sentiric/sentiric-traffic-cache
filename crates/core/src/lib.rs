//! The Core Crate
//!
//! This crate contains the shared business logic, data structures, and traits
//! for the `sentiric-traffic-cache`.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub proxy: Proxy,
    pub certs: Certs,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Proxy {
    pub port: u16,
    pub bind_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Certs {
    pub path: String,
}