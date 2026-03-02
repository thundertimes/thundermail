//! Theme Module - Sovereign Visual Styles
//!
//! This module provides the Thundermail theme styling.

#![forbid(unsafe_code)]

use eframe::egui;

/// Theme type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    /// Dark theme
    Dark,
    /// Light theme
    Light,
}

impl Default for ThemeType {
    fn default() -> Self {
        Self::Dark
    }
}

/// Thundermail theme
pub struct Theme {
    theme_type: ThemeType,
}

impl Theme {
    /// Create a new theme
    pub fn new(theme_type: ThemeType) -> Self {
        Self { theme_type }
    }

    /// Create default dark theme
    pub fn default() -> Self {
        Self::new(ThemeType::Dark)
    }

    /// Apply theme to context
    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = match self.theme_type {
            ThemeType::Dark => {
                let mut v = egui::Visuals::dark();
                v.override_text_color = Some(egui::Color32::from_gray(230));
                v
            }
            ThemeType::Light => {
                let mut v = egui::Visuals::light();
                v.override_text_color = Some(egui::Color32::from_gray(20));
                v
            }
        };

        // Customize panel backgrounds
        visuals.panel_fill = match self.theme_type {
            ThemeType::Dark => egui::Color32::from_gray(30),
            ThemeType::Light => egui::Color32::from_gray(240),
        };

        ctx.set_visuals(visuals);
    }

    /// Get accent color
    pub fn accent_color(&self) -> egui::Color32 {
        // Thundermail brand color - Electric Blue
        egui::Color32::from_rgb(0, 122, 255)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::default();
        assert_eq!(theme.theme_type, ThemeType::Dark);
    }
}
