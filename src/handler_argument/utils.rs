use openapiv3::RequestBody;
use schemars::gen::SchemaGenerator;
use serde::de::DeserializeOwned;

use crate::handler_argument::{
    simple_request_body, HandlerArgument, ShouldBeHandlerArgument, SimpleRequestBody,
};
use crate::utils::SchemalessJson;

impl<T: DeserializeOwned> ShouldBeHandlerArgument for SchemalessJson<T> {}
impl<T: DeserializeOwned> HandlerArgument for SchemalessJson<T> {
    fn request_body(_: &mut SchemaGenerator) -> Option<RequestBody> {
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_JSON,
            schema: None,
        }))
    }
}
