//! Crypto Module - Hardened Cryptography
//!
//! This module provides RFC 9788 LAMPS header protection, PGP encryption,
//! and ephemeral key management with forward secrecy.

#![forbid(unsafe_code)]

mod rfc9788;
mod pgp;
mod ephemeral;

pub use rfc9788::Rfc9788;
pub use pgp::Pgp;
pub use ephemeral::EphemeralKeyManager;

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Key ID type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyId(pub [u8; 8]);

impl KeyId {
    /// Create a new key ID
    pub fn new(data: [u8; 8]) -> Self {
        Self(data)
    }
}

/// Encryption result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Encrypted payload
    pub ciphertext: Vec<u8>,
    /// Key ID used
    pub key_id: KeyId,
    /// Initialization vector
    pub iv: Vec<u8>,
}

/// Decryption result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedData {
    /// Decrypted plaintext
    pub plaintext: Vec<u8>,
    /// Whether MDC is valid
    pub mdc_valid: bool,
}

/// Cryptographic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// Use forward secrecy
    pub forward_secrecy: bool,
    /// Key rotation interval (hours)
    pub key_rotation_hours: u32,
    /// Require MDC
    pub require_mdc: bool,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            forward_secrecy: true,
            key_rotation_hours: 24,
            require_mdc: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_config_default() {
        let config = CryptoConfig::default();
        assert!(config.forward_secrecy);
        assert!(config.require_mdc);
        assert_eq!(config.key_rotation_hours, 24);
    }

    #[test]
    fn test_key_id() {
        let key_id = KeyId::new([0u8; 8]);
        assert_eq!(key_id.0.len(), 8);
    }
}
