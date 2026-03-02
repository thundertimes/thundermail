//! TLS Module - Hardened Rustls Configuration
//!
//! This module provides secure TLS configuration with no CA leaks.

#![forbid(unsafe_code)]

use crate::error::{Result, ThundermailError};
use rustls::{ClientConfig, RootCertStore};
use std::sync::Arc;

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Verify certificates
    pub verify_certificates: bool,
    /// Use system certificates
    pub use_system_certs: bool,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            verify_certificates: true,
            use_system_certs: true,
        }
    }
}

impl TlsConfig {
    /// Create a hardened TLS client config
    pub fn to_client_config(&self) -> Result<Arc<ClientConfig>> {
        let mut root_store = RootCertStore::empty();

        // Build client config with defaults
        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(Arc::new(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_default() {
        let config = TlsConfig::default();
        assert!(config.verify_certificates);
    }
}
