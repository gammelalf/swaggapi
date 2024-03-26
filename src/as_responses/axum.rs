use std::borrow::Cow;

use axum::Json;
use bytes::Bytes;
use bytes::BytesMut;
use openapiv3::Responses;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use serde::Serialize;

use crate::as_responses::ok_binary;
use crate::as_responses::ok_json;
use crate::as_responses::ok_text;
use crate::as_responses::AsResponses;

impl AsResponses for &'static str {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_text()
    }
}

impl AsResponses for String {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_text()
    }
}

impl AsResponses for Box<str> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_text()
    }
}

impl AsResponses for Cow<'static, str> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_text()
    }
}

impl AsResponses for &'static [u8] {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl AsResponses for Vec<u8> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl AsResponses for Box<[u8]> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl AsResponses for Bytes {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl AsResponses for BytesMut {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl AsResponses for Cow<'static, [u8]> {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl<T: Serialize + JsonSchema> AsResponses for Json<T> {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        ok_json::<T>(gen)
    }
}
