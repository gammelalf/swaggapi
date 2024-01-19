use openapiv3::Responses;
use schemars::gen::SchemaGenerator;

use crate::handler::Handler;
use crate::handler_argument::HandlerArgumentFns;
use crate::Method;

/// Meta information about a handler gathered by the [`#[operation]`](operation) macro
pub struct HandlerDescription {
    /// The http method the handler handles
    pub method: Method,

    /// The handler's path
    pub path: &'static str,

    /// A common path to prefix [`Handler::path`] with
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
}
