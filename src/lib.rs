// #![warn(missing_docs)]
#![warn(clippy::todo)]

pub mod as_responses;
mod convert;
mod description;
pub mod handler;
pub mod handler_argument;
mod method;

pub use convert::convert_schema;
pub use swaggapi_macro::operation;

pub use self::description::OperationDescription;
pub use self::method::Method;

/// Reexports for macros and implementors
pub mod re_exports {
    #[cfg(feature = "actix")]
    pub use actix_web;
    #[cfg(feature = "axum")]
    pub use axum;
    pub use {indexmap, openapiv3, schemars};
}

use openapiv3::OpenAPI;

trait SwaggapiPage {
    fn builder() -> &'static SwaggapiPageBuilder;
}

struct SwaggapiPageBuilder {}

impl SwaggapiPageBuilder {
    fn build(&self) -> OpenAPI {
        todo!()
    }
}

#[derive(Default)]
pub struct OperationBuilder {
    /// `true` if `#[deprecated]` is present
    pub deprecated: bool,

    /// Set by macro if `#[doc = "..."]` (i.e. a doc comment) is present
    pub doc: Vec<&'static str>,

    /// The handler's identifier
    pub ident: &'static str,

    pub responses: Responses,

    pub request_body: Vec<RequestBody>,

    pub parameters: Vec<Parameter>,
}

impl OperationBuilder {
    pub fn build(mut self) -> Operation {
        let mut doc = self.doc.into_iter();
        let summary = doc.next().map(|line| line.trim().to_string());
        let description = summary
            .clone()
            .map(|summary| doc.fold(summary, |text, line| format!("{text}\n{}", line.trim())));

        Operation {
            summary,
            description,
            operation_id: Some(self.ident.to_string()),
            parameters: self.parameters.into_iter().map(ReferenceOr::Item).collect(),
            request_body: self.request_body.pop().map(ReferenceOr::Item),
            responses: self.responses,
            deprecated: self.deprecated,
            security: None,   // TODO
            tags: Vec::new(), // TODO
            // Not supported:
            callbacks: Default::default(),
            external_docs: Default::default(),
            servers: Default::default(),
            extensions: Default::default(),
        }
    }
}
