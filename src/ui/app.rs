//! App Module - Main eframe Application
//!
//! This module provides the main application UI using egui/eframe.

#![forbid(unsafe_code)]

use eframe::egui;
use super::Theme;

/// Main Thundermail application
pub struct ThundermailApp {
    /// Current view
    view: AppView,
    /// Theme
    theme: Theme,
}

/// Application views
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppView {
    /// Inbox view
    Inbox,
    /// Compose view
    Compose,
    /// Settings view
    Settings,
}

impl Default for AppView {
    fn default() -> Self {
        Self::Inbox
    }
}

impl ThundermailApp {
    /// Create a new application
    pub fn new() -> Self {
        Self {
            view: AppView::Inbox,
            theme: Theme::default(),
        }
    }
}

impl eframe::App for ThundermailApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.theme.apply(ctx);

        // Show top bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ Thundermail");
                ui.separator();
                
                if ui.button("Inbox").clicked() {
                    self.view = AppView::Inbox;
                }
                if ui.button("Compose").clicked() {
                    self.view = AppView::Compose;
                }
                if ui.button("Settings").clicked() {
                    self.view = AppView::Settings;
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Sovereign Mode Active 🔒");
                });
            });
        });

        // Show main content
        egui::CentralPanel::default().show(ctx, |_ui| {
            match self.view {
                AppView::Inbox => {
                    // Placeholder for inbox
                    egui::ScrollArea::vertical().show(_ui, |ui| {
                        ui.heading("Inbox");
                        ui.label("No emails yet.");
                        ui.label("Thundermail is ready to receive your sovereign communications.");
                    });
                }
                AppView::Compose => {
                    ui.heading("Compose");
                    ui.label("Compose new email...");
                }
                AppView::Settings => {
                    ui.heading("Settings");
                    ui.label("Configure Thundermail...");
                }
            }
        });
    }
}

impl Default for ThundermailApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = ThundermailApp::new();
        assert_eq!(app.view, AppView::Inbox);
    }
}
