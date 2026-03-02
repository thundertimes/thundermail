//! Ollama Provider - Local AI Inference
//!
//! This module provides integration with Ollama for local, sovereign AI inference.

#![forbid(unsafe_code)]

use super::{CategorizationResult, EmailContent, MailAgent};
use crate::error::{Result, ThundermailError};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Ollama provider configuration
#[derive(Debug, Clone)]
pub struct OllamaConfig {
    /// Ollama server URL
    pub base_url: String,
    /// Model name to use
    pub model: String,
    /// Temperature for generation
    pub temperature: f32,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model: "llama2".to_string(),
            temperature: 0.7,
        }
    }
}

/// Ollama provider for local AI inference
pub struct OllamaProvider {
    config: OllamaConfig,
    client: reqwest::Client,
}

impl OllamaProvider {
    /// Create a new Ollama provider
    pub fn new(config: OllamaConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Check if Ollama is available
    pub async fn is_available(&self) -> bool {
        let url = format!("{}/api/tags", self.config.base_url);
        self.client.get(&url).send().await.is_ok()
    }
}

impl MailAgent for OllamaProvider {
    fn categorize(&self, _email: &EmailContent) -> Result<CategorizationResult> {
        // Implementation would call Ollama API
        // For now, return default
        Ok(CategorizationResult::default())
    }

    fn suggest_reply(&self, _email: &EmailContent) -> Result<String> {
        // Implementation would call Ollama API
        Ok(String::new())
    }

    fn summarize(&self, _email: &EmailContent) -> Result<String> {
        // Implementation would call Ollama API
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_config_default() {
        let config = OllamaConfig::default();
        assert_eq!(config.base_url, "http://localhost:11434");
        assert_eq!(config.model, "llama2");
    }
}
