use alloc::string::String;

/// Contains information about a resolved asset.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AssetInfo {
    /// Version of the resolved asset, if any.
    version: Option<String>,

    /// The name of the asset represented by the resolved
    /// asset, if any.
    asset_name: Option<String>,

    /// The repository path corresponding to the resolved asset.
    repo_path: String,
    // Additional information specific to the active plugin
    // asset resolver implementation.
    // resolver_info: Box<&dyn Any>,
}
