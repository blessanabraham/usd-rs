use usd_plugin::info::{PluginInfo, PluginVariants};

#[test]
fn deserialize_single_plugin_info() {
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

    let plugin_info: PluginInfo = serde_json::from_str::<PluginVariants>(data).unwrap().into();
    assert_eq!(plugin_info.includes.len(), 0);
    assert_eq!(plugin_info.plugins.len(), 1)
}

#[test]
fn deserialize_multi_plugin_info() {
    let data = r#"
    {
        "Includes": [
            "/absolute/path/to/plugInfo.json",
            "/absolute/path/to/custom.filename",
            "/absolute/path/to/directory/with/plugInfo/",
            "relative/path/to/plugInfo.json",
            "relative/path/to/directory/with/plugInfo/",
            "glob*/pa*th/*to*/*/plugInfo.json",
            "recursive/pa**th/**/"
        ],
        "Plugins": [
            {
                "Type": "library",
                "Name": "MyPlugin",
                "Root": "/foo",
                "LibraryPath": "lib",
                "ResourcePath": "resources",
                "Info": {
                    "value": 1
                }
            }
        ]
    }"#;

    let plugin_info: PluginInfo = serde_json::from_str::<PluginVariants>(data).unwrap().into();
    assert_eq!(plugin_info.includes.len(), 7);
    assert_eq!(plugin_info.plugins.len(), 1)
}
