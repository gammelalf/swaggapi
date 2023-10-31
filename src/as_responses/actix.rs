use crate::as_responses::{simple_responses, AsResponses, SimpleResponse};
use crate::convert_schema;
use actix_web::{web, HttpResponse};
use indexmap::IndexMap;
use openapiv3::{MediaType, ReferenceOr, Response, Responses, StatusCode};
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use serde::Serialize;

impl AsResponses for &'static str {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        simple_responses([SimpleResponse {
            status_code: StatusCode::Code(200),
            mime_type: mime::TEXT_PLAIN_UTF_8,
            description: "Some plain text".to_string(),
            media_type: None,
        }])
    }
}
impl AsResponses for String {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        <&'static str>::responses(gen)
    }
}

impl AsResponses for &'static [u8] {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        simple_responses([SimpleResponse {
            status_code: StatusCode::Code(200),
            mime_type: mime::APPLICATION_OCTET_STREAM,
            description: "Some binary data".to_string(),
            media_type: None,
        }])
    }
}
impl AsResponses for Vec<u8> {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        <&'static [u8]>::responses(gen)
    }
}

impl<T: Serialize + JsonSchema> AsResponses for web::Json<T> {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        let schema = convert_schema(gen.subschema_for::<T>());
        Responses {
            responses: IndexMap::from_iter([
                (
                    StatusCode::Code(200),
                    ReferenceOr::Item(Response {
                        description: "".to_string(), // TODO take the schema's
                        content: IndexMap::from_iter([(
                            mime::APPLICATION_JSON.to_string(),
                            MediaType {
                                schema: Some(schema),
                                ..Default::default()
                            },
                        )]),
                        ..Default::default()
                    }),
                ),
                // TODO add error types
            ]),
            ..Default::default()
        }
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

impl AsResponses for HttpResponse {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Default::default()
    }
}
