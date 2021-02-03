use alloc::fmt;
use alloc::string::String;

/// Errors for [`WritableAsset`]
#[derive(Clone, Debug)]
pub enum WritableAssetError {
    /// Raised when failed to close an asset
    CloseFailed(String),

    /// Raised when failed to write to an asset
    WriteFailed(String),
}

impl fmt::Display for WritableAssetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WritableAssetError::CloseFailed(reason) => {
                write!(f, "failed to close asset: {}", reason)
            }
            WritableAssetError::WriteFailed(reason) => {
                write!(f, "failed to save changes to asset: {}", reason)
            }
        }
    }
}

/// Interface for writing data to an asset.
///
/// see [`Resolver::open_asset_for_write()`] for how to retrieve instances of
/// this object.
pub trait WritableAsset {
    /// Close this asset, performing any necessary finalization or commits
    /// of data that was previously written. Returns [`WriteAssetError`]
    /// on failure.
    ///
    /// If successful, reads to the written asset in the same process should
    /// reflect the fully written state by the time this function returns.
    /// Also, further calls to any functions on this interface are invalid.
    fn close(&mut self) -> Result<(), WritableAssetError>;

    /// Writes `count` bytes from `buffer` at `offset` from the beginning
    /// of the asset. Returns number of bytes written, or [`WritableAssetError`].
    fn write(
        &mut self,
        buffer: &[u8],
        count: usize,
        offset: usize,
    ) -> Result<usize, WritableAssetError>;
}
