//! Thundermail - A Sovereign Email Client
//!
//! This is the entry point for the Thundermail application.
//! Thundermail is a high-performance, privacy-first email client
//! written in 100% native Rust.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::panic;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

mod ai;
mod core;
mod crypto;
mod db;
mod net;
mod privacy;
mod ui;

use ui::ThundermailApp;

fn setup_logging() {
    // Set up logging directory
    let log_dir = directories::ProjectDirs::from("org", "thundermail", "Thundermail")
        .map(|dirs| dirs.data_local_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("thundermail"));

    std::fs::create_dir_all(&log_dir).ok();

    // Create rolling file appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        &log_dir,
        "thundermail.log",
    );

    // Initialize subscriber with both console and file output
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_writer(file_appender)
        .with_ansi(false)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

fn setup_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        let location = panic_info.location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        error!(
            target: "thundermail::panic",
            location = %location,
            message = %message,
            "Application panicked"
        );

        // Exit with error code
        std::process::exit(1);
    }));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logging();
    setup_panic_handler();

    info!("Thundermail v{} starting...", env!("CARGO_PKG_VERSION"));
    info!("A Sovereign, Privacy-First Email Client");

    // Run the egui application
    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Thundermail",
        native_options,
        Box::new(|_cc| Ok(Box::new(ThundermailApp::new()))),
    ).map_err(|e| {
        error!("Failed to run native application: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    info!("Thundermail shutting down gracefully");
    Ok(())
}
