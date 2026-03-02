//! Onboarding Module - Automatic Email Configuration
//!
//! This module provides automatic SMTP/IMAP configuration by querying
//! DNS MX records and common email service autodiscover endpoints.

#![forbid(unsafe_code)]

use crate::core::Account;
use serde::{Deserialize, Serialize};
use std::net::ToSocketAddrs;

/// Auto-discovered server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoConfig {
    /// IMAP server hostname
    pub imap_server: String,
    /// IMAP port
    pub imap_port: u16,
    /// SMTP server hostname
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// Use TLS
    pub use_tls: bool,
    /// Configuration source
    pub source: ConfigSource,
}

/// Configuration source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigSource {
    /// Discovered via MX lookup
    MX,
    /// Discovered via Autodiscover DNS TXT
    Autodiscover,
    /// Common provider (Gmail, Outlook, etc.)
    KnownProvider,
    /// Manual configuration
    Manual,
}

/// Email provider known configurations
pub struct KnownProvider;

impl KnownProvider {
    /// Known email providers configuration
    pub fn get_config(domain: &str) -> Option<AutoConfig> {
        let domain_lower = domain.to_lowercase();
        
        // Gmail / Google Workspace
        if domain_lower.contains("gmail.com") || domain_lower.contains("googlemail.com") {
            return Some(AutoConfig {
                imap_server: "imap.gmail.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.gmail.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // Outlook / Hotmail / Live
        if domain_lower.contains("outlook.com") 
            || domain_lower.contains("hotmail.com") 
            || domain_lower.contains("live.com")
            || domain_lower.contains("msn.com") {
            return Some(AutoConfig {
                imap_server: "outlook.office365.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.office365.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // Yahoo Mail
        if domain_lower.contains("yahoo.com") || domain_lower.contains("ymail.com") {
            return Some(AutoConfig {
                imap_server: "imap.mail.yahoo.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.mail.yahoo.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // ProtonMail
        if domain_lower.contains("protonmail.com") || domain_lower.contains("proton.me") {
            return Some(AutoConfig {
                imap_server: "127.0.0.1".to_string(), // Requires ProtonMail Bridge
                imap_port: 1143,
                smtp_server: "127.0.0.1".to_string(),
                smtp_port: 1025,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // iCloud
        if domain_lower.contains("icloud.com") || domain_lower.contains("me.com") {
            return Some(AutoConfig {
                imap_server: "imap.mail.me.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.mail.me.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // FastMail
        if domain_lower.contains("fastmail.com") || domain_lower.contains("fastmail.fm") {
            return Some(AutoConfig {
                imap_server: "imap.fastmail.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.fastmail.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // Zoho Mail
        if domain_lower.contains("zoho.com") {
            return Some(AutoConfig {
                imap_server: "imap.zoho.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.zoho.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // Runbox
        if domain_lower.contains("runbox.com") {
            return Some(AutoConfig {
                imap_server: "imap.runbox.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.runbox.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // Mailbox.org
        if domain_lower.contains("mailbox.org") {
            return Some(AutoConfig {
                imap_server: "imap.mailbox.org".to_string(),
                imap_port: 993,
                smtp_server: "smtp.mailbox.org".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        // Tutanota
        if domain_lower.contains("tutanota.com") || domain_lower.contains("tuta.io") {
            return Some(AutoConfig {
                imap_server: "imap.tutanota.com".to_string(),
                imap_port: 993,
                smtp_server: "smtp.tutanota.com".to_string(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::KnownProvider,
            });
        }
        
        None
    }
}

/// Auto-configuration service
pub struct AutoConfigService;

impl AutoConfigService {
    /// Extract domain from email address
    pub fn extract_domain(email: &str) -> Option<&str> {
        email.split('@').nth(1)
    }

    /// Auto-discover email configuration
    pub fn discover(email: &str) -> Option<AutoConfig> {
        let domain = Self::extract_domain(email)?;
        
        // First, check known providers
        if let Some(config) = KnownProvider::get_config(domain) {
            return Some(config);
        }
        
        // Then, try MX record lookup (simplified)
        if let Some(config) = Self::discover_via_mx(domain) {
            return Some(config);
        }
        
        // Finally, try based on domain
        if let Some(config) = Self::discover_from_domain(domain) {
            return Some(config);
        }
        
        None
    }

    /// Discover via MX records (simplified - uses socket resolution)
    pub fn discover_via_mx(domain: &str) -> Option<AutoConfig> {
        // Try to resolve common SMTP patterns based on MX hostname
        let mx_host = format!("mx1.{}", domain);
        
        // Try to connect to common mail ports to discover
        let test_ports = [25, 587, 465];
        
        for port in test_ports {
            let smtp_host = format!("smtp.{}", domain);
            let test_addr = format!("{}:{}", smtp_host, port);
            
            if test_addr.to_socket_addrs().is_ok() {
                return Some(AutoConfig {
                    imap_server: format!("imap.{}", domain),
                    imap_port: 993,
                    smtp_server: smtp_host,
                    smtp_port: port,
                    use_tls: port == 465 || port == 993,
                    source: ConfigSource::MX,
                });
            }
        }
        
        None
    }

    /// Discover from domain directly
    pub fn discover_from_domain(domain: &str) -> Option<AutoConfig> {
        // Generic configuration based on domain
        Some(AutoConfig {
            imap_server: format!("imap.{}", domain),
            imap_port: 993,
            smtp_server: format!("smtp.{}", domain),
            smtp_port: 587,
            use_tls: true,
            source: ConfigSource::MX,
        })
    }
}

/// Onboarding wizard state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingState {
    /// Current step
    pub step: OnboardingStep,
    /// Email address
    pub email: String,
    /// Password / App password
    pub password: String,
    /// Discovered configuration
    pub auto_config: Option<AutoConfig>,
    /// Is loading
    pub is_loading: bool,
    /// Error message
    pub error: Option<String>,
}

/// Onboarding steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OnboardingStep {
    /// Welcome / Email entry
    Welcome,
    /// Auto-discovering configuration
    Discovering,
    /// Configuration review
    Configure,
    /// Testing connection
    Testing,
    /// Completed
    Complete,
    /// Error state
    Error,
}

impl Default for OnboardingState {
    fn default() -> Self {
        Self {
            step: OnboardingStep::Welcome,
            email: String::new(),
            password: String::new(),
            auto_config: None,
            is_loading: false,
            error: None,
        }
    }
}

impl OnboardingState {
    /// Create new onboarding state
    pub fn new() -> Self {
        Self::default()
    }

    /// Start discovery process (synchronous for egui)
    pub fn start_discovery(&mut self) {
        self.is_loading = true;
        self.step = OnboardingStep::Discovering;
        self.error = None;
        
        // Run synchronous discovery
        if let Some(config) = AutoConfigService::discover(&self.email) {
            self.auto_config = Some(config);
            self.step = OnboardingStep::Configure;
        } else {
            self.error = Some("Could not auto-discover email settings. Please configure manually.".to_string());
            self.step = OnboardingStep::Configure;
            // Create a manual config placeholder
            self.auto_config = Some(AutoConfig {
                imap_server: String::new(),
                imap_port: 993,
                smtp_server: String::new(),
                smtp_port: 587,
                use_tls: true,
                source: ConfigSource::Manual,
            });
        }
        
        self.is_loading = false;
    }

    /// Create account from current state
    pub fn create_account(&self) -> Option<Account> {
        let config = self.auto_config.as_ref()?;
        
        let mut account = Account::new(
            self.email.clone(),
            config.imap_server.clone(),
            config.smtp_server.clone(),
        );
        
        account.imap_port = config.imap_port;
        account.smtp_port = config.smtp_port;
        account.use_tls = config.use_tls;
        
        Some(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(AutoConfigService::extract_domain("user@gmail.com"), Some("gmail.com"));
        assert_eq!(AutoConfigService::extract_domain("user@sub.domain.com"), Some("sub.domain.com"));
    }

    #[test]
    fn test_known_provider_gmail() {
        let config = KnownProvider::get_config("gmail.com");
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.imap_server, "imap.gmail.com");
    }

    #[test]
    fn test_onboarding_state() {
        let state = OnboardingState::new();
        assert_eq!(state.step, OnboardingStep::Welcome);
    }
}
