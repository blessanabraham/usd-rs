use alloc::boxed::Box;
use alloc::vec::{IntoIter, Vec};
use core::cmp::PartialOrd;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::AddAssign;

use super::ClientContext;

/// An asset resolver context allows clients to provide additional data
/// to the resolver for use during resolution. Clients may provide this
/// data via context objects of their own (subject to restrictions below).
/// An ArResolverContext is simply a wrapper around these objects that
/// allows it to be treated as a single type. Note that an ArResolverContext
/// may not hold multiple context objects with the same type.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct ResolverContext {
    contexts: Vec<Box<dyn ClientContext>>,
}

impl ResolverContext {
    /// Constructor
    ///
    /// # Examples
    /// ```
    /// use ar::{ResolverContext};
    ///
    /// ResolverContext::new();
    /// ```
    pub fn new() -> Self {
        Self { contexts: vec![] }
    }

    /// Push a client context
    pub fn push<Context>(&mut self, context: Context) -> &mut Self
    where
        Context: 'static + ClientContext + Debug + PartialOrd + PartialEq + Hash,
    {
        self.contexts.push(Box::new(context));
        self
    }

    /// Return pointer to the context object held in this asset resolver
    /// context
    ///
    /// # Examples
    /// ```
    /// use ar::{ClientContext, ResolverContext};
    ///
    /// #[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
    /// struct Context1 {
    ///     id: usize,
    /// }
    ///
    /// impl ClientContext for Context1 {}
    ///
    /// #[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
    /// struct Context2 {
    ///     data: usize,
    /// }
    ///
    /// impl ClientContext for Context2 {}
    ///
    /// let mut resolver = ResolverContext::new();
    /// let ctx1 = Context1 { id: 1 };
    /// resolver.push(ctx1);
    /// assert!(resolver.get::<Context2>().is_none());
    /// assert!(resolver.get::<Context1>().is_some());
    /// ```
    pub fn get<Context>(&self) -> Option<&dyn ClientContext>
    where
        Context: 'static + ClientContext,
    {
        for context in &self.contexts {
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
    ///
    /// # Examples
    /// ```
    /// use ar::{ClientContext, ResolverContext};
    ///
    /// #[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
    /// struct Context {
    ///     id: usize,
    /// }
    ///
    /// impl ClientContext for Context {}
    ///
    /// let client_ctx = Context { id: 1 };
    /// let mut resolver = ResolverContext::new();
    /// resolver.push(client_ctx);
    /// assert_eq!(resolver.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.contexts.is_empty()
    }
}

impl IntoIterator for ResolverContext {
    type Item = Box<dyn ClientContext>;
    type IntoIter = IntoIter<Box<dyn ClientContext>>;

    fn into_iter(self) -> Self::IntoIter {
        self.contexts.into_iter()
    }
}

impl AddAssign for ResolverContext {
    fn add_assign(&mut self, rhs: Self) {
        self.contexts.extend(rhs.contexts);
    }
}
