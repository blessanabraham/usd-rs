use alloc::string::{String, ToString};

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        extern crate std;
        use std::sync::{Arc, Mutex};

        lazy_static! {
            pub static ref PREFERRED_RESOLVER: Arc<Mutex<String>> = Arc::new(Mutex::new(String::default()));
        }

        pub fn set_preferred_resolver(preferred_resolver: &str) {
            let mut resolver = PREFERRED_RESOLVER.lock().unwrap();
             *resolver = preferred_resolver.to_string();
        }

        pub fn get_preferred_resolver() -> Option<String> {
            if let Ok(resolver) = PREFERRED_RESOLVER.lock() {
                if !resolver.is_empty() {
                    return Some(resolver.clone());
                }
            }

            None
        }
    } else {
        static mut PREFERRED_RESOLVER: String = String::new();

        pub fn set_preferred_resolver(preferred_resolver: &str) {
            unsafe {
                PREFERRED_RESOLVER = preferred_resolver.to_string();
            }
        }

        pub fn get_preferred_resolver() -> Option<String> {
            let resolver = unsafe { PREFERRED_RESOLVER.to_string() };
            if resolver.is_empty() {
                None
            } else {
                Some(resolver)
            }
        }
    }
}

// Private ArResolver implementation that owns and forwards calls to the
// plugin asset resolver implementation. This is used to overlay additional
// behaviors on top of the plugin resolver.
pub(super) struct ResolverWrapper {}

impl ResolverWrapper {
    fn initialize_underlying_resolver(&mut self) {}
}
