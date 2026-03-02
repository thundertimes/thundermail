//! TLS Module - Hardened Rustls Configuration
//!
//! This module provides secure TLS configuration with no CA leaks.

#![forbid(unsafe_code)]

use crate::error::{Result, ThundermailError};
use rustls::{ClientConfig, RootCertStore, ServerName};
use std::sync::Arc;

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Minimum TLS version
    pub min_version: rustls::Version,
    /// Maximum TLS version
    pub max_version: rustls::Version,
    /// Verify certificates
    pub verify_certificates: bool,
    /// Use system certificates
    pub use_system_certs: bool,
    /// Custom root certificates
    pub custom_certs: Vec<Vec<u8>>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            min_version: rustls::Version::TLS13,
            max_version: rustls::Version::TLS13,
            verify_certificates: true,
            use_system_certs: true,
            custom_certs: vec![],
        }
    }
}

impl TlsConfig {
    /// Create a hardened TLS client config
    pub fn to_client_config(&self) -> Result<Arc<ClientConfig>> {
        let mut root_store = RootCertStore::empty();

        // Load system certificates if enabled
        if self.use_system_certs {
            #[cfg(feature = "std")]
            {
                let (added, skipped) = root_store.extend_from_env();
                tracing::debug!(
                    "Loaded {} system certificates, {} skipped",
                    added,
                    skipped
                );
            }
        }

        // Add custom certificates
        for cert_pem in &self.custom_certs {
            if let Ok(cert) = rustls::pem::parse(cert_pem) {
                root_store.add(&rustls::Certificate(cert)).ok();
            }
        }

        // Build client config
        let mut config = ClientConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_protocol_versions(&[self.min_version, self.max_version])?
            .custom_root_certificates(root_store)
            .with_no_client_auth();

        // Disable certificate verification if requested (dangerous!)
        if !self.verify_certificates {
            config.dangerous().set_certificate_verifier(Arc::new(NoVerifier));
        }

        Ok(Arc::new(config))
    }
}

/// No-op certificate verifier (dangerous!)
struct NoVerifier;

impl rustls::client::danger::ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_default() {
        let config = TlsConfig::default();
        assert!(config.verify_certificates);
        assert!(config.use_system_certs);
    }
}
