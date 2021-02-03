use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use serde::Deserialize;
use serde_json::Value as Json;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    #[serde(skip)]
    Unknown,

    Library,

    #[cfg(feature = "python_support_enabled")]
    Python,

    Resource,
}

pub struct Plugin {
    name: String,
    path: String,
    resource_path: String,
    plug_info: Json,
    plug_type: PluginType,
}

impl Plugin {
    /// Loads the plugin.
    /// This is a noop if the plugin is already loaded.
    pub fn load(&self) {
        todo!()
    }

    /// Returns `true` if the plugin is currently loaded. Resource
    /// plugins always report as loaded.
    pub fn is_loaded() -> bool {
        todo!()
    }

    /// Returns `true` if the plugin is a python module.
    #[cfg(feature = "python_support_enabled")]
    pub fn is_python_module(&self) -> bool {
        todo!()
    }

    /// Returns `true` if the plugin is resource-only.
    pub fn is_resource(&self) -> bool {
        todo!()
    }

    /// Returns the dictionary containing meta-data for the plugin.
    pub fn get_metadata(&self) -> Json {
        todo!()
    }

    /// Returns the metadata sub-dictionary for a particular type.
    pub fn get_metadata_for_type<Type>(&self, plug_type: &Type) -> Json {
        todo!()
    }

    /// Returns the dictionary containing the dependencies for the plugin.
    pub fn get_dependencies(&self) -> Json {
        todo!()
    }

    /// Returns true if `type` is declared by this plugin.
    /// If `include_subclasses` is specified, also returns true if any
    /// subclasses of `type` have been declared.
    pub fn declares_type<Type>(&self, plug_type: Type, include_subclasses: bool) -> bool {
        todo!()
    }

    /// Returns the plugin's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the plugin's filesystem path.
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Returns the plugin's resources filesystem path.
    pub fn get_resource_path(&self) -> &str {
        &self.resource_path
    }

    /// Build a plugin resource path by returning a given absolute path or
    /// combining the plugin's resource path with a given relative path.
    pub fn make_resource_path(&self, path: &str) -> &str {
        todo!()
    }

    /// Find a plugin resource by absolute or relative path optionally
    /// verifying that file exists. If verification fails an empty path
    /// is returned. Relative paths are relative to the plugin's resource
    /// path.
    pub fn find_plugin_resource(&self, path: &str, verify: bool) -> &str {
        todo!()
    }

    // Crate level methods

    /// Private constructor, plugins are constructed only by [`Registry`].
    pub(crate) fn new(
        path: &str,
        name: &str,
        resource_path: &str,
        plug_info: Json,
        plug_type: PluginType,
    ) -> Self {
        todo!()
    }

    pub(crate) fn get_plugin_for_type<Type>(plug_type: Type) -> Rc<RefCell<Plugin>> {
        todo!()
    }

    pub(crate) fn register_all_plugins() {
        todo!()
    }

    pub(crate) fn get_plugin_with_name(name: &str) -> Rc<RefCell<Plugin>> {
        todo!()
    }

    pub(crate) fn get_all_plugins() -> Vec<Rc<RefCell<Plugin>>> {
        todo!()
    }
}
