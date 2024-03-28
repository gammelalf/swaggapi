use openapiv3::Responses;

use crate::handler_argument::HandlerArgumentFns;
use crate::internals::HttpMethod;
use crate::internals::SchemaGenerator;

/// Meta information about a handler gathered by the [`#[handler]`](crate::handler) macro
#[derive(Copy, Clone, Debug)]
pub struct SwaggapiHandler {
    /// The http method the handler handles
    pub method: HttpMethod,

    /// The handler's path
    pub path: &'static str,

    /// `true` if `#[deprecated]` is present
    pub deprecated: bool,

    /// Set by macro if `#[doc = "..."]` (i.e. a doc comment) is present
    pub doc: &'static [&'static str],

    /// The handler's identifier
    pub ident: &'static str,

    /// Tags set through `#[operation(..., tags(...))]`
    pub tags: &'static [&'static str],

    /// The handler's return type's [`AsResponses::responses`](crate::as_responses::AsResponses::responses)
    pub responses: fn(&mut SchemaGenerator) -> Responses,

    /// The handler's arguments' [`HandlerArgument`](crate::handler_argument::HandlerArgument)'s methods
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
    ($method:expr, $ident:ident) => {
        ()
    };
}
#[cfg(feature = "actix")]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_actix {
    ($method:expr, $ident:ident) => {
        || {
            $crate::re_exports::actix_web::Route::new()
                .method($method.actix())
                .to($ident)
        }
    };
}

#[cfg(not(feature = "axum"))]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_axum {
    ($method:expr, $ident:ident) => {
        ()
    };
}
#[cfg(feature = "axum")]
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_axum {
    ($method:expr, $ident:ident) => {
        || $crate::re_exports::axum::routing::MethodRouter::new().on($method.axum(), $ident)
    };
}
