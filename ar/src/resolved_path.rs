use alloc::borrow::ToOwned;
use alloc::fmt;
use alloc::string::String;

/// Represents a resolved asset path.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResolvedPath(String);

impl ResolvedPath {
    /// Constructor
    pub fn new(path: &str) -> Self {
        ResolvedPath(path.to_owned())
    }

    /// Return the resolved path held by this object as a string.
    pub fn get_path_string(&self) -> &str {
        &self.0
    }

    /// Return true if this object is holding an empty resolved path,
    /// false otherwise.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for ResolvedPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
