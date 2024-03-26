#![warn(missing_docs)]
#![warn(clippy::todo)]

pub mod as_responses;
mod context;
pub mod handler_argument;
pub mod internals;
mod page;
#[cfg(feature = "swagger-ui")]
mod swagger;
pub mod utils;

pub use swaggapi_macro::*;

pub use self::context::ApiContext;
pub use self::page::PageOfEverything;
pub use self::page::SwaggapiPage;
#[cfg(feature = "swagger-ui")]
pub use self::swagger::SwaggerUi;

/// Reexports for macros and implementors
pub mod re_exports {
    #[cfg(feature = "actix")]
    pub use actix_web;
    #[cfg(feature = "axum")]
    pub use axum;
    pub use indexmap;
    pub use mime;
    pub use openapiv3;
    pub use schemars;
    #[cfg(feature = "swagger-ui")]
    pub use swagger_ui;
}
