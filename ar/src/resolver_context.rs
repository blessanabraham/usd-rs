mod client_context;
pub use client_context::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "resolver_v1")] {
        mod resolver_context_v1;
        pub use resolver_context_v1::ResolverContext;
    } else if #[cfg(feature = "resolver_v2")] {
        mod resolver_context_v2;
        pub use resolver_context_v2::ResolverContext;
    }
}
