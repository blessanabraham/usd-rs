#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! The Ar (asset resolution) library provides functions for interacting with asset paths in USD.
//! An asset path is a string that describes the location of an asset in a user's system.
//! These paths are used extensively throughout USD; for example, asset paths are used to specify
//! sub-layers, references, and other composition arcs in scene description.
//!
//! Ar's primary responsibility is to 'resolve' an asset path into a corresponding filesystem path.
//! By default, Ar assumes that all asset paths are simple filesystem paths and treats them accordingly.
//! However, clients may implement their own resolver class by implementing the [`Resolver`] interface to
//! provide custom resolution logic for asset paths. This allows clients to use asset paths that are
//! appropriate for their situation.
//!
//! For example, a client might have an asset management system that uses URLs to identify assets.
//! That client could implement a custom resolver that would resolve those URLs and fetch data from a server
//! to the local filesystem and return that filesystem path as the 'resolved' path.
//! This would enable the client to use these URLs in scene description (e.g., for referencing) for USD.

#![no_std]

#[macro_use]
extern crate alloc;
#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate std;

// #[macro_use]
// extern crate log;

pub use asset::*;
pub use asset_info::*;
pub use resolved_path::*;
pub use resolver::*;
pub use resolver_context::*;
pub use writable_asset::*;

mod plugin;

mod package_utils;

mod asset;
mod asset_info;
mod resolved_path;
mod resolver;
mod resolver_context;
mod writable_asset;
