pub use error::*;

mod error;
cfg_if::cfg_if! {
    if #[cfg(feature = "resolver_v1")] {
        mod v1;
        pub use v1::*;
    } else if #[cfg(feature = "resolver_v2")] {
        mod v2;
        pub use v2::*;
    }
}
