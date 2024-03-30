use actix_web::web;
use actix_web::web::Redirect;
use actix_web::HttpResponse;
use indexmap::IndexMap;
use openapiv3::Header;
use openapiv3::ParameterSchemaOrContent;
use openapiv3::ReferenceOr;
use openapiv3::Response;
use openapiv3::Responses;
use openapiv3::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;

use crate::as_responses::ok_binary;
use crate::as_responses::ok_json;
use crate::as_responses::ok_text;
use crate::as_responses::AsResponses;
use crate::internals::SchemaGenerator;

impl AsResponses for &'static str {
    fn responses(_: &mut SchemaGenerator) -> Responses {
        ok_text()
    }
}
impl AsResponses for String {
    fn responses(_: &mut SchemaGenerator) -> Responses {
        ok_text()
    }
}

impl AsResponses for &'static [u8] {
    fn responses(_: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}
impl AsResponses for Vec<u8> {
    fn responses(_: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl<T: Serialize + JsonSchema> AsResponses for web::Json<T> {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        ok_json::<T>(gen)
    }
}

impl<T: AsResponses, E: AsResponses> AsResponses for Result<T, E> {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        let ok = T::responses(gen);
        let err = E::responses(gen);

        Responses {
            default: ok.default.or(err.default),
            responses: ok
                .responses
                .into_iter()
                .chain(err.responses.into_iter())
                .collect(),
            extensions: ok
                .extensions
                .into_iter()
                .chain(err.extensions.into_iter())
                .collect(),
        }
    }
}

impl AsResponses for Redirect {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        Responses {
            responses: IndexMap::from_iter([(
                StatusCode::Range(3),
                ReferenceOr::Item(Response {
                    description: "A generic http redirect".to_string(),
                    headers: IndexMap::from_iter([(
                        "Location".to_string(),
                        ReferenceOr::Item(Header {
                            description: None,
                            style: Default::default(),
                            required: false,
                            deprecated: None,
                            format: ParameterSchemaOrContent::Schema(gen.generate::<String>()),
                            example: None,
                            examples: Default::default(),
                            extensions: Default::default(),
                        }),
                    )]),
                    ..Default::default()
                }),
            )]),
            ..Default::default()
        }
    }
}

impl AsResponses for actix_web::Error {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Responses {
            default: Some(ReferenceOr::Item(Response {
                description: "Some error".to_string(),
                ..Default::default()
            })),
            ..Default::default()
        }
    }
}

impl AsResponses for HttpResponse {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Default::default()
    }
}

#[cfg(feature = "actix-files")]
impl AsResponses for actix_files::NamedFile {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Responses {
            responses: IndexMap::from_iter([(
                StatusCode::Code(200),
                ReferenceOr::Item(Response {
                    description: "A downloadable file".to_string(),
                    ..Default::default()
                }),
            )]),
            ..Default::default()
        }
    }
}
