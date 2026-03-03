//! UI Module - Native Immediate-Mode GUI (egui)
//!
//! This module provides the egui-based user interface for Thundermail.

#![forbid(unsafe_code)]

mod app;
mod theme;
mod sidebar;
mod onboarding;

pub use app::ThundermailApp;
pub use theme::{Theme, ScreenSize};
pub use sidebar::{Sidebar, FolderItem, FolderType};
pub use onboarding::{OnboardingState, OnboardingStep, AutoConfig, AutoConfigService, ConfigSource};

use crate::error::{Result, ThundermailError};
use serde::{Deserialize, Serialize};

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme
    pub theme: String,
    /// Show sidebar
    pub show_sidebar: bool,
    /// Font size
    pub font_size: f32,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            show_sidebar: true,
            font_size: 14.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_config_default() {
        let config = UiConfig::default();
        assert_eq!(config.theme, "dark");
    }
}
