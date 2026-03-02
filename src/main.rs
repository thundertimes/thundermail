//! Thundermail - A Sovereign Email Client
//!
//! This is the entry point for the Thundermail application.

#![forbid(unsafe_code)]

use thundermail::error;
use thundermail::ai;
use thundermail::core;
use thundermail::crypto;
use thundermail::db;
use thundermail::net;
use thundermail::privacy;
use thundermail::ui::ThundermailApp;

fn main() {
    println!("Thundermail v{} starting...", env!("CARGO_PKG_VERSION"));
    println!("A Sovereign, Privacy-First Email Client");

    // Run the egui application
    let native_options = eframe::NativeOptions::default();
    
    let app = ThundermailApp::new();
    eframe::run_native("Thundermail", native_options, Box::new(|_cc| Box::new(app)));
}
