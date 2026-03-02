//! Headers Module - MUA Fingerprint Masking
//!
//! This module provides MUA (Mail User Agent) header masking to prevent
//! fingerprinting and enhance privacy.

#![forbid(unsafe_code)]

use crate::error::{Result, ThundermailError};
use std::collections::HashMap;

/// Header mask configuration
#[derive(Debug, Clone)]
pub struct HeaderMaskConfig {
    /// Mask User-Agent header
    pub mask_user_agent: bool,
    /// Mask X-Mailer header
    pub mask_x_mailer: bool,
    /// Mask X-Mua header
    pub mask_x_mua: bool,
    /// Custom masked values
    pub custom_masks: HashMap<String, String>,
}

impl Default for HeaderMaskConfig {
    fn default() -> Self {
        Self {
            mask_user_agent: true,
            mask_x_mailer: true,
            mask_x_mua: true,
            custom_masks: HashMap::new(),
        }
    }
}

/// Header masker
pub struct HeaderMask {
    config: HeaderMaskConfig,
}

impl HeaderMask {
    /// Create a new header masker
    pub fn new(config: HeaderMaskConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(HeaderMaskConfig::default())
    }

    /// Mask outgoing headers
    pub fn mask_outgoing(&self, headers: &mut HashMap<String, String>) {
        // Mask User-Agent
        if self.config.mask_user_agent {
            headers.insert(
                "User-Agent".to_string(),
                self.config
                    .custom_masks
                    .get("User-Agent")
                    .cloned()
                    .unwrap_or_else(|| "Thundermail/0.2".to_string()),
            );
        }

        // Mask X-Mailer
        if self.config.mask_x_mailer {
            headers.insert(
                "X-Mailer".to_string(),
                self.config
                    .custom_masks
                    .get("X-Mailer")
                    .cloned()
                    .unwrap_or_else(|| "Thundermail".to_string()),
            );
        }

        // Mask X-MUA
        if self.config.mask_x_mua {
            headers.insert(
                "X-MUA".to_string(),
                self.config
                    .custom_masks
                    .get("X-MUA")
                    .cloned()
                    .unwrap_or_else(|| "Thundermail".to_string()),
            );
        }

        // Remove potentially fingerprinting headers
        let fingerprint_headers = [
            "X-Originating-IP",
            "X-Sender-IP",
            "X-Mailer",
            "X-MS-Exchange-",
        ];
        
        // Collect keys to remove
        let keys_to_remove: Vec<String> = headers
            .keys()
            .filter(|key| {
                for fp in &fingerprint_headers {
                    if key.starts_with(fp) {
                        return true;
                    }
                }
                false
            })
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            headers.remove(&key);
        }
    }

    /// Sanitize incoming headers (remove tracking)
    pub fn sanitize_incoming(&self, headers: &mut HashMap<String, String>) {
        // Remove tracking headers
        let tracking_headers = [
            "X-Google-DKIM-Signature",
            "X-Gm-Message-State",
            "X-Received",
        ];
        
        // Collect keys to remove
        let keys_to_remove: Vec<String> = headers
            .keys()
            .filter(|key| {
                for th in &tracking_headers {
                    if key.starts_with(th) {
                        return true;
                    }
                }
                false
            })
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            headers.remove(&key);
        }
    }
}

/// Default masked User-Agent
pub const DEFAULT_USER_AGENT: &str = "Thundermail/0.2";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_mask() {
        let masker = HeaderMask::default();
        let mut headers = HashMap::new();
        
        headers.insert("User-Agent".to_string(), "Thunderbird/78.0".to_string());
        
        masker.mask_outgoing(&mut headers);
        
        assert_eq!(headers.get("User-Agent"), Some(&"Thundermail/0.2".to_string()));
    }

    #[test]
    fn test_sanitize_incoming() {
        let masker = HeaderMask::default();
        let mut headers = HashMap::new();
        
        headers.insert("X-Received".to_string(), "by 2002:a05:6357:".to_string());
        
        masker.sanitize_incoming(&mut headers);
        
        assert!(!headers.contains_key("X-Received"));
    }
}
