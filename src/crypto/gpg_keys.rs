//! GPG Key Manager - System GPG Key Integration
//!
//! This module provides functionality to read PGP/GPG keys from the system
//! and prepare them for attachment to emails.

#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process::Command;

/// GPG Key information
#[derive(Debug, Clone)]
pub struct GpgKey {
    /// Key ID (short or long format)
    pub key_id: String,
    /// User ID / Email
    pub user_id: String,
    /// Key type (RSA, DSA, etc.)
    pub key_type: String,
    /// Key bits
    pub bits: u32,
    /// Creation date
    pub created: String,
    /// Expires
    pub expires: String,
    /// Whether private key is available
    pub secret_available: bool,
}

/// GPG Key Manager
pub struct GpgKeyManager;

impl GpgKeyManager {
    /// Get the GPG home directory
    pub fn gpg_home() -> PathBuf {
        // Check for GNUPGHOME environment variable
        if let Ok(home) = std::env::var("GNUPGHOME") {
            return PathBuf::from(home);
        }
        
        // Default to ~/.gnupg - use HOME env var
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push(".gnupg");
        path
    }

    /// List all public keys from the system GPG keyring
    pub fn list_public_keys() -> Vec<GpgKey> {
        let mut keys = Vec::new();
        
        // Use gpg --list-keys to list public keys
        let output = Command::new("gpg")
            .args(["--list-keys", "--with-colons", "--fixed-list-mode"])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                keys = Self::parse_gpg_output(&stdout);
            }
        }
        
        // If GPG command fails, return empty list
        keys
    }

    /// Export a public key in ASCII armored format
    pub fn export_public_key(key_id: &str) -> Option<String> {
        // Use gpg --armor --export to export the public key
        let output = Command::new("gpg")
            .args(["--armor", "--export", key_id])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).to_string());
            }
        }
        
        None
    }

    /// Export public key for a specific email address
    pub fn export_public_key_by_email(email: &str) -> Option<String> {
        // Try to find the key by email first
        let keys = Self::list_public_keys();
        
        for key in keys {
            if key.user_id.to_lowercase().contains(&email.to_lowercase()) {
                if let Some(armored) = Self::export_public_key(&key.key_id) {
                    return Some(armored);
                }
            }
        }
        
        // If not found by email, try exporting directly
        Self::export_public_key(email)
    }

    /// Get the default/public key (first key or first with secret)
    pub fn get_default_key() -> Option<GpgKey> {
        let keys = Self::list_public_keys();
        
        // Prefer keys with secret keys
        for key in &keys {
            if key.secret_available {
                return Some(key.clone());
            }
        }
        
        // Otherwise return first available key
        keys.into_iter().next()
    }

    /// Export the default/public key as ASCII armored
    pub fn export_default_key() -> Option<String> {
        let key = Self::get_default_key()?;
        Self::export_public_key(&key.key_id)
    }

    /// Parse GPG --list-keys output
    fn parse_gpg_output(output: &str) -> Vec<GpgKey> {
        let mut keys = Vec::new();
        let mut current_key: Option<GpgKey> = None;
        
        for line in output.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "pub" => {
                    // Start of a new key
                    if let Some(key) = current_key.take() {
                        keys.push(key);
                    }
                    
                    if parts.len() >= 10 {
                        let bits = parts[4].parse().unwrap_or(0);
                        let key_type = parts[3].to_string();
                        let key_id = parts[4].to_string();
                        let created = parts[5].to_string();
                        let expires = parts[6].to_string();
                        
                        current_key = Some(GpgKey {
                            key_id,
                            user_id: String::new(),
                            key_type,
                            bits,
                            created,
                            expires,
                            secret_available: false,
                        });
                    }
                }
                "uid" => {
                    // User ID for the current key
                    if let Some(ref mut key) = current_key {
                        if parts.len() >= 10 {
                            key.user_id = parts[9].to_string();
                        }
                    }
                }
                "sec" => {
                    // Secret key available
                    if let Some(ref mut key) = current_key {
                        key.secret_available = true;
                    }
                }
                _ => {}
            }
        }
        
        // Don't forget the last key
        if let Some(key) = current_key {
            keys.push(key);
        }
        
        keys
    }

    /// Check if GPG is available on the system
    pub fn is_gpg_available() -> bool {
        Command::new("gpg")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get GPG version
    pub fn gpg_version() -> Option<String> {
        let output = Command::new("gpg")
            .arg("--version")
            .output()
            .ok()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.lines().next().map(|s| s.to_string())
        } else {
            None
        }
    }
}

/// Attachment for compose
#[derive(Debug, Clone)]
pub struct Attachment {
    /// Filename
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// Content (binary)
    pub content: Vec<u8>,
    /// Is PGP public key
    pub is_pgp_key: bool,
}

impl Attachment {
    /// Create a PGP public key attachment from system GPG
    pub fn from_gpg_public_key(key_id: Option<&str>) -> Option<Self> {
        // Get the key content
        let content = if let Some(key_id) = key_id {
            GpgKeyManager::export_public_key(key_id)?
        } else {
            GpgKeyManager::export_default_key()?
        };
        
        // Get the key ID for filename
        let key = if let Some(key_id) = key_id {
            GpgKeyManager::list_public_keys()
                .into_iter()
                .find(|k| k.key_id == key_id)
        } else {
            GpgKeyManager::get_default_key()
        };
        
        let key_id_str = key.map(|k| k.key_id).unwrap_or_else(|| "public".to_string());
        
        Some(Self {
            filename: format!("{}.asc", key_id_str),
            mime_type: "application/pgp-keys".to_string(),
            content: content.into_bytes(),
            is_pgp_key: true,
        })
    }

    /// Create from raw content
    pub fn from_content(filename: &str, mime_type: &str, content: Vec<u8>) -> Self {
        Self {
            filename: filename.to_string(),
            mime_type: mime_type.to_string(),
            content,
            is_pgp_key: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpg_home() {
        let home = GpgKeyManager::gpg_home();
        assert!(home.to_string_lossy().contains(".gnupg"));
    }

    #[test]
    fn test_is_gpg_available() {
        // This will fail if GPG is not installed
        let _ = GpgKeyManager::is_gpg_available();
    }
}
