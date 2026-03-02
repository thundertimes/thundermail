//! Core Module - The Mail Engine
//!
//! This module provides IMAP/SMTP state machines and account lifecycle management.

#![forbid(unsafe_code)]

mod session;
mod labels;
mod worker;

pub use session::Session;
pub use labels::Labels;
pub use worker::Worker;

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account ID
    pub id: String,
    /// Display name
    pub display_name: String,
    /// Email address
    pub email: String,
    /// IMAP server
    pub imap_server: String,
    /// IMAP port
    pub imap_port: u16,
    /// SMTP server
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// Use TLS
    pub use_tls: bool,
    /// OAuth2 token (if using OAuth)
    pub oauth_token: Option<String>,
}

impl Account {
    /// Create a new account
    pub fn new(
        email: String,
        imap_server: String,
        smtp_server: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            display_name: email.split('@').next().unwrap_or("User").to_string(),
            email,
            imap_server,
            imap_port: 993,
            smtp_server,
            smtp_port: 587,
            use_tls: true,
            oauth_token: None,
        }
    }
}

/// Email message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// Unique message ID
    pub id: String,
    /// IMAP UID
    pub uid: u32,
    /// Message-ID header
    pub message_id: String,
    /// From header
    pub from: String,
    /// To header
    pub to: String,
    /// Subject
    pub subject: String,
    /// Date
    pub date: chrono::DateTime<chrono::Utc>,
    /// Body (plain text)
    pub body: String,
    /// HTML body
    pub html_body: Option<String>,
    /// Labels
    pub labels: Vec<String>,
    /// Is read
    pub is_read: bool,
    /// Is starred
    pub is_starred: bool,
    /// Has attachments
    pub has_attachments: bool,
}

impl Email {
    /// Create a new email
    pub fn new(uid: u32, message_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            uid,
            message_id,
            from: String::new(),
            to: String::new(),
            subject: String::new(),
            date: chrono::Utc::now(),
            body: String::new(),
            html_body: None,
            labels: vec![],
            is_read: false,
            is_starred: false,
            has_attachments: false,
        }
    }
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    /// Not connected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected and authenticated
    Connected,
    /// Error state
    Error,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_creation() {
        let account = Account::new(
            "user@example.com".to_string(),
            "imap.example.com".to_string(),
            "smtp.example.com".to_string(),
        );
        
        assert_eq!(account.email, "user@example.com");
        assert_eq!(account.imap_port, 993);
        assert!(account.use_tls);
    }

    #[test]
    fn test_email_creation() {
        let email = Email::new(1, "<test@example.com>".to_string());
        assert_eq!(email.uid, 1);
        assert!(!email.is_read);
        assert!(!email.is_starred);
    }
}
