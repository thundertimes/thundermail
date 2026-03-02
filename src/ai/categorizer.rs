//! Email Categorizer - AI-Powered Email Sorting
//!
//! This module provides automatic categorization of emails into
//! tabs like Promotions, Social, Updates using local context analysis.

#![forbid(unsafe_code)]

use super::{CategorizationResult, EmailCategory, EmailContent, MailAgent};
use crate::error::{Result, ThundermailError};

/// Categorizer for sorting emails into tabs
pub struct Categorizer<M: MailAgent> {
    provider: M,
}

impl<M: MailAgent> Categorizer<M> {
    /// Create a new categorizer with the given provider
    pub fn new(provider: M) -> Self {
        Self { provider }
    }

    /// Categorize an email based on content analysis
    pub fn categorize(&self, email: &EmailContent) -> Result<CategorizationResult> {
        self.provider.categorize(email)
    }

    /// Get suggested labels for an email
    pub fn get_labels(&self, email: &EmailContent) -> Result<Vec<String>> {
        let result = self.provider.categorize(email)?;
        Ok(result.labels)
    }
}

/// Simple rule-based categorizer (fallback when AI is unavailable)
pub struct RuleBasedCategorizer;

impl RuleBasedCategorizer {
    /// Categorize based on simple rules
    pub fn categorize(email: &EmailContent) -> CategorizationResult {
        let subject_lower = email.subject.to_lowercase();
        let body_lower = email.body.to_lowercase();
        let from_lower = email.from.to_lowercase();

        // Check for promotional keywords
        let promo_keywords = ["deal", "sale", "discount", "offer", "free", "limited time"];
        if promo_keywords.iter().any(|k| subject_lower.contains(k) || body_lower.contains(k)) {
            return CategorizationResult {
                category: EmailCategory::Promotions,
                confidence: 0.8,
                labels: vec!["promotional".to_string()],
            };
        }

        // Check for social keywords
        let social_keywords = ["facebook", "twitter", "linkedin", "instagram", "social"];
        if social_keywords.iter().any(|k| from_lower.contains(k) || body_lower.contains(k)) {
            return CategorizationResult {
                category: EmailCategory::Social,
                confidence: 0.8,
                labels: vec!["social".to_string()],
            };
        }

        // Check for newsletter/update keywords
        let update_keywords = ["newsletter", "weekly", "monthly", "digest", "update"];
        if update_keywords.iter().any(|k| subject_lower.contains(k) || body_lower.contains(k)) {
            return CategorizationResult {
                category: EmailCategory::Updates,
                confidence: 0.7,
                labels: vec!["newsletter".to_string()],
            };
        }

        // Default to primary
        CategorizationResult::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_based_promotions() {
        let email = EmailContent {
            subject: "Big Sale Today!".to_string(),
            from: "deals@shop.com".to_string(),
            body: "Get 50% off today only!".to_string(),
            snippet: "Get 50% off today only!".to_string(),
        };
        
        let result = RuleBasedCategorizer::categorize(&email);
        assert_eq!(result.category, EmailCategory::Promotions);
    }

    #[test]
    fn test_rule_based_social() {
        let email = EmailContent {
            subject: "Someone mentioned you".to_string(),
            from: "notification@twitter.com".to_string(),
            body: "Someone mentioned you on Twitter".to_string(),
            snippet: "Someone mentioned you on Twitter".to_string(),
        };
        
        let result = RuleBasedCategorizer::categorize(&email);
        assert_eq!(result.category, EmailCategory::Social);
    }

    #[test]
    fn test_rule_based_primary() {
        let email = EmailContent {
            subject: "Project update".to_string(),
            from: "colleague@company.com".to_string(),
            body: "Let's schedule a meeting".to_string(),
            snippet: "Let's schedule a meeting".to_string(),
        };
        
        let result = RuleBasedCategorizer::categorize(&email);
        assert_eq!(result.category, EmailCategory::Primary);
    }
}
