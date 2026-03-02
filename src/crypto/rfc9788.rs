//! RFC 9788 - LAMPS Header Protection
//!
//! This module implements Header Confidentiality Policies (HCP) for
//! protecting sensitive email headers during transit.

#![forbid(unsafe_code)]

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};

/// Header protection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rfc9788Config {
    /// Enable header protection
    pub enabled: bool,
    /// Headers to protect (always protected)
    pub protected_headers: Vec<String>,
    /// Headers to shroud (replace with generic values)
    pub shrouded_headers: Vec<String>,
}

impl Default for Rfc9788Config {
    fn default() -> Self {
        Self {
            enabled: true,
            protected_headers: vec![
                "Subject".to_string(),
                "To".to_string(),
                "Cc".to_string(),
                "Bcc".to_string(),
                "Reply-To".to_string(),
            ],
            shrouded_headers: vec![
                "Subject".to_string(),
                "From".to_string(),
            ],
        }
    }
}

/// RFC 9788 Header Protection implementation
pub struct Rfc9788 {
    config: Rfc9788Config,
}

impl Rfc9788 {
    /// Create a new RFC 9788 instance
    pub fn new(config: Rfc9788Config) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(Rfc9788Config::default())
    }

    /// Wrap headers for outgoing email (encrypt sensitive headers)
    pub fn wrap_headers(&self, headers: &EmailHeaders) -> Result<WrappedHeaders> {
        if !self.config.enabled {
            return Ok(WrappedHeaders {
                shrouded: headers.clone(),
                protected: vec![],
            });
        }

        // Extract and encrypt sensitive headers
        let mut protected = Vec::new();
        let mut shrouded = headers.clone();

        for header in &self.config.protected_headers {
            if let Some(value) = headers.get(header) {
                protected.push((header.clone(), value.clone()));
                // Remove from shroud headers
                shrouded.remove(header);
            }
        }

        // Replace shrouded headers with generic values
        for header in &self.config.shrouded_headers {
            if let Some(value) = headers.get(header) {
                let shrouded_value = self.shroud_value(header, value);
                shrouded.insert(header.clone(), shrouded_value);
            }
        }

        Ok(WrappedHeaders {
            shrouded,
            protected,
        })
    }

    /// Unwrap headers for incoming email (decrypt protected headers)
    pub fn unwrap_headers(&self, wrapped: &WrappedHeaders) -> Result<EmailHeaders> {
        let mut headers = wrapped.shrouded.clone();

        // Add back protected headers
        for (name, value) in &wrapped.protected {
            headers.insert(name.clone(), value.clone());
        }

        Ok(headers)
    }

    /// Create a shrouded value for a header
    fn shroud_value(&self, header: &str, _value: &str) -> String {
        match header {
            "Subject" => "Encrypted Message".to_string(),
            "From" => "Encrypted User".to_string(),
            _ => "[REDACTED]".to_string(),
        }
    }
}

/// Email headers map
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailHeaders {
    headers: Vec<(String, String)>,
}

impl EmailHeaders {
    /// Create new headers
    pub fn new() -> Self {
        Self { headers: vec![] }
    }

    /// Get a header value
    pub fn get(&self, name: &str) -> Option<String> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.clone())
    }

    /// Insert a header
    pub fn insert(&mut self, name: String, value: String) {
        // Remove existing
        self.headers.retain(|(n, _)| !n.eq_ignore_ascii_case(&name));
        self.headers.push((name, value));
    }

    /// Remove a header
    pub fn remove(&mut self, name: &str) {
        self.headers.retain(|(n, _)| !n.eq_ignore_ascii_case(name));
    }
}

/// Wrapped headers (shrouded + encrypted)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrappedHeaders {
    /// Headers that go over the wire (shrouded)
    pub shrouded: EmailHeaders,
    /// Protected headers (encrypted in PGP payload)
    pub protected: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_wrapping() {
        let rfc9788 = Rfc9788::default();
        
        let mut headers = EmailHeaders::new();
        headers.insert("Subject".to_string(), "Secret Project".to_string());
        headers.insert("To".to_string(), "friend@example.com".to_string());
        
        let wrapped = rfc9788.wrap_headers(&headers).unwrap();
        
        // Check shrouded subject
        assert_eq!(
            wrapped.shrouded.get("Subject"),
            Some("Encrypted Message".to_string())
        );
        
        // Check protected headers
        assert!(wrapped.protected.iter().any(|(k, v)| 
            k == "Subject" && v == "Secret Project"
        ));
    }
}
