#![no_std]

extern crate alloc;

pub use plugin_map::*;
pub use registry::*;

pub mod info;

mod plugin_map;
mod registry;
mod plugin;
