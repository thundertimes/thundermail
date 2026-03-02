//! Thundermail - A Sovereign Email Client
//!
//! This is the entry point for the Thundermail application.

#![forbid(unsafe_code)]

use std::sync::Arc;
use egui::IconData;
use thundermail::ui::ThundermailApp;

fn load_icon() -> Option<IconData> {
    let icon_bytes = include_bytes!("../resources/icon.png");
    let icon_image = image::load_from_memory(icon_bytes).ok()?;
    let rgba = icon_image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some(IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    })
}

fn main() {
    println!("Thundermail v{} starting...", env!("CARGO_PKG_VERSION"));
    println!("A Sovereign, Privacy-First Email Client");

    // Load icon
    let icon = load_icon();
    
    // Run the egui application
    let mut native_options = eframe::NativeOptions::default();
    
    // Set the icon via viewport
    if let Some(icon) = icon {
        native_options.viewport.inner_size = Some(egui::vec2(1024.0, 768.0));
        native_options.viewport.icon = Some(Arc::new(icon));
    }
    
    let app = ThundermailApp::new();
    eframe::run_native("Thundermail", native_options, Box::new(|_cc| Box::new(app)));
}
