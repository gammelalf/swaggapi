#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "axum")]
mod axum;

#[cfg(feature = "axum")]
pub use axum::RouterExt;
use openapiv3::Responses;
use schemars::gen::SchemaGenerator;

use crate::handler_argument::HandlerArgumentFns;
use crate::Method;

/// Meta information about a handler gathered by the [`#[operation]`](operation) macro
#[derive(Copy, Clone, Debug)]
pub struct Handler {
    /// The http method the handler handles
    pub method: Method,

    /// The handler's path
    pub path: &'static str,

    /// A common path to prefix `path` with
    pub ctx_path: &'static str,

    /// `true` if `#[deprecated]` is present
    pub deprecated: bool,

    /// Set by macro if `#[doc = "..."]` (i.e. a doc comment) is present
    pub doc: &'static [&'static str],

    /// The handler's identifier
    pub ident: &'static str,

    /// The handler's return type's [`AsResponses::responses`]
    pub responses: fn(&mut SchemaGenerator) -> Responses,

    /// The handler's arguments' [`HandlerArgument`]'s methods
    pub handler_arguments: &'static [Option<HandlerArgumentFns>],

    /// The actual function stored in an actix specific format
    #[cfg(feature = "actix")]
    pub actix: fn() -> actix_web::Route,
    /// Placeholder to make the macro code cleaner
    #[cfg(not(feature = "actix"))]
    pub actix: (),

    /// The actual function stored in an axum specific format
    #[cfg(feature = "axum")]
    pub axum: fn() -> ::axum::routing::MethodRouter,
    /// Placeholder to make the macro code cleaner
    #[cfg(not(feature = "axum"))]
    pub axum: (),
}

#[cfg(not(feature = "actix"))]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_actix {
    ($ident:path: fn($($arg:ty),*) -> $ret:ty) => {
        ()
    };
}
#[cfg(not(feature = "axum"))]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_axum {
    ($ident:path: fn($($arg:ty),*) -> $ret:ty) => {
        ()
    };
}
