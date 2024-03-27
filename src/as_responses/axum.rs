use std::borrow::Cow;

use axum::Json;
use bytes::buf::Chain;
use bytes::Buf;
use bytes::Bytes;
use bytes::BytesMut;
use openapiv3::Responses;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use serde::Serialize;

use crate::as_responses::ok_binary;
use crate::as_responses::ok_empty;
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

impl<const N: usize> AsResponses for &'static [u8; N] {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}

impl<const N: usize> AsResponses for [u8; N] {
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

impl AsResponses for () {
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_empty()
    }
}

impl<T, E> AsResponses for Result<T, E>
where
    T: AsResponses,
    E: AsResponses,
{
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        let mut res = E::responses(gen);
        let ok_res = T::responses(gen);

        // As we want to preserve in almost any cases the Ok branch of the result, we're extending
        // the IndexMaps of the error-branch with those of the ok-branch
        res.responses.extend(ok_res.responses);
        res.extensions.extend(ok_res.extensions);
        if ok_res.default.is_some() {
            res.default = ok_res.default;
        }

        res
    }
}

impl<T, U> AsResponses for Chain<T, U>
where
    T: Buf + Unpin + Send + 'static,
    U: Buf + Unpin + Send + 'static,
{
    fn responses(_gen: &mut SchemaGenerator) -> Responses {
        ok_binary()
    }
}
