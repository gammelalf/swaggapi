//! The [`AsResponses`] trait, its implementations and utilises for implementing it.

use mime::Mime;
use openapiv3::{MediaType, ReferenceOr, Response, Responses, StatusCode};
use schemars::gen::SchemaGenerator;

#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "axum")]
mod axum;

/// A type returned by a handler which can be described with a [responses object](https://spec.openapis.org/oas/v3.0.3#responses-object)
///
/// This type should be implemented by everything which implements
/// [`IntoResponse`] when using [axum] or
/// [`Responder`] when using [actix]
pub trait AsResponses {
    /// Get the [responses object](https://spec.openapis.org/oas/v3.0.3#responses-object) describing `Self`
    fn responses(gen: &mut SchemaGenerator) -> Responses;
}

/// Helper function for building a [`Responses`] for some simple responses
pub fn simple_responses(responses: impl IntoIterator<Item = SimpleResponse>) -> Responses {
    Responses {
        responses: responses
            .into_iter()
            .map(|response: SimpleResponse| {
                (
                    response.status_code,
                    ReferenceOr::Item(Response {
                        description: response.description,
                        content: FromIterator::from_iter([(
                            response.mime_type.to_string(),
                            response.media_type.unwrap_or_default(),
                        )]),
                        ..Default::default()
                    }),
                )
            })
            .collect(),
        ..Default::default()
    }
}

/// Describes the response for a specific status code
pub struct SimpleResponse {
    /// The response's status code
    pub status_code: StatusCode,

    /// The response's mime type
    pub mime_type: Mime,

    /// A short description of the response.
    /// CommonMark syntax MAY be used for rich text representation.
    pub description: String,

    /// Optional more details explanation of the response's data
    pub media_type: Option<MediaType>,
}
