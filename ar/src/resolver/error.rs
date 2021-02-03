use alloc::string::String;
use core::fmt;

/// Resolver errors
#[derive(Clone, Debug)]
pub enum ResolverError {
    /// Resolver failed to open asset
    OpenAssetError(String),

    /// Resolver failed to get the last modified time of the asset
    AssetMtimeError,

    /// Resolver cannot write layer to path
    CannotWriteLayerToPath(String, String),

    /// Resolver cannot create a new layer with the given identifier
    CannotCreateNewLayerWithIdentifier(String, String),
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolverError::OpenAssetError(s) => write!(f, "failed to open asset: {}", s),
            ResolverError::AssetMtimeError => {
                write!(f, "failed to get asset's modified time")
            }
            ResolverError::CannotWriteLayerToPath(path, reason) => {
                write!(f, "cannot write layer to path `{}`: {}", path, reason)
            }
            ResolverError::CannotCreateNewLayerWithIdentifier(identifier, reason) => {
                write!(
                    f,
                    "cannot create new layer with identifier `{}`: {}",
                    identifier, reason
                )
            }
        }
    }
}
