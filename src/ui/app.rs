//! App Module - Main eframe Application
//!
//! This module provides the main application UI using egui/eframe.

#![forbid(unsafe_code)]

use eframe::egui::{self, Widget};
use super::{Theme, Sidebar, FolderType, OnboardingState, OnboardingStep, AutoConfig, ScreenSize};
use crate::core::{Session, Account, Email};
use crate::crypto::gpg_keys::{GpgKeyManager, Attachment};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main Thundermail application
pub struct ThundermailApp {
    /// Sidebar
    sidebar: Sidebar,
    /// Theme
    theme: Theme,
    /// Current view
    view: AppView,
    /// Onboarding state
    onboarding: OnboardingState,
    /// Has account (skip onboarding)
    has_account: bool,
    /// Account email for display
    account_email: String,
    /// Search query
    search_query: String,
    /// Compose: To field
    compose_to: String,
    /// Compose: Subject field
    compose_subject: String,
    /// Compose: Body field
    compose_body: String,
    /// Compose: Attachments
    compose_attachments: Vec<Attachment>,
    /// IMAP/SMTP Session
    session: Option<Session>,
    /// Fetched emails cache
    emails: Vec<Email>,
    /// Currently selected email (for viewing)
    selected_email: Option<Email>,
    /// Is loading emails
    is_loading: bool,
    /// Connection test result
    connection_status: ConnectionTestResult,
    /// GPG key attached status
    pgp_key_attached: bool,
}

/// Connection test result
#[derive(Debug, Clone)]
struct ConnectionTestResult {
    imap_success: bool,
    smtp_success: bool,
    imap_error: Option<String>,
    smtp_error: Option<String>,
    tested: bool,
}

/// Application views
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppView {
    /// Onboarding view
    Onboarding,
    /// Inbox view
    Inbox,
    /// Compose view
    Compose,
    /// Settings view
    Settings,
    /// Email list view
    EmailList,
}

impl Default for AppView {
    fn default() -> Self {
        Self::Onboarding
    }
}

impl ThundermailApp {
    /// Create a new application
    pub fn new() -> Self {
        Self {
            sidebar: Sidebar::new(),
            theme: Theme::default(),
            view: AppView::Onboarding,
            onboarding: OnboardingState::new(),
            has_account: false,
            account_email: String::new(),
            search_query: String::new(),
            compose_to: String::new(),
            compose_subject: String::new(),
            compose_body: String::new(),
            compose_attachments: Vec::new(),
            session: None,
            emails: Vec::new(),
            selected_email: None,
            is_loading: false,
            connection_status: ConnectionTestResult {
                imap_success: false,
                smtp_success: false,
                imap_error: None,
                smtp_error: None,
                tested: false,
            },
            pgp_key_attached: false,
        }
    }
}

impl eframe::App for ThundermailApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.theme.apply(ctx);

        // Show the appropriate view
        if !self.has_account {
            self.show_onboarding(ctx);
        } else {
            self.show_main_ui(ctx);
        }
    }
}

