use libloading::{Library, Symbol};
use thundermail_sdk::{ThundermailPlugin, PluginDeclaration};

pub struct PluginManager {
    plugins: Vec<Box<dyn ThundermailPlugin>>,
    loaded_libraries: Vec<Library>, // Must keep libs in memory
}

impl PluginManager {
    pub unsafe fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib = Library::new(path)?;
        let decl: Symbol<*const PluginDeclaration> = lib.get(b"plugin_declaration\0")?;
        
        // Version check to prevent segfaults
        if (**decl).core_version != env!("CARGO_PKG_VERSION") {
            return Err("Plugin version mismatch".into());
        }

        let plugin = ((**decl).register)();
        self.plugins.push(plugin);
        self.loaded_libraries.push(lib);
        Ok(())
    }
}