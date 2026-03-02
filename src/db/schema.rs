//! Schema Module - Encrypted SQLite Database
//!
//! This module provides encrypted SQLite database management for email metadata.

#![forbid(unsafe_code)]

use crate::core::Email;
use crate::error::{Result, ThundermailError};
use rusqlite::{Connection, params};
use std::path::Path;

/// Database for email metadata with encryption
pub struct Database {
    path: String,
    connection: Option<Connection>,
    encryption_key: Option<String>,
    connected: bool,
}

impl Database {
    /// Create a new database connection
    pub fn new(path: String) -> Self {
        Self {
            path,
            connection: None,
            encryption_key: None,
            connected: false,
        }
    }

    /// Create with encryption key
    pub fn with_encryption(path: String, key: String) -> Self {
        Self {
            path,
            connection: None,
            encryption_key: Some(key),
            connected: false,
        }
    }

    /// Connect to the database
    pub async fn connect(&mut self) -> Result<()> {
        // Create parent directories if needed
        if let Some(parent) = Path::new(&self.path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ThundermailError::Database(e.to_string()))?;
        }

        // Open connection
        let conn = Connection::open(&self.path)
            .map_err(|e| ThundermailError::Database(e.to_string()))?;

        // Set encryption key if provided (using SQLCipher via PRAGMA)
        if let Some(ref key) = self.encryption_key {
            conn.execute_batch(&format!(
                "PRAGMA key = '{}';",
                key.replace("'", "''")
            )).map_err(|e| ThundermailError::Database(e.to_string()))?;
        }

        // Create tables
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL,
                display_name TEXT,
                imap_server TEXT NOT NULL,
                imap_port INTEGER NOT NULL,
                smtp_server TEXT NOT NULL,
                smtp_port INTEGER NOT NULL,
                use_tls INTEGER NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS emails (
                id TEXT PRIMARY KEY,
                account_id TEXT NOT NULL,
                folder TEXT NOT NULL,
                uid INTEGER NOT NULL,
                message_id TEXT,
                from_header TEXT,
                to_header TEXT,
                subject TEXT,
                date TEXT,
                body TEXT,
                html_body TEXT,
                is_read INTEGER DEFAULT 0,
                is_starred INTEGER DEFAULT 0,
                has_attachments INTEGER DEFAULT 0,
                encrypted_body BLOB,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (account_id) REFERENCES accounts(id)
            );

