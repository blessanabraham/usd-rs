use alloc::string::String;
use alloc::vec::Vec;

use serde::Deserialize;

pub use registration_metadata::*;

mod registration_metadata;

type SinglePlugin = RegistrationMetadata;

const fn default_plugins() -> Vec<RegistrationMetadata> {
    Vec::new()
}

const fn default_includes() -> Vec<String> {
    Vec::new()
}

#[derive(Clone, Debug, Deserialize)]
pub struct PluginInfo {
    #[serde(rename = "Plugins", default = "default_plugins")]
    pub plugins: Vec<RegistrationMetadata>,

    #[serde(rename = "Includes", default = "default_includes")]
    pub includes: Vec<String>,
}

impl From<SinglePlugin> for PluginInfo {
    fn from(info: SinglePlugin) -> Self {
        Self {
            includes: Vec::new(),
            plugins: Vec::from([info]),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum PluginVariants {
    Single(SinglePlugin),
    Multi(PluginInfo),
}

impl From<PluginVariants> for PluginInfo {
    fn from(variants: PluginVariants) -> Self {
        match variants {
            PluginVariants::Single(info) => info.into(),
            PluginVariants::Multi(info) => info,
        }
    }
}
