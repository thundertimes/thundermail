use egui::Ui;

pub trait ThundermailPlugin: Send + Sync {
    /// Metadata for the plugin manager
    fn name(&self) -> &'static str;
    
    /// Hook: Process mail before it's saved (e.g., custom filtering)
    fn on_mail_received(&self, mail: &mut SanitizedMail);

    /// Hook: Add custom UI elements to the egui sidebar or message view
    fn render_settings(&mut self, ui: &mut Ui);
}

/// The entry point every plugin must export
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe fn() -> Box<dyn ThundermailPlugin>,
}