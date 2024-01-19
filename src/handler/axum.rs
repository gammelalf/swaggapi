use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use axum::Router;

use crate::Handler;

#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_axum {
    ($module:ident::$ident:ident: fn($($arg:ty),*) -> $ret:ty) => {
        || {
            <$crate::PageOfEverything as $crate::SwaggapiPage>::builder().add_handler(&$ident);
            $crate::re_exports::axum::routing::MethodRouter::new()
                .on($ident.method.axum(), $module::$ident)
        }
    };
}

/// Extension trait to give [`Router`] swaggapi support
pub trait RouterExt {
    /// Add a set of swaggapi [`Handler`]s to the router
    fn routes<const N: usize>(self, handlers: [Handler; N]) -> Self;
}
impl RouterExt for Router {
    fn routes<const N: usize>(self, handlers: [Handler; N]) -> Self {
        let mut routes = BTreeMap::new();
        for handler in handlers {
            match routes.entry(handler.path) {
                Entry::Vacant(entry) => {
                    entry.insert((handler.axum)());
                }
                Entry::Occupied(entry) => {
                    let (path, method_router) = entry.remove_entry();
                    routes.insert(path, method_router.merge((handler.axum)()));
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
