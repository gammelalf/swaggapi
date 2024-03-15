use std::borrow::Cow;

use axum::Json;
use bytes::{Bytes, BytesMut};
use indexmap::IndexMap;
use openapiv3::{MediaType, ReferenceOr, Response, Responses, StatusCode};
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use serde::Serialize;

use crate::as_responses::{simple_responses, AsResponses, SimpleResponse};
use crate::internals::convert_schema;

impl AsResponses for &'static str {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, str>::responses(_gen)
    }
}

impl AsResponses for String {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, str>::responses(_gen)
    }
}

impl AsResponses for Box<str> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, str>::responses(_gen)
    }
}

impl AsResponses for Cow<'static, str> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        simple_responses([SimpleResponse {
            status_code: StatusCode::Code(200),
            mime_type: mime::TEXT_PLAIN_UTF_8,
            description: "Some plain text".to_string(),
            media_type: None,
        }])
    }
}

impl AsResponses for &'static [u8] {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, [u8]>::responses(_gen)
    }
}

impl AsResponses for Vec<u8> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, [u8]>::responses(_gen)
    }
}

impl AsResponses for Box<[u8]> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, [u8]>::responses(_gen)
    }
}

impl AsResponses for Bytes {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, [u8]>::responses(_gen)
    }
}

impl AsResponses for BytesMut {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        Cow::<'static, [u8]>::responses(_gen)
    }
}

impl AsResponses for Cow<'static, [u8]> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        simple_responses([SimpleResponse {
            status_code: StatusCode::Code(200),
            mime_type: mime::APPLICATION_OCTET_STREAM,
            description: "Some binary data".to_string(),
            media_type: None,
        }])
    }
}

impl<T: Serialize + JsonSchema> AsResponses for Json<T> {
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
