//! DB Module - Encrypted Local Storage
//!
//! This module provides SQLite (SQLx) for metadata storage and Tantivy
//! for encrypted full-text search.

#![forbid(unsafe_code)]

mod schema;
mod search;

pub use schema::Database;
pub use search::SearchIndex;

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    /// Database file path
    pub path: String,
    /// Enable encryption
    pub encrypted: bool,
    /// Encryption key (should be from system keyring)
    pub encryption_key: Option<String>,
}

impl Default for DbConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            path: format!("{}/.local/share/thundermail/thundermail.db", home),
            encrypted: true,
            encryption_key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_config_default() {
        let config = DbConfig::default();
        assert!(config.encrypted);
    }
}
