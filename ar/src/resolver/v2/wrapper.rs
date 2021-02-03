use alloc::boxed::Box;

use super::Resolver;

/// Private ArResolver implementation that owns and forwards calls to the
/// plugin asset resolver implementation. This is used to overlay additional
/// behaviors on top of the plugin resolver.
pub(super) struct ResolverWrapper {
    resolver: Box<dyn Resolver>,
    max_uri_scheme_length: usize,
}

impl ResolverWrapper {
    ///
    pub fn get_primary_resolver(&self) -> &dyn Resolver {
        self.resolver.as_ref()
    }
}

// impl Resolver for ResolverWrapper {
//     fn configure_resolver_for_asset(&mut self, path: &str) {}
//
//     fn create_context_from_string(&self, uri_scheme: &str, context_str: &str) -> ResolverContext {}
// }
