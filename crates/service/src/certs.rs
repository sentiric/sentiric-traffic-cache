// File: crates/service/src/certs.rs

use anyhow::{Context, Result};
use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair, SanType,
    PKCS_ECDSA_P256_SHA256,
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio_rustls::rustls::{self, PrivateKey, ServerConfig};
use tracing::{debug, info};

/// Manages the root CA and generates leaf certificates for domains on the fly.
pub struct CertificateAuthority {
    ca_cert: Certificate,
    _certs_path: PathBuf,
    leaf_cache: Mutex<HashMap<String, Arc<ServerConfig>>>,
}

impl CertificateAuthority {
    pub fn new(path: &str) -> Result<Self> {
        let certs_path = Path::new(path);
        fs::create_dir_all(certs_path).context("Failed to create certificate directory")?;

        let ca_cert_path = certs_path.join("ca.crt");
        let ca_key_path = certs_path.join("ca.key");

        let cert: Certificate;
        if ca_cert_path.exists() && ca_key_path.exists() {
            info!("Loading existing Root CA...");
            let cert_pem = fs::read_to_string(&ca_cert_path)?;
            let key_pem = fs::read_to_string(&ca_key_path)?;
            let key_pair = KeyPair::from_pem(&key_pem)?;
            let params = CertificateParams::from_ca_cert_pem(&cert_pem, key_pair)?;
            cert = Certificate::from_params(params)?;
        } else {
            info!("Generating new Root CA...");
            let mut params = CertificateParams::new(vec![]);
            params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
            params.distinguished_name = DistinguishedName::new();
            params.distinguished_name.push(DnType::OrganizationName, "Sentiric Traffic Cache");
            params.distinguished_name.push(DnType::CommonName, "Sentiric CA (Development)");
            params.alg = &PKCS_ECDSA_P256_SHA256;

            cert = Certificate::from_params(params)?;
            let cert_pem = cert.serialize_pem()?;
            let key_pem = cert.serialize_private_key_pem();
            fs::write(&ca_cert_path, &cert_pem)?;
            fs::write(&ca_key_path, &key_pem)?;
            info!("New Root CA saved to {:?}", certs_path);
        }

        Ok(Self {
            ca_cert: cert,
            _certs_path: certs_path.to_path_buf(),
            leaf_cache: Mutex::new(HashMap::new()),
        })
    }
    
    pub fn get_server_config(&self, domain: &str) -> Result<Arc<ServerConfig>> {
        let mut cache = self.leaf_cache.lock().unwrap();
        if let Some(config) = cache.get(domain) {
            debug!("Leaf certificate loaded from memory cache for: {}", domain);
            return Ok(config.clone());
        }

        debug!("Generating new leaf certificate for: {}", domain);
        let mut params = CertificateParams::new(vec![domain.to_string()]);
        params.subject_alt_names.push(SanType::DnsName(domain.to_string()));
        params.alg = &PKCS_ECDSA_P256_SHA256;

        let leaf_cert = Certificate::from_params(params)?;
        let cert_pem = leaf_cert.serialize_pem_with_signer(&self.ca_cert)?;
        let key_pem = leaf_cert.serialize_private_key_pem();
        
        let mut cert_chain_bytes = cert_pem.as_bytes().to_vec();
        cert_chain_bytes.extend_from_slice(b"\n");
        cert_chain_bytes.extend_from_slice(self.ca_cert.serialize_pem()?.as_bytes());

        let cert_chain = rustls_pemfile::certs(&mut &*cert_chain_bytes)?
            .into_iter()
            .map(rustls::Certificate)
            .collect();

        let key_der = rustls_pemfile::pkcs8_private_keys(&mut key_pem.as_bytes())?.remove(0);
        let key = PrivateKey(key_der);

        let mut config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)?;
        
        // ======================== NİHAİ DÜZELTME ========================
        // Sürdürülebilirlik: ALPN protokollerini, HTTP katmanının beklentisiyle
        // (http1_only) tutarlı hale getiriyoruz. Bu, TLS el sıkışması sırasında
        // istemciyle en başından doğru protokol üzerinde anlaşmamızı sağlar.
        config.alpn_protocols = vec![b"http/1.1".to_vec()];
        // ========================= DÜZELTME BİTİŞİ =========================

        let arc_config = Arc::new(config);
        cache.insert(domain.to_string(), arc_config.clone());

        Ok(arc_config)
    }
}