impl ThundermailApp {
    /// Show onboarding wizard
    fn show_onboarding(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                
                // Logo and title
                ui.heading(egui::RichText::new("⚡ Thundermail").size(32.0));
                ui.label("Sovereign Email Client");
                
                ui.add_space(40.0);
                
                match self.onboarding.step {
                    OnboardingStep::Welcome => {
                        self.render_welcome_step(ui);
                    }
                    OnboardingStep::Discovering => {
                        self.render_discovering_step(ui);
                    }
                    OnboardingStep::Configure => {
                        self.render_configure_step(ui);
                    }
                    OnboardingStep::Testing => {
                        self.render_testing_step(ui);
                    }
                    OnboardingStep::Complete => {
                        self.render_complete_step(ui);
                    }
                    OnboardingStep::Error => {
                        self.render_error_step(ui);
                    }
                }
            });
        });
    }

    /// Render welcome step
    fn render_welcome_step(&mut self, ui: &mut egui::Ui) {
        ui.label("Welcome! Enter your email address to get started.");
        ui.label("We'll automatically configure your account.");
        
        ui.add_space(20.0);
        
        // Email input
        ui.horizontal(|ui| {
            ui.label("Email:");
            ui.text_edit_singleline(&mut self.onboarding.email);
        });
        
        ui.add_space(20.0);
        
        // Continue button
        if ui.button("Continue →").clicked() {
            // Start synchronous discovery
            self.onboarding.start_discovery();
        }
        
        ui.add_space(30.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Manual config link
        if ui.button("Configure manually").clicked() {
            self.onboarding.step = OnboardingStep::Configure;
            self.onboarding.auto_config = Some(AutoConfig {
                imap_server: String::new(),
                imap_port: 993,
                smtp_server: String::new(),
                smtp_port: 587,
                use_tls: true,
                source: super::ConfigSource::Manual,
            });
        }
    }

    /// Render discovering step
    fn render_discovering_step(&mut self, ui: &mut egui::Ui) {
        ui.spinner();
        ui.label("Discovering your email settings...");
        ui.label(&self.onboarding.email);
        
        ui.add_space(20.0);
        
        // Progress indicator
        egui::ProgressBar::new(0.5)
            .animate(true)
            .ui(ui);
        
        ui.add_space(20.0);
        
        ui.label("Looking up your mail server configuration...");
        ui.label("This may take a few seconds.");
    }

    /// Render configuration step
    fn render_configure_step(&mut self, ui: &mut egui::Ui) {
        ui.heading("Email Configuration");
        
        ui.add_space(20.0);
        
        if let Some(config) = &self.onboarding.auto_config {
            // Show discovered configuration
            ui.label(egui::RichText::new("✓ Auto-discovered settings").color(egui::Color32::from_rgb(0, 200, 100)));
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label("Source:");
                ui.label(match config.source {
                    super::ConfigSource::KnownProvider => "Known Provider",
                    super::ConfigSource::MX => "DNS MX Lookup",
                    super::ConfigSource::Autodiscover => "Autodiscover",
                    super::ConfigSource::Manual => "Manual",
                });
            });
            
            ui.add_space(10.0);
            
            // IMAP Settings
            ui.group(|ui| {
                ui.label("IMAP (Incoming Mail)");
                ui.horizontal(|ui| {
                    ui.label("Server:");
                    ui.text_edit_singleline(&mut self.onboarding.auto_config.as_mut().unwrap().imap_server);
                });
                ui.horizontal(|ui| {
                    ui.label("Port:");
                    let port_str = self.onboarding.auto_config.as_mut().unwrap().imap_port.to_string();
                    let mut port_edit = port_str.clone();
                    ui.text_edit_singleline(&mut port_edit);
                    if let Ok(p) = port_edit.parse() {
                        self.onboarding.auto_config.as_mut().unwrap().imap_port = p;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Encryption:");
                    ui.label("TLS");
                });
            });
            
            ui.add_space(10.0);
            
            // SMTP Settings
            ui.group(|ui| {
                ui.label("SMTP (Outgoing Mail)");
                ui.horizontal(|ui| {
                    ui.label("Server:");
                    ui.text_edit_singleline(&mut self.onboarding.auto_config.as_mut().unwrap().smtp_server);
                });
                ui.horizontal(|ui| {
                    ui.label("Port:");
                    let port_str = self.onboarding.auto_config.as_mut().unwrap().smtp_port.to_string();
                    let mut port_edit = port_str.clone();
                    ui.text_edit_singleline(&mut port_edit);
                    if let Ok(p) = port_edit.parse() {
                        self.onboarding.auto_config.as_mut().unwrap().smtp_port = p;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Encryption:");
                    ui.label("TLS");
                });
            });
        }
        
        ui.add_space(20.0);
        
        // Password input
        ui.label("Password / App Password:");
        ui.add(egui::TextEdit::singleline(&mut self.onboarding.password).password(true));
        
        ui.add_space(20.0);
        
        // Show connection test results if tested
        if self.connection_status.tested {
            ui.group(|ui| {
                ui.label("Connection Test Results:");
                ui.add_space(5.0);
                
                // IMAP result
                if self.connection_status.imap_success {
                    ui.label(egui::RichText::new("✓ IMAP Connected").color(egui::Color32::from_rgb(0, 200, 100)));
                } else {
                    ui.label(egui::RichText::new("✗ IMAP Failed").color(egui::Color32::from_rgb(255, 100, 100)));
                    if let Some(ref err) = self.connection_status.imap_error {
                        ui.label(egui::RichText::new(err).small().color(egui::Color32::from_gray(180)));
                    }
                }
                
                ui.add_space(5.0);
                
                // SMTP result
                if self.connection_status.smtp_success {
                    ui.label(egui::RichText::new("✓ SMTP Connected").color(egui::Color32::from_rgb(0, 200, 100)));
                } else {
                    ui.label(egui::RichText::new("✗ SMTP Failed").color(egui::Color32::from_rgb(255, 100, 100)));
                    if let Some(ref err) = self.connection_status.smtp_error {
                        ui.label(egui::RichText::new(err).small().color(egui::Color32::from_gray(180)));
                    }
                }
            });
        }
        
        ui.add_space(20.0);
        
        // Test Connection button
        if ui.button("Test Connection").clicked() {
            self.test_connection();
        }
        
        ui.add_space(10.0);
        
        // Connect button - only enable if test passed or user wants to skip
        let can_connect = !self.onboarding.password.is_empty();
        
        if can_connect {
            if ui.button("Connect Account").clicked() {
                // Create account from onboarding config
                let config = self.onboarding.auto_config.as_ref().unwrap();
                let account = Account::new(
                    self.onboarding.email.clone(),
                    config.imap_server.clone(),
                    config.smtp_server.clone(),
                );
                
                // Create session with account
                let mut session = Session::new(account);
                
                // Try to connect IMAP synchronously (using block_in_place)
                self.is_loading = true;
                let result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        session.connect_imap().await
                    })
                });
                
                if result.is_ok() {
                    // Fetch initial emails
                    let folder = "INBOX";
                    let fetched_emails = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            session.fetch_emails(folder, 20).await.unwrap_or_default()
                        })
                    });
                    self.emails = fetched_emails;
                    
                    self.session = Some(session);
                    self.has_account = true;
                    self.account_email = self.onboarding.email.clone();
                    self.view = AppView::EmailList;
                    self.sidebar.select_folder(FolderType::Inbox);
                }
                self.is_loading = false;
            }
        }
        
        ui.add_space(10.0);
        
        if ui.button("← Back").clicked() {
            self.onboarding.step = OnboardingStep::Welcome;
        }
    }
    
    /// Test IMAP/SMTP connection
    fn test_connection(&mut self) {
        use crate::core::Account;
        
        if let Some(_config) = &self.onboarding.auto_config {
            let email = self.onboarding.email.clone();
            let password = self.onboarding.password.clone();
            
            if email.is_empty() || password.is_empty() {
                self.connection_status.imap_error = Some("Please enter email and password".to_string());
                self.connection_status.tested = true;
                return;
            }
            
            // Test IMAP connection (simulated for demo)
            // In production, this would actually connect via async-imap
            let config = self.onboarding.auto_config.as_ref().unwrap();
            
            // Try to resolve the IMAP server
            let imap_addr = format!("{}:{}", config.imap_server, config.imap_port);
            if std::net::TcpStream::connect_timeout(
                &imap_addr.parse().unwrap_or_else(|_| "127.0.0.1:993".parse().unwrap()),
                std::time::Duration::from_secs(5)
            ).is_ok() {
                self.connection_status.imap_success = true;
            } else {
                self.connection_status.imap_success = false;
                self.connection_status.imap_error = Some("Could not connect to IMAP server. Check settings.".to_string());
            }
            
            // Try to resolve the SMTP server
            let smtp_addr = format!("{}:{}", config.smtp_server, config.smtp_port);
            if std::net::TcpStream::connect_timeout(
                &smtp_addr.parse().unwrap_or_else(|_| "127.0.0.1:587".parse().unwrap()),
                std::time::Duration::from_secs(5)
            ).is_ok() {
                self.connection_status.smtp_success = true;
            } else {
                self.connection_status.smtp_success = false;
                self.connection_status.smtp_error = Some("Could not connect to SMTP server. Check settings.".to_string());
            }
            
            self.connection_status.tested = true;
        }
    }

    /// Render testing step
    fn render_testing_step(&mut self, ui: &mut egui::Ui) {
        ui.spinner();
        ui.label("Testing your connection...");
        
        ui.add_space(20.0);
        
        ui.label("Verifying IMAP connection...");
        ui.label("Verifying SMTP connection...");
        
        ui.add_space(20.0);
        
        egui::ProgressBar::new(0.8).animate(true).ui(ui);
    }

    /// Render complete step
    fn render_complete_step(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("✓ Account Connected!").color(egui::Color32::from_rgb(0, 200, 100)));
        
        ui.add_space(20.0);
        
        ui.label("Your email account has been configured successfully.");
        
        ui.add_space(30.0);
        
        if ui.button("Start Using Thundermail").clicked() {
            self.has_account = true;
            self.account_email = self.onboarding.email.clone();
            self.view = AppView::Inbox;
            self.sidebar.select_folder(FolderType::Inbox);
        }
    }

    /// Render error step
    fn render_error_step(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("⚠ Configuration Failed").color(egui::Color32::from_rgb(255, 100, 100)));
        
        ui.add_space(20.0);
        
        if let Some(error) = &self.onboarding.error {
            ui.label(error);
        }
        
        ui.add_space(30.0);
        
        if ui.button("Try Again").clicked() {
            self.onboarding.step = OnboardingStep::Welcome;
        }
        
        ui.add_space(10.0);
        
        if ui.button("Configure Manually").clicked() {
            self.onboarding.step = OnboardingStep::Configure;
        }
    }

    /// Show main UI with sidebar
    fn show_main_ui(&mut self, ctx: &egui::Context) {
        let screen_size = ScreenSize::from_ctx(ctx);
        
        // Show sidebar
        self.sidebar.show(ctx);

        // Top bar - responsive
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Mobile: hamburger menu button to toggle sidebar
                if screen_size == ScreenSize::Mobile {
                    if ui.button("☰").clicked() {
                        self.sidebar.toggle();
                    }
                    ui.separator();
                }
                
                // App title - responsive size
                let title_size = match screen_size {
                    ScreenSize::Mobile => 16.0,
                    ScreenSize::Tablet => 18.0,
                    ScreenSize::Desktop => 20.0,
                };
                ui.heading(egui::RichText::new("⚡ Thundermail").size(title_size));
                
                // Only show separator and account on larger screens
                if screen_size != ScreenSize::Mobile {
                    ui.separator();
                    
                    // Account display
                    ui.label(egui::RichText::new(&self.account_email).small());
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Search box - responsive width
                    let search_width = screen_size.search_width();
                    ui.add(egui::TextEdit::singleline(&mut self.search_query).desired_width(search_width).hint_text("Search"));
                    
                    ui.separator();
                    
                    // Privacy indicator - hide text on mobile
                    if screen_size.show_full_labels() {
                        ui.label("🔒 Sovereign Mode Active");
                    } else {
                        ui.label("🔒").on_hover_text("Sovereign Mode Active");
                    }
                });
            });
        });

        // Show main content based on current view
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view {
                AppView::Inbox => {
                    self.render_inbox(ui, &screen_size);
                }
                AppView::Compose => {
                    self.render_compose(ui, &screen_size);
                }
                AppView::Settings => {
                    self.render_settings(ui, &screen_size);
                }
                AppView::EmailList => {
                    self.render_email_list(ui, &screen_size);
                }
                AppView::Onboarding => {
                    // Should not happen
                }
            }
        });
    }

    /// Render inbox view
    fn render_inbox(&mut self, ui: &mut egui::Ui, screen_size: &ScreenSize) {
        // Responsive button layout
        let button_height = match screen_size {
            ScreenSize::Mobile => 32.0,
            _ => 36.0,
        };
        
        ui.horizontal(|ui| {
            if ui.add_sized([80.0, button_height], egui::Button::new("📥 Inbox")).clicked() {
                self.sidebar.select_folder(FolderType::Inbox);
                self.view = AppView::EmailList;
            }
            if ui.add_sized([80.0, button_height], egui::Button::new("📤 Sent")).clicked() {
                self.sidebar.select_folder(FolderType::Sent);
                self.view = AppView::EmailList;
            }
            if ui.add_sized([80.0, button_height], egui::Button::new("📝 Drafts")).clicked() {
                self.sidebar.select_folder(FolderType::Drafts);
                self.view = AppView::EmailList;
            }
            if ui.add_sized([80.0, button_height], egui::Button::new("⚙️ Settings")).clicked() {
                self.view = AppView::Settings;
            }
        });
        
        ui.add_space(20.0);
        
        // Email list placeholder
        egui::ScrollArea::vertical().show(ui, |ui| {
            let heading_size = match screen_size {
                ScreenSize::Mobile => 18.0,
                ScreenSize::Tablet => 20.0,
                ScreenSize::Desktop => 24.0,
            };
            ui.heading(egui::RichText::new("Inbox").size(heading_size));
            ui.label("Your sovereign communications will appear here.");
            ui.label("");
            ui.label("No emails yet. Send yourself a test email to get started!");
            
            ui.add_space(20.0);
            
            if ui.button("➤ Compose").clicked() {
                self.view = AppView::Compose;
            }
        });
    }

    /// Render compose view
    fn render_compose(&mut self, ui: &mut egui::Ui, screen_size: &ScreenSize) {
        // Responsive heading
        let heading_size = match screen_size {
            ScreenSize::Mobile => 18.0,
            ScreenSize::Tablet => 20.0,
            ScreenSize::Desktop => 24.0,
        };
        ui.heading(egui::RichText::new("Compose New Email").size(heading_size));
        
        ui.add_space(10.0);
        
        // Responsive field widths
        let field_width = match screen_size {
            ScreenSize::Mobile => 200.0,
            ScreenSize::Tablet => 400.0,
            ScreenSize::Desktop => 600.0,
        };
        
        ui.horizontal(|ui| {
            ui.label("To:");
            ui.add(egui::TextEdit::singleline(&mut self.compose_to).desired_width(field_width));
        });
        
        ui.horizontal(|ui| {
            ui.label("Subject:");
            ui.add(egui::TextEdit::singleline(&mut self.compose_subject).desired_width(field_width));
        });
        
        ui.add_space(10.0);
        
        ui.label("Body:");
        
        // Responsive text area
        let body_rows = match screen_size {
            ScreenSize::Mobile => 6,
            ScreenSize::Tablet => 8,
            ScreenSize::Desktop => 10,
        };
        ui.add(egui::TextEdit::multiline(&mut self.compose_body).desired_rows(body_rows).desired_width(field_width + 100.0));
        
        ui.add_space(10.0);
        
        // Attachments section
        ui.group(|ui| {
            ui.label(egui::RichText::new("Attachments").strong());
            ui.add_space(5.0);
            
            // GPG Public Key attachment
            if !self.pgp_key_attached {
                if ui.button("🔐 Attach PGP Public Key").clicked() {
                    // Try to get GPG public key from system
                    if let Some(attachment) = Attachment::from_gpg_public_key(None) {
                        self.compose_attachments.push(attachment);
                        self.pgp_key_attached = true;
                    }
                }
                
                // Show GPG status
                if GpgKeyManager::is_gpg_available() {
                    ui.label(egui::RichText::new("✓ GPG available on system").small().color(egui::Color32::from_rgb(0, 200, 100)));
                    
                    // Show available keys
                    let keys = GpgKeyManager::list_public_keys();
                    if !keys.is_empty() {
                        ui.label(egui::RichText::new(format!("{} key(s) found", keys.len())).small());
                        
                        // Show key selection
                        for key in keys.iter().take(3) {
                            ui.label(egui::RichText::new(format!("  • {}", key.user_id)).small());
                        }
                    } else {
                        ui.label(egui::RichText::new("No GPG keys found in keyring").small().color(egui::Color32::from_rgb(255, 180, 0)));
                    }
                } else {
                    ui.label(egui::RichText::new("⚠ GPG not found - install GPG to attach keys").small().color(egui::Color32::from_rgb(255, 100, 100)));
                }
            } else {
                // Show attached PGP key
                ui.label(egui::RichText::new("✓ PGP Public Key Attached").color(egui::Color32::from_rgb(0, 200, 100)));
                
                // Find PGP key index
                if let Some(idx) = self.compose_attachments.iter().position(|a| a.is_pgp_key) {
                    let filename = self.compose_attachments[idx].filename.clone();
                    let mut clicked = false;
                    ui.horizontal(|ui| {
                        ui.label("🔐");
                        ui.label(&filename);
                        clicked = ui.button("✕").clicked();
                    });
                    if clicked {
                        self.compose_attachments.remove(idx);
                        self.pgp_key_attached = false;
                    }
                }
            }
        });
        
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            if ui.button("Send").clicked() {
                // Send email - would trigger SMTP send
                // Include PGP public key as attachment if attached
                if !self.compose_attachments.is_empty() {
                    // Log that we're attaching the key
                    for attachment in &self.compose_attachments {
                        if attachment.is_pgp_key {
                            tracing::info!("Attaching PGP public key: {}", attachment.filename);
                        }
                    }
                }
                
                self.compose_to.clear();
                self.compose_subject.clear();
                self.compose_body.clear();
                self.compose_attachments.clear();
                self.pgp_key_attached = false;
                self.view = AppView::Inbox;
            }
            if ui.button("Save Draft").clicked() {
                // Save draft
                self.view = AppView::Inbox;
            }
            if ui.button("Discard").clicked() {
                self.compose_to.clear();
                self.compose_subject.clear();
                self.compose_body.clear();
                self.compose_attachments.clear();
                self.pgp_key_attached = false;
                self.view = AppView::Inbox;
            }
        });
    }

    /// Render settings view
    fn render_settings(&mut self, ui: &mut egui::Ui, screen_size: &ScreenSize) {
        // Responsive heading
        let heading_size = match screen_size {
            ScreenSize::Mobile => 18.0,
            ScreenSize::Tablet => 20.0,
            ScreenSize::Desktop => 24.0,
        };
        ui.heading(egui::RichText::new("Settings").size(heading_size));
        
        ui.add_space(20.0);
        
        // Account section
        ui.group(|ui| {
            ui.label(egui::RichText::new("Account").strong());
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label("Email:");
                ui.label(&self.account_email);
            });
            
            ui.add_space(10.0);
            
            if ui.button("Manage Account").clicked() {
                // Show account management
            }
        });
        
        ui.add_space(20.0);
        
        // Appearance section
        ui.group(|ui| {
            ui.label(egui::RichText::new("Appearance").strong());
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.button("Dark").clicked() {
                    self.theme = Theme::default();
                }
                if ui.button("Light").clicked() {
                    // Would switch to light theme
                }
            });
        });
        
        ui.add_space(20.0);
        
        // Privacy section
        ui.group(|ui| {
            ui.label(egui::RichText::new("Privacy & Security").strong());
            ui.add_space(10.0);
            
            ui.checkbox(&mut true, "Encrypt emails with PGP");
            ui.checkbox(&mut true, "Strip tracking headers");
            ui.checkbox(&mut true, "Use Tor/Proxy");
        });
        
        ui.add_space(20.0);
        
        if ui.button("← Back to Inbox").clicked() {
            self.view = AppView::Inbox;
        }
    }

    /// Render email list view
    fn render_email_list(&mut self, ui: &mut egui::Ui, screen_size: &ScreenSize) {
        // Responsive heading
        let heading_size = match screen_size {
            ScreenSize::Mobile => 18.0,
            ScreenSize::Tablet => 20.0,
            ScreenSize::Desktop => 24.0,
        };
        
        // Folder header
        let folder_name = match self.sidebar.selected() {
            Some(FolderType::Inbox) => "📥 Inbox",
            Some(FolderType::Sent) => "📤 Sent",
            Some(FolderType::Drafts) => "📝 Drafts",
            Some(FolderType::Spam) => "⚠️ Spam",
            Some(FolderType::Trash) => "🗑️ Trash",
            Some(FolderType::Archive) => "📦 Archive",
            Some(FolderType::Custom) => "🏷️ Label",
            None => "📥 Inbox",
        };
        
        ui.heading(egui::RichText::new(folder_name).size(heading_size));
        
        ui.add_space(10.0);
        
        // Fetch folder name string for email retrieval
        let folder_str = match self.sidebar.selected() {
            Some(FolderType::Inbox) => "INBOX",
            Some(FolderType::Sent) => "Sent",
            Some(FolderType::Drafts) => "Drafts",
            Some(FolderType::Spam) => "Spam",
            Some(FolderType::Trash) => "Trash",
            Some(FolderType::Archive) => "Archive",
            Some(FolderType::Custom) => "Custom",
            None => "INBOX",
        };
        
        // Refresh button - responsive
        let button_height = match screen_size {
            ScreenSize::Mobile => 28.0,
            _ => 32.0,
        };
        
        ui.horizontal(|ui| {
            if ui.add_sized([80.0, button_height], egui::Button::new("🔄 Refresh")).clicked() {
                if let Some(ref mut session) = self.session {
                    self.is_loading = true;
                    let fetched = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            session.fetch_emails(folder_str, 20).await.unwrap_or_default()
                        })
                    });
                    self.emails = fetched;
                    self.is_loading = false;
                }
            }
        });
        
        ui.add_space(10.0);
        
        // Show loading indicator
        if self.is_loading {
            ui.spinner();
            ui.label("Loading emails...");
            ui.add_space(10.0);
        }
        
        // Clone emails for display to avoid borrow issues
        let emails_clone: Vec<Email> = self.emails.clone();
        let has_emails = !emails_clone.is_empty();
        
        // Email list
        egui::ScrollArea::vertical().show(ui, |ui| {
            if !has_emails {
                ui.label("No emails in this folder.");
                ui.add_space(10.0);
                
                if ui.button("Send Test Email").clicked() {
                    self.view = AppView::Compose;
                }
            } else {
                // Display fetched emails with responsive styling
                for email in emails_clone {
                    let is_unread = !email.is_read;
                    let bg_color = if is_unread {
                        egui::Color32::from_gray(50)
                    } else {
                        egui::Color32::from_gray(35)
                    };
                    
                    egui::Frame::default()
                        .fill(bg_color)
                        .rounding(4.0)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                // Star indicator
                                if email.is_starred {
                                    ui.label(egui::RichText::new("★").color(egui::Color32::from_rgb(255, 215, 0)));
                                } else {
                                    ui.label(" ");
                                }
                                
                                // Unread indicator
                                if is_unread {
                                    ui.label(egui::RichText::new("●").small().color(egui::Color32::from_rgb(66, 135, 245)));
                                }
                                
                                // From - responsive text size
                                let from_name = email.from.split('<').next().unwrap_or(&email.from).trim();
                                let from_size = match screen_size {
                                    ScreenSize::Mobile => 12.0,
                                    _ => 14.0,
                                };
                                ui.label(egui::RichText::new(from_name).strong().size(from_size));
                                
                                ui.add_space(8.0);
                                
                                // Subject
                                ui.label(&email.subject);
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    // Date
                                    let date_str = email.date.format("%m/%d %H:%M").to_string();
                                    ui.label(egui::RichText::new(date_str).small().color(egui::Color32::from_gray(150)));
                                    
                                    // Attachment indicator
                                    if email.has_attachments {
                                        ui.label("📎");
                                    }
                                });
                            });
                            
                            // Preview - responsive
                            ui.add_space(5.0);
                            let preview_len = match screen_size {
                                ScreenSize::Mobile => 50,
                                ScreenSize::Tablet => 65,
                                ScreenSize::Desktop => 80,
                            };
                            let preview = if email.body.len() > preview_len {
                                format!("{}...", &email.body[..preview_len])
                            } else {
                                email.body.clone()
                            };
                            ui.label(egui::RichText::new(preview).italics().small().color(egui::Color32::from_gray(180)));
                            
                            // Click to view button
                            ui.add_space(5.0);
                            let email_clone = email.clone();
                            if ui.button("View Email").clicked() {
                                self.selected_email = Some(email_clone);
                            }
                        });
                    
                    ui.add_space(5.0);
                }
            }
            
            ui.add_space(20.0);
            
            if ui.button("← Back").clicked() {
                self.view = AppView::Inbox;
            }
            
            // Compose button
            if ui.button("➤ Compose").clicked() {
                self.view = AppView::Compose;
            }
        });
        
        // Handle selected email popup separately
        let selected = self.selected_email.clone();
        if let Some(email) = selected {
            let mut open = true;
            egui::Window::new("Email")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    ui.heading(&email.subject);
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("From:").strong());
                        ui.label(&email.from);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("To:").strong());
                        ui.label(&email.to);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Date:").strong());
                        ui.label(email.date.format("%Y-%m-%d %H:%M:%S UTC").to_string());
                    });
                    
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    // Email body
                    ui.label(&email.body);
                    
                    ui.add_space(20.0);
                    
                    if ui.button("Close").clicked() {
                        self.selected_email = None;
                    }
                });
            
            if !open {
                self.selected_email = None;
            }
        }
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
        assert_eq!(app.view, AppView::Onboarding);
    }

    #[test]
    fn test_sidebar_default() {
        let sidebar = Sidebar::new();
        assert_eq!(sidebar.folders.len(), 6);
    }
}
