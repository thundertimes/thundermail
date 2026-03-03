//! Theme Module - Sovereign Visual Styles
//!
//! This module provides the Thundermail theme styling.

#![forbid(unsafe_code)]

use eframe::egui;

/// Responsive breakpoints for different device sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenSize {
    /// Mobile: < 600px
    Mobile,
    /// Tablet: 600px - 1024px
    Tablet,
    /// Desktop: > 1024px
    Desktop,
}

impl ScreenSize {
    /// Detect screen size from context
    pub fn from_ctx(ctx: &egui::Context) -> Self {
        let screen_width = ctx.available_rect().width();
        if screen_width < 600.0 {
            Self::Mobile
        } else if screen_width < 1024.0 {
            Self::Tablet
        } else {
            Self::Desktop
        }
    }

    /// Get sidebar width for this screen size
    pub fn sidebar_width(&self) -> f32 {
        match self {
            Self::Mobile => 0.0,  // Hidden by default on mobile
            Self::Tablet => 200.0,
            Self::Desktop => 250.0,
        }
    }

    /// Get max compose width for this screen size
    pub fn compose_width(&self) -> f32 {
        match self {
            Self::Mobile => 400.0,
            Self::Tablet => 600.0,
            Self::Desktop => 800.0,
        }
    }

    /// Get search box width for this screen size
    pub fn search_width(&self) -> f32 {
        match self {
            Self::Mobile => 100.0,
            Self::Tablet => 150.0,
            Self::Desktop => 200.0,
        }
    }

    /// Should show full header labels
    pub fn show_full_labels(&self) -> bool {
        match self {
            Self::Mobile => false,
            Self::Tablet => false,
            Self::Desktop => true,
        }
    }

    /// Font size scale factor
    pub fn font_scale(&self) -> f32 {
        match self {
            Self::Mobile => 0.85,
            Self::Tablet => 0.95,
            Self::Desktop => 1.0,
        }
    }
}

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
