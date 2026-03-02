//! Thundermail - A Sovereign Email Client Library
//!
//! This library provides the core functionality for Thundermail,
//! a privacy-first email client with RFC 9788 header protection,
//! forward secrecy, and local-first AI intelligence.
//!
//! # Architecture
//!
//! The library is organized into several key modules:
//!
//! - [`ai`] - Private Intelligence Layer (Ollama/Venice AI)
//! - [`core`] - IMAP/SMTP state machines and account lifecycle
//! - [`crypto`] - RFC 9788 wrapping, PGP signing/encryption
//! - [`db`] - Encrypted local storage (SQLite + Tantivy)
//! - [`net`] - SOCKS5/Tor proxy and TLS configuration
//! - [`privacy`] - Sanitization, header masking, UTC enforcement
//! - [`ui`] - Native immediate-mode GUI (egui)

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod ai;
pub mod core;
pub mod crypto;
pub mod db;
pub mod net;
pub mod privacy;
pub mod ui;

// Re-export commonly used types
pub use error::{Result, ThundermailError};

/// Thundermail error types
pub mod error {
    use thiserror::Error;

    /// Main Thundermail error type
    #[derive(Error, Debug)]
    pub enum ThundermailError {
        #[error("IMAP error: {0}")]
        Imap(String),
        
        #[error("SMTP error: {0}")]
        Smtp(String),
        
        #[error("Cryptography error: {0}")]
        Crypto(String),
        
        #[error("Database error: {0}")]
        Database(String),
        
        #[error("Network error: {0}")]
        Network(String),
        
        #[error("Privacy error: {0}")]
        Privacy(String),
        
        #[error("AI error: {0}")]
        Ai(String),
        
        #[error("Configuration error: {0}")]
        Config(String),
        
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        
        #[error("Serialization error: {0}")]
        Serde(#[from] serde_json::Error),
    }

    /// Alias for Result type
    pub type Result<T> = std::result::Result<T, ThundermailError>;
}

/// Thundermail version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application metadata
pub mod metadata {
    use serde::Serialize;

    /// Application information
    #[derive(Serialize)]
    pub struct AppInfo {
        /// Application name
        pub name: &'static str,
        /// Application version
        pub version: &'static str,
        /// Git commit hash (if available)
        pub commit: Option<&'static str>,
        /// Build timestamp
        pub build_timestamp: Option<&'static str>,
    }

    /// Get application information
    pub fn get_app_info() -> AppInfo {
        AppInfo {
            name: "Thundermail",
            version: env!("CARGO_PKG_VERSION"),
            commit: option_env!("GIT_HASH"),
            build_timestamp: option_env!("BUILD_TIMESTAMP"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_app_info() {
        let info = metadata::get_app_info();
        assert_eq!(info.name, "Thundermail");
        assert!(!info.version.is_empty());
    }
}
