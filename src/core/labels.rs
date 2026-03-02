//! Labels Module - Flat-DB Label Logic
//!
//! This module provides label management similar to Gmail's X-GM-LABELS system.

#![forbid(unsafe_code)]

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Label system (Gmail-style)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Labels {
    /// Set of all labels
    labels: HashSet<String>,
    /// System labels (cannot be deleted)
    system_labels: HashSet<String>,
}

impl Labels {
    /// Create a new labels container
    pub fn new() -> Self {
        let mut system_labels = HashSet::new();
        system_labels.insert("\\Inbox".to_string());
        system_labels.insert("\\Sent".to_string());
        system_labels.insert("\\Draft".to_string());
        system_labels.insert("\\Trash".to_string());
        system_labels.insert("\\Starred".to_string());
        system_labels.insert("\\Unread".to_string());
        
        Self {
            labels: system_labels.clone(),
            system_labels,
        }
    }

    /// Add a label to an email
    pub fn add_label(&mut self, label: &str) {
        self.labels.insert(label.to_string());
    }

    /// Remove a label from an email
    pub fn remove_label(&mut self, label: &str) {
        // Don't remove system labels
        if !self.system_labels.contains(label) {
            self.labels.remove(label);
        }
    }

    /// Check if email has a label
    pub fn has_label(&self, label: &str) -> bool {
        self.labels.contains(label)
    }

    /// Get all labels
    pub fn get_all(&self) -> Vec<String> {
        self.labels.iter().cloned().collect()
    }

    /// Get user labels (non-system)
    pub fn get_user_labels(&self) -> Vec<String> {
        self.labels
            .difference(&self.system_labels)
            .cloned()
            .collect()
    }
}

/// Standard system labels
pub mod system_labels {
    /// Inbox label
    pub const INBOX: &str = "\\Inbox";
    /// Sent mail label
    pub const SENT: &str = "\\Sent";
    /// Drafts label
    pub const DRAFT: &str = "\\Draft";
    /// Trash label
    pub const TRASH: &str = "\\Trash";
    /// Starred label
    pub const STARRED: &str = "\\Starred";
    /// Unread label
    pub const UNREAD: &str = "\\Unread";
    /// Important label
    pub const IMPORTANT: &str = "\\Important";
    /// Spam label
    pub const SPAM: &str = "\\Spam";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labels_creation() {
        let labels = Labels::new();
        assert!(labels.has_label("\\Inbox"));
        assert!(labels.has_label("\\Sent"));
    }

    #[test]
    fn test_add_remove_label() {
        let mut labels = Labels::new();
        labels.add_label("work");
        assert!(labels.has_label("work"));
        
        labels.remove_label("work");
        assert!(!labels.has_label("work"));
    }

    #[test]
    fn test_cannot_remove_system_label() {
        let mut labels = Labels::new();
        labels.remove_label("\\Inbox");
        assert!(labels.has_label("\\Inbox"));
    }
}
