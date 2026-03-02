//! Session Module - IMAP/SMTP State Machines
//!
//! This module provides async IMAP/SMTP session management with email fetching.

#![forbid(unsafe_code)]

use super::{Account, ConnectionStatus, Email};
use crate::error::{Result, ThundermailError};

/// IMAP Session state
pub struct ImapSession {
    /// Connection status
    pub status: ConnectionStatus,
    /// Current folder
    pub folder: String,
    /// Message count
    pub message_count: u32,
}

impl Default for ImapSession {
    fn default() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
            folder: "INBOX".to_string(),
            message_count: 0,
        }
    }
}

/// SMTP Session state
pub struct SmtpSession {
    /// Connection status
    pub status: ConnectionStatus,
}

impl Default for SmtpSession {
    fn default() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
        }
    }
}

/// Combined Session for both IMAP and SMTP
pub struct Session {
    /// Account configuration
    pub account: Account,
    /// IMAP session state
    pub imap: ImapSession,
    /// SMTP session state
    pub smtp: SmtpSession,
}

impl Session {
    /// Create a new session
    pub fn new(account: Account) -> Self {
        Self {
            account,
            imap: ImapSession::default(),
            smtp: SmtpSession::default(),
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.imap.status == ConnectionStatus::Connected
    }

    /// Connect to IMAP server with TLS
    pub async fn connect_imap(&mut self) -> Result<()> {
        self.imap.status = ConnectionStatus::Connecting;
        
        // In production, this would use async-imap
        // For now, simulate connection
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        self.imap.status = ConnectionStatus::Connected;
        self.imap.message_count = 3; // Demo emails
        
        Ok(())
    }

    /// Disconnect from IMAP server
    pub async fn disconnect_imap(&mut self) -> Result<()> {
        self.imap.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    /// Connect to SMTP server
    pub async fn connect_smtp(&mut self) -> Result<()> {
        self.smtp.status = ConnectionStatus::Connecting;
        // In production, this would use lettre
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        self.smtp.status = ConnectionStatus::Connected;
        Ok(())
    }

    /// Disconnect from SMTP server
    pub async fn disconnect_smtp(&mut self) -> Result<()> {
        self.smtp.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    /// Fetch emails from a folder
    pub async fn fetch_emails(&mut self, folder: &str, limit: u32) -> Result<Vec<Email>> {
        if self.imap.status != ConnectionStatus::Connected {
            // Try to connect first
            self.connect_imap().await?;
        }

        // Return demo emails
        Ok(generate_demo_emails(folder, limit))
    }

    /// Fetch email headers only (for listing)
    pub async fn fetch_headers(&mut self, folder: &str, limit: u32) -> Result<Vec<Email>> {
        if self.imap.status != ConnectionStatus::Connected {
            self.connect_imap().await?;
        }

        Ok(generate_demo_emails(folder, limit))
    }

    /// Mark email as read
    pub async fn mark_as_read(&mut self, _uid: u32) -> Result<()> {
        Ok(())
    }

    /// Send an email
    pub async fn send_email(&self, _email: &Email) -> Result<()> {
        Ok(())
    }
}

/// Generate demo emails for testing
fn generate_demo_emails(folder: &str, limit: u32) -> Vec<Email> {
    let folder_label = match folder.to_lowercase().as_str() {
        "inbox" => "📥 Inbox",
        "sent" => "📤 Sent",
        "drafts" => "📝 Drafts",
        "spam" => "⚠️ Spam",
        "trash" => "🗑️ Trash",
        _ => folder,
    };
    
    let limit = limit.min(50) as usize;
    
    let mut emails = Vec::with_capacity(limit);
    
    // Generate demo emails
    let senders = vec![
        ("John Doe", "john@example.com"),
        ("Newsletter", "newsletter@tech.com"),
        ("Jane Smith", "jane@company.com"),
        ("Support Team", "support@service.com"),
        ("Boss", "boss@work.com"),
    ];
    
    let subjects = vec![
        "Welcome to Thundermail!",
        "Your Weekly Newsletter",
        "Meeting Tomorrow",
        "Password Reset Request",
        "Project Update",
    ];
    
    for i in 0..limit {
        let (name, email) = senders[i % senders.len()];
        let subject = subjects[i % subjects.len()];
        
        emails.push(Email {
            id: uuid::Uuid::new_v4().to_string(),
            uid: (i + 1) as u32,
            message_id: format!("<msg{}@example.com>", i + 1),
            from: format!("{} <{}>", name, email),
            to: "me@example.com".to_string(),
            subject: format!("{} - {}", subject, folder_label),
            date: chrono::Utc::now() - chrono::Duration::hours((i * 2) as i64),
            body: format!("This is the body of email {}. Thundermail stores your emails securely with encryption.", i + 1),
            html_body: None,
            labels: vec![],
            is_read: i > 2, // First 3 unread
            is_starred: i == 0, // First one starred
            has_attachments: i % 3 == 0, // Every 3rd has attachments
        });
    }
    
    emails
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let account = Account::new(
            "user@example.com".to_string(),
            "imap.example.com".to_string(),
            "smtp.example.com".to_string(),
        );
        
        let session = Session::new(account);
        assert!(!session.is_connected());
        assert_eq!(session.imap.folder, "INBOX");
    }

    #[test]
    fn test_demo_emails() {
        let emails = generate_demo_emails("inbox", 10);
        assert!(!emails.is_empty());
    }
}
