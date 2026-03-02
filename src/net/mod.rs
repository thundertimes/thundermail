//! Net Module - Sovereign Networking
//!
//! This module provides SOCKS5/Tor proxy routing and hardened TLS configuration.

#![forbid(unsafe_code)]

mod proxy;
mod tls;

pub use proxy::Proxy;
pub use tls::TlsConfig;

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetConfig {
    /// Enable Tor/SOCKS5
    pub use_proxy: bool,
    /// Proxy type
    pub proxy_type: ProxyType,
    /// Proxy host
    pub proxy_host: Option<String>,
    /// Proxy port
    pub proxy_port: Option<u16>,
    /// Force TLS
    pub force_tls: bool,
    /// Disable certificate verification (dangerous!)
    pub verify_certificates: bool,
}

impl Default for NetConfig {
    fn default() -> Self {
        Self {
            use_proxy: false,
            proxy_type: ProxyType::None,
            proxy_host: None,
            proxy_port: None,
            force_tls: true,
            verify_certificates: true,
        }
    }
}

/// Proxy type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyType {
    /// No proxy
    None,
    /// SOCKS5 proxy
    Socks5,
    /// Tor (SOCKS5)
    Tor,
}

impl Default for ProxyType {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_config_default() {
        let config = NetConfig::default();
        assert!(config.force_tls);
        assert!(config.verify_certificates);
    }
}
