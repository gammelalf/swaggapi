use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use axum::Router;

use crate::handler::Handler;
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_axum {
    ($ident:path: fn($($arg:ty),*) -> $ret:ty) => {
        fn axum(&self) -> $crate::re_exports::axum::routing::MethodRouter {
            <$crate::PageOfEverything as $crate::SwaggapiPage>::builder().add_handler(self);
            $crate::re_exports::axum::routing::MethodRouter::new()
                .on(self.description().method.axum(), $ident)
        }
    };
}

/// Extension trait to give [`Router`] swaggapi support
pub trait RouterExt {
    /// Add a set of swaggapi [`Handler`]s to the router
    fn routes(self, handlers: &[&dyn Handler]) -> Self;
}
impl RouterExt for Router {
    fn routes(self, handlers: &[&dyn Handler]) -> Self {
        let mut routes = BTreeMap::new();
        for handler in handlers {
            let desc = handler.description();
            match routes.entry(desc.path) {
                Entry::Vacant(entry) => {
                    entry.insert(handler.axum());
                }
                Entry::Occupied(entry) => {
                    let (path, method_router) = entry.remove_entry();
                    routes.insert(path, method_router.merge(handler.axum()));
                }
            }
        }
        routes
            .into_iter()
            .fold(self, |router, (path, method_router)| {
                router.route(path, method_router)
            })
    }
}
