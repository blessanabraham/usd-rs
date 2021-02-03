use alloc::boxed::Box;
use core::fmt::Debug;

use crate::ClientContext;

/// An asset resolver context allows clients to provide additional data
/// to the resolver for use during resolution. Clients may provide this
/// data via a context object of their own (subject to restrictions below).
/// An ArResolverContext is simply a wrapper around this object that
/// allows it to be treated as a single type.
///
/// A client-defined context object must implement the following traits:
///   - [`std::clone::Clone`]
///   - [`std::fmt::Debug`]
///   - [`std::cmp::PartialOrd`]
///   - [`std::cmp::PartialEq`]
///   - ['std::hash::Hash`]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct ResolverContext {
    context: Option<Box<dyn ClientContext>>,
}

impl ResolverContext {
    /// Constructor
    ///
    /// # Examples
    /// ```
    /// use ar::{ClientContext, ResolverContext};
    ///
    /// #[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
    /// struct Context1 {
    ///     id: usize,
    /// }
    ///
    /// impl ClientContext for Context1 {}
    ///
    /// let client_ctx = Context1 { id: 1 };
    /// ResolverContext::new(client_ctx);
    /// ```
    pub fn new(context: impl ClientContext + 'static) -> Self {
        Self {
            context: Some(Box::new(context)),
        }
    }

    /// Return pointer to the context object held in this asset resolver
    /// context
    ///
    /// # Examples
    /// ```
    /// use ar::{ClientContext, ResolverContext};
    ///
    /// #[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
    /// struct Context1 {
    ///     id: usize,
    /// }
    ///
    /// impl ClientContext for Context1 {}
    ///
    /// #[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
    /// struct Context2 {
    ///     id: usize,
    /// }
    ///
    /// impl ClientContext for Context2 {}
    ///
    /// let client_ctx = Context1 { id: 1 };
    /// let resolver = ResolverContext::new(client_ctx);
    /// assert!(resolver.get::<Context2>().is_none());
    /// assert!(resolver.get::<Context1>().is_some());
    /// ```
    pub fn get<Context>(&self) -> Option<&dyn ClientContext>
    where
        Context: ClientContext + 'static,
    {
        if let Some(context) = &self.context {
            if context
                .as_ref()
                .as_any()
                .downcast_ref::<Context>()
                .is_some()
            {
                return Some(context.as_ref());
            }
        }
        None
    }

    /// Returns whether this resolver context is empty.
    pub fn is_empty(&self) -> bool {
        self.context.is_some()
    }
}
