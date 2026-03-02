//! Sanitizer Module - PII Redaction & Tracking Pixel Stripping
//!
//! This module provides content sanitization to prevent privacy leaks.

#![forbid(unsafe_code)]

use crate::error::{Result, ThundermailError};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Tracking pixel patterns
    static ref TRACKING_PIXELS: Vec<Regex> = vec![
        // Common tracking domains
        Regex::new(r"https?://[^\s]*\.?(sendgrid\.net|mailchimp\.com|constantcontact\.com|hubspot\.com|marketo\.com|eloqua\.com|pardot\.com)[^\s]*").unwrap(),
        // Tracking URLs with common parameters
        Regex::new(r"(open|click|track|beacon)[^\s]*\.(gif|png|jpg)").unwrap(),
    ];
    
    // Email patterns for PII redaction
    static ref EMAIL_PATTERN: Regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    static ref PHONE_PATTERN: Regex = Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap();
    static ref SSN_PATTERN: Regex = Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap();
}

/// Sanitizer for email content
pub struct Sanitizer {
    strip_trackers: bool,
    redact_pii: bool,
}

impl Sanitizer {
    /// Create a new sanitizer
    pub fn new(strip_trackers: bool, redact_pii: bool) -> Self {
        Self {
            strip_trackers,
            redact_pii,
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(true, true)
    }

    /// Sanitize HTML content
    pub fn sanitize_html(&self, html: &str) -> String {
        let mut result = html.to_string();
        
        if self.strip_trackers {
            // Remove tracking pixels
            for pattern in TRACKING_PIXELS.iter() {
                result = pattern.replace_all(&result, "[TRACKER REMOVED]").to_string();
            }
            
            // Remove tracking parameters from URLs
            let tracking_params = ["utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content", "mc_eid", "mc_cid", "_hsenc", "_hsmi"];
            for param in tracking_params {
                let param_regex = Regex::new(&format!(r"[\?&]({})=[^&]*", param)).unwrap();
                result = param_regex.replace_all(&result, "").to_string();
            }
        }
        
        if self.redact_pii {
            // Redact email addresses
            result = EMAIL_PATTERN.replace_all(&result, "[EMAIL REDACTED]").to_string();
            // Redact phone numbers
            result = PHONE_PATTERN.replace_all(&result, "[PHONE REDACTED]").to_string();
            // Redact SSN
            result = SSN_PATTERN.replace_all(&result, "[SSN REDACTED]").to_string();
        }
        
        result
    }

    /// Sanitize plain text content
    pub fn sanitize_text(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        if self.redact_pii {
            // Redact email addresses
            result = EMAIL_PATTERN.replace_all(&result, "[EMAIL REDACTED]").to_string();
            // Redact phone numbers
            result = PHONE_PATTERN.replace_all(&result, "[PHONE REDACTED]").to_string();
            // Redact SSN
            result = SSN_PATTERN.replace_all(&result, "[SSN REDACTED]").to_string();
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_email() {
        let sanitizer = Sanitizer::default();
        
        let text = "Contact me at john@example.com";
        let result = sanitizer.sanitize_text(text);
        
        assert!(result.contains("[EMAIL REDACTED]"));
        assert!(!result.contains("john@example.com"));
    }

    #[test]
    fn test_sanitize_phone() {
        let sanitizer = Sanitizer::default();
        
        let text = "Call me at 555-123-4567";
        let result = sanitizer.sanitize_text(text);
        
        assert!(result.contains("[PHONE REDACTED]"));
    }

    #[test]
    fn test_strip_trackers() {
        let sanitizer = Sanitizer::default();
        
        let html = r#"<img src="https://track.sendgrid.net/open.gif?u=123">"#;
        let result = sanitizer.sanitize_html(html);
        
        assert!(result.contains("[TRACKER REMOVED]"));
    }
}
