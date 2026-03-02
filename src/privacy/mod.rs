//! Privacy Module - The Sanitization Gatekeeper
//!
//! This module provides PII redaction, tracking pixel stripping, and
//! MUA fingerprint masking for privacy protection.

#![forbid(unsafe_code)]

mod sanitizer;
mod headers;

pub use sanitizer::Sanitizer;
pub use headers::HeaderMask;

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Enable privacy features
    pub enabled: bool,
    /// Strip tracking pixels
    pub strip_trackers: bool,
    /// Redact PII
    pub redact_pii: bool,
    /// Mask MUA headers
    pub mask_mua: bool,
    /// Force UTC dates
    pub force_utc: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strip_trackers: true,
            redact_pii: true,
            mask_mua: true,
            force_utc: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_config_default() {
        let config = PrivacyConfig::default();
        assert!(config.enabled);
        assert!(config.strip_trackers);
        assert!(config.redact_pii);
    }
}
