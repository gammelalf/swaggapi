use schemars::gen::SchemaGenerator;

use crate::{Method, OperationDescription};

#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "axum")]
mod axum;
#[cfg(feature = "axum")]
pub use axum::RouterExt;

pub trait Handler {
    fn method(&self) -> Method;
    fn path(&self) -> &'static str;
    fn ctx_path(&self) -> &'static str;
    fn description(&self, gen: &mut SchemaGenerator) -> OperationDescription;

    fn as_dyn(&self) -> &dyn Handler
    where
        Self: Sized,
    {
        self
    }

    #[cfg(feature = "actix")]
    fn actix(&self) -> ::actix_web::Route;

    #[cfg(feature = "axum")]
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
