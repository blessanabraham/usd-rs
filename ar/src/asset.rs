use alloc::string::String;
use core::fmt;

/// Asset errors
#[derive(Debug, Clone)]
pub enum AssetError {
    /// Asset read error
    ReadError(String),
}

impl fmt::Display for AssetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetError::ReadError(s) => write!(f, "failed to read asset: {}", s),
        }
    }
}

/// Trait for accessing the contents of an asset
/// See [`crate::Resolver::open_asset()`] for how to retrieve instances of this object.
pub trait Asset {
    /// Returns size of the asset.
    fn get_size(&self) -> usize;

    /// Returns a buffer with the contents of the asset,
    /// with size given by [`Asset::get_size`]. Returns an error
    /// if the contents could not be retrieved.
    fn get_buffer(&self) -> Result<&[u8], AssetError>;

    /// Read `count` bytes at `offset` from the beginning of the asset
    /// into `buffer`. Returns number of bytes read, or error.
    ///
    /// Implementers should range-check calls and return error for out-of-bounds
    /// reads.
    fn read(&self, buffer: &mut [u8], count: usize, offset: usize) -> Result<usize, AssetError>;
}
