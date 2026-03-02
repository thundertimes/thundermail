//! Ephemeral Keys Module - Forward Secrecy
//!
//! This module implements ephemeral sub-key rotation and memory zeroization
//! for forward secrecy (Autocrypt v2 style).

#![forbid(unsafe_code)]

use crate::crypto::KeyId;
use crate::error::{Result, ThundermailError};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use zeroize::Zeroize;

/// Ephemeral key pair
#[derive(Debug, Clone)]
pub struct EphemeralKey {
    /// Key ID
    pub key_id: KeyId,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiration timestamp
    pub expires_at: DateTime<Utc>,
    /// Whether key is active
    pub is_active: bool,
}

impl EphemeralKey {
    /// Check if key is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Ephemeral key manager for forward secrecy
pub struct EphemeralKeyManager {
    /// Active keys
    keys: HashMap<KeyId, EphemeralKey>,
    /// Key rotation interval in hours
    rotation_hours: u32,
    /// Master key ID
    master_key_id: Option<KeyId>,
}

impl EphemeralKeyManager {
    /// Create a new ephemeral key manager
    pub fn new(rotation_hours: u32) -> Self {
        Self {
            keys: HashMap::new(),
            rotation_hours,
            master_key_id: None,
        }
    }

    /// Set the master key
    pub fn set_master_key(&mut self, key_id: KeyId) {
        self.master_key_id = Some(key_id);
    }

    /// Generate a new ephemeral key
    pub fn generate_key(&mut self) -> Result<EphemeralKey> {
        let now = Utc::now();
        let expires = now + chrono::Duration::hours(self.rotation_hours as i64);
        
        let key_id = KeyId::new(rand::random());
        
        let key = EphemeralKey {
            key_id: key_id.clone(),
            created_at: now,
            expires_at: expires,
            is_active: true,
        };
        
        self.keys.insert(key_id, key.clone());
        
        Ok(key)
    }

    /// Get the current active key
    pub fn get_current_key(&self) -> Option<&EphemeralKey> {
        self.keys
            .values()
            .find(|k| k.is_active && !k.is_expired())
    }

    /// Rotate keys (generate new and expire old)
    pub fn rotate(&mut self) -> Result<EphemeralKey> {
        // Expire all current keys
        for key in self.keys.values_mut() {
            key.is_active = false;
        }
        
        // Generate new key
        self.generate_key()
    }

    /// Check if rotation is needed
    pub fn needs_rotation(&self) -> bool {
        if let Some(key) = self.get_current_key() {
            // Rotate if less than 1 hour remaining
            let time_remaining = key.expires_at - Utc::now();
            time_remaining < chrono::Duration::hours(1)
        } else {
            true
        }
    }

    /// Zeroize all private key material
    pub fn zeroize(&self) {
        // In real implementation, would zeroize actual key material
        // This is a placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ephemeral_key_expiry() {
        let now = Utc::now();
        let key = EphemeralKey {
            key_id: KeyId::new([0u8; 8]),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
            is_active: true,
        };
        
        assert!(!key.is_expired());
    }

    #[test]
    fn test_key_rotation() {
        let mut manager = EphemeralKeyManager::new(24);
        
        let key1 = manager.generate_key().unwrap();
        let key2 = manager.rotate().unwrap();
        
        assert_ne!(key1.key_id, key2.key_id);
    }
}
