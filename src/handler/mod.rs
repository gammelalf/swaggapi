#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "axum")]
mod axum;
mod description;

#[cfg(feature = "axum")]
pub use axum::RouterExt;
pub use description::HandlerDescription;

/// A function annotated with [`#[swaggapi::get]`](swaggapi_macro::get) or one of its siblings.
pub trait Handler {
    /// Build the handler's description registering required schemas
    fn description(&self) -> HandlerDescription;

    #[cfg(feature = "actix")]
    #[doc(hidden)]
    fn actix(&self) -> ::actix_web::Route;

    #[cfg(feature = "axum")]
    #[doc(hidden)]
    fn axum(&self) -> ::axum::routing::MethodRouter;
}

#[cfg(not(feature = "actix"))]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_actix {
    ($ident:path: fn($($arg:ty),*) -> $ret:ty) => {};
}
#[cfg(not(feature = "axum"))]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_axum {
    ($ident:path: fn($($arg:ty),*) -> $ret:ty) => {};
}