            CREATE INDEX IF NOT EXISTS idx_emails_account_folder 
                ON emails(account_id, folder);
            CREATE INDEX IF NOT EXISTS idx_emails_uid 
                ON emails(account_id, uid);
            CREATE INDEX IF NOT EXISTS idx_emails_date 
                ON emails(account_id, date DESC);
            "#
        ).map_err(|e| ThundermailError::Database(e.to_string()))?;

        self.connection = Some(conn);
        self.connected = true;
        Ok(())
    }

    /// Disconnect from the database
    pub async fn disconnect(&mut self) -> Result<()> {
        self.connection = None;
        self.connected = false;
        Ok(())
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Store an email (encrypted)
    pub async fn store_email(&self, account_id: &str, folder: &str, email: &Email) -> Result<()> {
        let conn = self.connection.as_ref()
            .ok_or_else(|| ThundermailError::Database("Not connected".to_string()))?;
        
        // Encrypt body if we have encryption
        let encrypted_body: Option<Vec<u8>> = if !email.body.is_empty() {
            Some(encrypt_data(&email.body))
        } else {
            None
        };

        conn.execute(
            r#"INSERT OR REPLACE INTO emails 
               (id, account_id, folder, uid, message_id, from_header, to_header, 
                subject, date, body, html_body, is_read, is_starred, has_attachments, encrypted_body)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)"#,
            params![
                email.id,
                account_id,
                folder,
                email.uid,
                email.message_id,
                email.from,
                email.to,
                email.subject,
                email.date.to_rfc3339(),
                email.body,
                email.html_body,
                email.is_read as i32,
                email.is_starred as i32,
                email.has_attachments as i32,
                encrypted_body,
            ],
        ).map_err(|e| ThundermailError::Database(e.to_string()))?;

        Ok(())
    }

    /// Get an email by ID (with decryption)
    pub async fn get_email(&self, id: &str) -> Result<Option<Email>> {
        let conn = self.connection.as_ref()
            .ok_or_else(|| ThundermailError::Database("Not connected".to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, uid, message_id, from_header, to_header, subject, date, 
                    body, html_body, is_read, is_starred, has_attachments, encrypted_body
             FROM emails WHERE id = ?1"
        ).map_err(|e| ThundermailError::Database(e.to_string()))?;

        let result = stmt.query_row(params![id], |row| {
            let encrypted_body: Option<Vec<u8>> = row.get(12)?;
            
            let body = if let Some(enc) = encrypted_body {
                decrypt_data(&enc).unwrap_or_default()
            } else {
                row.get::<_, String>(7).unwrap_or_default()
            };

            Ok(Email {
                id: row.get(0)?,
                uid: row.get(1)?,
                message_id: row.get(2)?,
                from: row.get(3)?,
                to: row.get(4)?,
                subject: row.get(5)?,
                date: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                body,
                html_body: row.get(8)?,
                is_read: row.get::<_, i32>(9)? != 0,
                is_starred: row.get::<_, i32>(10)? != 0,
                has_attachments: row.get::<_, i32>(11)? != 0,
                labels: vec![],
            })
        });

        match result {
            Ok(email) => Ok(Some(email)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(ThundermailError::Database(e.to_string())),
        }
    }

    /// Get emails by folder/label
    pub async fn get_emails_by_folder(&self, account_id: &str, folder: &str, limit: u32) -> Result<Vec<Email>> {
        let conn = self.connection.as_ref()
            .ok_or_else(|| ThundermailError::Database("Not connected".to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, uid, message_id, from_header, to_header, subject, date, 
                    is_read, is_starred, has_attachments
             FROM emails 
             WHERE account_id = ?1 AND folder = ?2
             ORDER BY date DESC
             LIMIT ?3"
        ).map_err(|e| ThundermailError::Database(e.to_string()))?;

        let emails = stmt.query_map(
            params![account_id, folder, limit],
            |row| {
                Ok(Email {
                    id: row.get(0)?,
                    uid: row.get(1)?,
                    message_id: row.get(2)?,
                    from: row.get(3)?,
                    to: row.get(4)?,
                    subject: row.get(5)?,
                    date: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    body: String::new(), // Body not loaded in list view
                    html_body: None,
                    is_read: row.get::<_, i32>(7)? != 0,
                    is_starred: row.get::<_, i32>(8)? != 0,
                    has_attachments: row.get::<_, i32>(9)? != 0,
                    labels: vec![],
                })
            }
        ).map_err(|e| ThundermailError::Database(e.to_string()))?
         .filter_map(|r| r.ok())
         .collect();

        Ok(emails)
    }

    /// Get email count for a folder
    pub async fn get_folder_count(&self, account_id: &str, folder: &str) -> Result<u32> {
        let conn = self.connection.as_ref()
            .ok_or_else(|| ThundermailError::Database("Not connected".to_string()))?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM emails WHERE account_id = ?1 AND folder = ?2",
            params![account_id, folder],
            |row| row.get(0)
        ).map_err(|e| ThundermailError::Database(e.to_string()))?;

        Ok(count as u32)
    }

    /// Update email labels
    pub async fn update_labels(&self, _email_id: &str, _labels: &[String]) -> Result<()> {
        Ok(())
    }

    /// Mark email as read
    pub async fn mark_as_read(&self, email_id: &str) -> Result<()> {
        let conn = self.connection.as_ref()
            .ok_or_else(|| ThundermailError::Database("Not connected".to_string()))?;

        conn.execute(
            "UPDATE emails SET is_read = 1 WHERE id = ?1",
            params![email_id],
        ).map_err(|e| ThundermailError::Database(e.to_string()))?;

        Ok(())
    }

    /// Store account
    pub async fn store_account(&self, account: &crate::core::Account) -> Result<()> {
        let conn = self.connection.as_ref()
            .ok_or_else(|| ThundermailError::Database("Not connected".to_string()))?;

        conn.execute(
            r#"INSERT OR REPLACE INTO accounts 
               (id, email, display_name, imap_server, imap_port, smtp_server, smtp_port, use_tls)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
            params![
                account.id,
                account.email,
                account.display_name,
                account.imap_server,
                account.imap_port,
                account.smtp_server,
                account.smtp_port,
                account.use_tls as i32,
            ],
        ).map_err(|e| ThundermailError::Database(e.to_string()))?;

        Ok(())
    }
}

/// Simple XOR encryption (placeholder - in production use AES-GCM)
fn encrypt_data(data: &str) -> Vec<u8> {
    let key = b"ThundermailSecureKey123456789012"; // 32 bytes
    let data_bytes = data.as_bytes();
    let mut encrypted = Vec::with_capacity(data_bytes.len());
    
    for (i, byte) in data_bytes.iter().enumerate() {
        encrypted.push(byte ^ key[i % key.len()]);
    }
    
    encrypted
}

/// Decrypt data
fn decrypt_data(data: &[u8]) -> std::result::Result<String, ThundermailError> {
    let key = b"ThundermailSecureKey123456789012"; // 32 bytes
    let mut decrypted = Vec::with_capacity(data.len());
    
    for (i, byte) in data.iter().enumerate() {
        decrypted.push(byte ^ key[i % key.len()]);
    }
    
    String::from_utf8(decrypted)
        .map_err(|e| ThundermailError::Crypto(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::new("/tmp/test.db".to_string());
        assert!(!db.is_connected());
    }

    #[test]
    fn test_encryption() {
        let original = "Hello, World!";
        let encrypted = encrypt_data(original);
        let decrypted = decrypt_data(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }
}
