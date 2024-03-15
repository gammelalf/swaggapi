use openapiv3::{Responses, StatusCode};
use schemars::gen::SchemaGenerator;
use serde::Serialize;

use crate::as_responses::{simple_responses, AsResponses, SimpleResponse};
use crate::utils::SchemalessJson;

impl<T: Serialize> AsResponses for SchemalessJson<T> {
    fn responses(_: &mut SchemaGenerator) -> Responses {
        simple_responses([
            SimpleResponse {
                status_code: StatusCode::Code(200),
                mime_type: mime::APPLICATION_JSON,
                description: "Some json data".to_string(),
                media_type: None,
            },
            // TODO add error
        ])
    }
}
