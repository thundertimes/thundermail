//! AI Module - Private Intelligence Layer
//!
//! This module provides the MailAgent trait and implementations for:
//! - Local inference via Ollama
//! - Private cloud inference via Venice AI
//! - Email categorization

#![forbid(unsafe_code)]

mod ollama;
mod venice;
mod categorizer;

pub use ollama::OllamaProvider;
pub use venice::VeniceProvider;
pub use categorizer::Categorizer;

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};

/// MailAgent trait for AI providers
pub trait MailAgent: Send + Sync {
    /// Categorize an email
    fn categorize(&self, email: &EmailContent) -> Result<CategorizationResult>;
    
    /// Generate a reply suggestion
    fn suggest_reply(&self, email: &EmailContent) -> Result<String>;
    
    /// Summarize email content
    fn summarize(&self, email: &EmailContent) -> Result<String>;
}

/// Email content for AI processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailContent {
    /// Email subject (should be redacted for privacy)
    pub subject: String,
    /// Email sender
    pub from: String,
    /// Email body (plain text)
    pub body: String,
    /// Email snippet (first 200 chars)
    pub snippet: String,
}

/// Categorization result from AI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmailCategory {
    /// Primary inbox emails
    Primary,
    /// Promotional emails
    Promotions,
    /// Social media notifications
    Social,
    /// Newsletter/updates
    Updates,
    /// Spam (should be rare with good filtering)
    Spam,
    /// Trash
    Trash,
}

/// Categorization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorizationResult {
    /// The assigned category
    pub category: EmailCategory,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Suggested labels
    pub labels: Vec<String>,
}

impl Default for CategorizationResult {
    fn default() -> Self {
        Self {
            category: EmailCategory::Primary,
            confidence: 1.0,
            labels: vec![],
        }
    }
}

/// AI Provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiProvider {
    /// Local Ollama instance
    Ollama,
    /// Venice AI cloud service
    Venice,
}

impl Default for AiProvider {
    fn default() -> Self {
        Self::Ollama
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_category_serialization() {
        let category = EmailCategory::Primary;
        let serialized = serde_json::to_string(&category).unwrap();
        assert_eq!(serialized, "\"primary\"");
        
        let deserialized: EmailCategory = serde_json::from_str("\"social\"").unwrap();
        assert_eq!(deserialized, EmailCategory::Social);
    }

    #[test]
    fn test_categorization_result_default() {
        let result = CategorizationResult::default();
        assert_eq!(result.category, EmailCategory::Primary);
        assert_eq!(result.confidence, 1.0);
        assert!(result.labels.is_empty());
    }
}
