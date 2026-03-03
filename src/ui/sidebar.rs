//! Sidebar Module - Gmail-like Navigation
//!
//! This module provides the Gmail-style sidebar with folders and labels.

#![forbid(unsafe_code)]

use eframe::egui;
use serde::{Deserialize, Serialize};
use super::ScreenSize;

/// Sidebar folder item (like Gmail's default folders)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderItem {
    /// Display name
    pub name: String,
    /// Icon (emoji or text)
    pub icon: String,
    /// Unread count
    pub unread: u32,
    /// Is selected
    pub selected: bool,
    /// Folder type
    pub folder_type: FolderType,
}

/// Folder type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FolderType {
    /// Inbox
    Inbox,
    /// Sent mail
    Sent,
    /// Drafts
    Drafts,
    /// Spam/Junk
    Spam,
    /// Trash
    Trash,
    /// Archive
    Archive,
    /// Custom label
    Custom,
}

impl FolderItem {
    /// Create a new folder item
    pub fn new(name: &str, icon: &str, folder_type: FolderType) -> Self {
        Self {
            name: name.to_string(),
            icon: icon.to_string(),
            unread: 0,
            selected: false,
            folder_type,
        }
    }

    /// Get default Gmail-style folders
    pub fn default_folders() -> Vec<Self> {
        vec![
            Self::new("Inbox", "📥", FolderType::Inbox),
            Self::new("Sent", "📤", FolderType::Sent),
            Self::new("Drafts", "📝", FolderType::Drafts),
            Self::new("Spam", "⚠️", FolderType::Spam),
            Self::new("Trash", "🗑️", FolderType::Trash),
            Self::new("Archive", "📦", FolderType::Archive),
        ]
    }
}

