use alloc::string::{String, ToString};

use serde::Deserialize;
use serde_json::Value as Json;

use crate::plugin::PluginType;

fn default_path() -> String {
    ".".to_string()
}

#[cfg(not(feature = "std"))]
fn append_to_root_path(root_path_name: &str, sub_path_name: &str) -> String {
    use alloc::format;

    if sub_path_name.is_empty() {
        return root_path_name.to_string();
    }

    if sub_path_name.starts_with("/") {
        return sub_path_name.to_string();
    }

    format!("{}/{}", root_path_name, sub_path_name)
}

#[cfg(feature = "std")]
fn append_to_root_path(root_path_name: &str, sub_path_name: &str) -> String {
    extern crate std;
    use std::path::Path;

    if sub_path_name.is_empty() {
        return root_path_name.to_string();
    }

    if Path::new(sub_path_name).is_absolute() {
        return sub_path_name.to_string();
    }

    Path::new(root_path_name)
        .join(sub_path_name)
        .to_str()
        .unwrap()
        .to_string()
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistrationMetadata {
    #[serde(rename = "Type")]
    pub plugin_type: PluginType,

    #[serde(rename = "Name")]
    pub name: String,

    #[cfg(feature = "std")]
    #[serde(rename = "Root", default = "default_path")]
    pub plugin_path: String,

    #[cfg(not(feature = "std"))]
    #[serde(rename = "Root")]
    pub plugin_path: String,

    #[serde(rename = "LibraryPath")]
    library_path: String,

    #[serde(rename = "ResourcePath", default = "default_path")]
    resource_path: String,

    #[serde(rename = "Info")]
    pub info: Json,
}

impl RegistrationMetadata {
    pub fn library_path(&self) -> String {
        append_to_root_path(&self.plugin_path, &self.library_path)
    }

    pub fn resource_path(&self) -> String {
        append_to_root_path(&self.plugin_path, &self.resource_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
    {
        "Type": "library",
        "Name": "MyPlugin",
        "Root": "/foo",
        "LibraryPath": "lib",
        "ResourcePath": "resources",
        "Info": {
            "value": 1
        }
    }"#;

        let metadata: RegistrationMetadata = serde_json::from_str(data).unwrap();
        assert_eq!(metadata.name, "MyPlugin");
        assert_eq!(metadata.plugin_type, PluginType::Library);
        assert_eq!(metadata.plugin_path, "/foo");
        assert_eq!(metadata.library_path(), "/foo/lib");
        assert_eq!(metadata.resource_path(), "/foo/resources");
        assert_eq!(metadata.info["value"], 1)
    }

    #[test]
    fn type_cannot_be_unknown() {
        let data = r#"
    {
        "Type": "unknown",
        "Name": "MyPlugin",
        "Root": "/foo",
        "LibraryPath": "lib",
        "ResourcePath": "resources",
        "Info": {
            "value": 1
        }
    }"#;

        let result = serde_json::from_str::<RegistrationMetadata>(data);
        assert!(result.is_err());
    }

    #[cfg(not(feature = "std"))]
    #[test]
    fn test_root_path_is_required_in_no_std() {
        let data = r#"
    {
        "Type": "library",
        "Name": "MyPlugin",
        "LibraryPath": "lib",
        "ResourcePath": "resources",
        "Info": {
            "value": 1
        }
    }"#;

        let metadata: Result<RegistrationMetadata, serde_json::Error> = serde_json::from_str(data);
        assert!(metadata.is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn root_path_defaults_in_std() {
        let data = r#"
    {
        "Type": "library",
        "Name": "MyPlugin",
        "LibraryPath": "lib",
        "ResourcePath": "resources",
        "Info": {
            "value": 1
        }
    }"#;

        let metadata: RegistrationMetadata = serde_json::from_str(data).unwrap();
        assert_eq!(metadata.plugin_path, ".")
    }

    #[test]
    fn resource_path_defaults() {
        let data = r#"
    {
        "Type": "library",
        "LibraryPath": "lib",
        "Name": "MyPlugin",
        "Root": "/root",
        "Info": {
            "value": 1
        }
    }"#;

        let metadata: RegistrationMetadata = serde_json::from_str(data).unwrap();
        assert_eq!(metadata.resource_path(), "/root/.")
    }
}
