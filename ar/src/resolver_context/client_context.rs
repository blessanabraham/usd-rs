use alloc::boxed::Box;
use core::any::Any;
use core::cmp::Ordering;
use core::fmt::Debug;

/// The trait for Client Contexts
pub trait ClientContext: ClientContextClone + Debug + ClientContextCmp {}

#[doc(hidden)]
/// Trait to clone a [`ClientContext`]
pub trait ClientContextClone {
    /// Method to clone a [`ClientContext`]
    fn clone_box(&self) -> Box<dyn ClientContext>;
}

impl<Context> ClientContextClone for Context
where
    Context: 'static + ClientContext + Clone,
{
    fn clone_box(&self) -> Box<dyn ClientContext> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ClientContext> {
    fn clone(&self) -> Box<dyn ClientContext> {
        self.clone_box()
    }
}

#[doc(hidden)]
/// Trait to compare two Client Contexts
pub trait ClientContextCmp {
    /// An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;

    /// Perform the equality tests.
    fn eq_box(&self, other: &dyn ClientContextCmp) -> bool;

    /// Perform the partial comparison tests
    fn partial_cmp_box(&self, other: &dyn ClientContextCmp) -> Option<Ordering>;
}

impl<Context> ClientContextCmp for Context
where
    Context: 'static + PartialEq + PartialOrd,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_box(&self, other: &dyn ClientContextCmp) -> bool {
        // Do a type-safe casting. If the types are different,
        // return false, otherwise tests the values for equality.
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |ctx| ctx == self)
    }

    fn partial_cmp_box(&self, other: &dyn ClientContextCmp) -> Option<Ordering> {
        // Do a type-safe casting. If the types are different,
        // return None, otherwise tests the values for order.
        other
            .as_any()
            .downcast_ref::<Self>()
            .and_then(|ctx| self.partial_cmp_box(ctx))
    }
}

impl PartialOrd for Box<dyn ClientContext> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_cmp_box(other)
    }
}

// impl Ord for Box<dyn ClientContext> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.cmp(other)
//     }
// }

impl PartialEq for Box<dyn ClientContext> {
    fn eq(&self, other: &Self) -> bool {
        self.eq_box(other)
    }
}

impl Eq for Box<dyn ClientContext> {}