/// Sidebar state
pub struct Sidebar {
    /// Folder items
    pub folders: Vec<FolderItem>,
    /// Custom labels
    pub labels: Vec<FolderItem>,
    /// Is collapsed (mobile)
    pub collapsed: bool,
    /// Sidebar width
    pub width: f32,
    /// Is sidebar visible (for mobile toggle)
    pub visible: bool,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            folders: FolderItem::default_folders(),
            labels: Vec::new(),
            collapsed: false,
            width: 250.0,
            visible: true,
        }
    }
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a custom label
    pub fn add_label(&mut self, name: &str, icon: &str) {
        self.labels.push(FolderItem::new(name, icon, FolderType::Custom));
    }

    /// Render the sidebar with responsive behavior
    pub fn show(&mut self, ctx: &egui::Context) {
        let screen_size = ScreenSize::from_ctx(ctx);
        
        // Adjust sidebar based on screen size
        match screen_size {
            ScreenSize::Mobile => {
                // On mobile, sidebar is hidden by default, toggle via button
                if self.visible {
                    self.width = ctx.available_rect().width() * 0.8; // 80% of screen
                    egui::SidePanel::left("sidebar")
                        .width_range(self.width..=self.width)
                        .show(ctx, |ui| {
                            self.render_content(ui, &screen_size);
                        });
                }
            }
            ScreenSize::Tablet => {
                // Tablet: narrower sidebar
                self.width = screen_size.sidebar_width();
                self.visible = true;
                egui::SidePanel::left("sidebar")
                    .width_range(self.width..=self.width)
                    .show(ctx, |ui| {
                        self.render_content(ui, &screen_size);
                    });
            }
            ScreenSize::Desktop => {
                // Desktop: full sidebar
                self.width = screen_size.sidebar_width();
                self.visible = true;
                egui::SidePanel::left("sidebar")
                    .width_range(200.0..=350.0)
                    .default_width(self.width)
                    .show(ctx, |ui| {
                        self.render_content(ui, &screen_size);
                    });
            }
        }
    }

    /// Toggle sidebar visibility (for mobile)
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Show/hide sidebar
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if sidebar is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Render sidebar content with responsive layout
    fn render_content(&mut self, ui: &mut egui::Ui, screen_size: &ScreenSize) {
        // Header with compose button (Gmail style)
        ui.add_space(8.0);
        
        // Compose button
        let compose_button = egui::Button::new("➤  Compose")
            .min_size(egui::vec2(0.0, 40.0))
            .fill(egui::Color32::from_rgb(0, 122, 255));
        
        if ui.add(compose_button).clicked() {
            // Trigger compose action - would be handled by parent
        }
        
        ui.add_space(16.0);
        
        // Render folders
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Labels section header
            ui.label("FOLDERS");
            ui.separator();
            
            // Render folders using indices
            for i in 0..self.folders.len() {
                let folder = &self.folders[i];
                let is_selected = folder.selected;
                let folder_name = folder.name.clone();
                let folder_icon = folder.icon.clone();
                let unread = folder.unread;
                
                let (bg_color, text_color) = if is_selected {
                    (egui::Color32::from_rgb(0, 122, 255).linear_multiply(0.2), egui::Color32::from_rgb(0, 122, 255))
                } else {
                    (egui::Color32::TRANSPARENT, egui::Color32::from_gray(230))
                };

                let button = egui::Button::new(
                    egui::RichText::new(format!("{} {}", folder_icon, folder_name))
                        .color(text_color)
                )
                .fill(bg_color)
                .min_size(egui::vec2(0.0, 32.0));

                if ui.add(button).clicked() {
                    self.select_folder_by_index(i);
                }
            }
            
            ui.add_space(16.0);
            
            // Custom labels section header
            ui.label("LABELS");
            ui.separator();
            
            // Render labels
            for i in 0..self.labels.len() {
                let label = &self.labels[i];
                let is_selected = label.selected;
                let label_name = label.name.clone();
                let label_icon = label.icon.clone();
                
                let (bg_color, text_color) = if is_selected {
                    (egui::Color32::from_rgb(0, 122, 255).linear_multiply(0.2), egui::Color32::from_rgb(0, 122, 255))
                } else {
                    (egui::Color32::TRANSPARENT, egui::Color32::from_gray(230))
                };

                let button = egui::Button::new(
                    egui::RichText::new(format!("{} {}", label_icon, label_name))
                        .color(text_color)
                )
                .fill(bg_color)
                .min_size(egui::vec2(0.0, 32.0));

                if ui.add(button).clicked() {
                    self.select_label(i);
                }
            }
            
            // Add label button
            if ui.button("+ Add label").clicked() {
                // Would trigger label creation dialog
            }
        });
        
        // Footer with account info
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.label("🔒 Sovereign Mode");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("v0.2.0");
            });
        });
    }

    /// Select a folder by index (internal)
    fn select_folder_by_index(&mut self, index: usize) {
        for (i, folder) in &mut self.folders.iter_mut().enumerate() {
            folder.selected = i == index;
        }
        for label in &mut self.labels {
            label.selected = false;
        }
    }

    /// Select folder by index (public)
    pub fn select_folder(&mut self, folder_type: FolderType) {
        for folder in &mut self.folders {
            folder.selected = folder.folder_type == folder_type;
        }
        for label in &mut self.labels {
            label.selected = false;
        }
    }

    /// Select a label by index
    pub fn select_label(&mut self, index: usize) {
        for folder in &mut self.folders {
            folder.selected = false;
        }
        for (i, label) in &mut self.labels.iter_mut().enumerate() {
            label.selected = i == index;
        }
    }

    /// Get selected folder type
    pub fn selected(&self) -> Option<FolderType> {
        for folder in &self.folders {
            if folder.selected {
                return Some(folder.folder_type);
            }
        }
        for label in &self.labels {
            if label.selected {
                return Some(label.folder_type);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_folders() {
        let folders = FolderItem::default_folders();
        assert_eq!(folders.len(), 6);
        assert_eq!(folders[0].name, "Inbox");
    }

    #[test]
    fn test_sidebar_selection() {
        let mut sidebar = Sidebar::new();
        sidebar.select_folder(FolderType::Sent);
        assert_eq!(sidebar.selected(), Some(FolderType::Sent));
    }
}
