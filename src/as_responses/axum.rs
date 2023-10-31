use openapiv3::{Responses, StatusCode};
use schemars::gen::SchemaGenerator;

use crate::as_responses::{simple_responses, AsResponses, SimpleResponse};

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
