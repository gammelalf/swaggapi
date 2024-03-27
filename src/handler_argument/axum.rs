use std::any::type_name;

use axum::body::Bytes;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::RawForm;
use axum::Form;
use axum::Json;
use log::warn;
use openapiv3::Parameter;
use openapiv3::ParameterData;
use openapiv3::ParameterSchemaOrContent;
use openapiv3::RequestBody;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

use crate::handler_argument::simple_request_body;
use crate::handler_argument::HandlerArgument;
use crate::handler_argument::ShouldBeHandlerArgument;
use crate::handler_argument::SimpleRequestBody;
use crate::internals::SchemaGenerator;

impl ShouldBeHandlerArgument for String {}
impl HandlerArgument for String {
    fn request_body(_gen: &mut SchemaGenerator) -> Option<RequestBody> {
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::TEXT_PLAIN_UTF_8,
            schema: None,
        }))
    }
}

impl ShouldBeHandlerArgument for Bytes {}
impl HandlerArgument for Bytes {
    fn request_body(_gen: &mut SchemaGenerator) -> Option<RequestBody> {
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_OCTET_STREAM,
            schema: None,
        }))
    }
}

impl<T> ShouldBeHandlerArgument for Json<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for Json<T> {
    fn request_body(gen: &mut SchemaGenerator) -> Option<RequestBody> {
        let schema = gen.generate::<T>();
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_JSON,
            schema: Some(schema),
        }))
    }
}

impl<T> ShouldBeHandlerArgument for Form<T> {}
/*
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for Form<T> {
    fn request_body(gen: &mut SchemaGenerator) -> Option<RequestBody> {
        let schema = convert_schema(gen.generate::<T>());
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_WWW_FORM_URLENCODED,
            schema: Some(schema),
        }))
    }
}
*/

impl ShouldBeHandlerArgument for RawForm {}
/*
impl HandlerArgument for RawForm {
    fn request_body(_gen: &mut SchemaGenerator) -> Option<RequestBody> {
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_WWW_FORM_URLENCODED,
            schema: None,
        }))
    }
}
*/

#[cfg(feature = "dep:axum/multipart")]
const _: () = {
    use axum::extract::Multipart;
    impl ShouldBeHandlerArgument for Multipart {}
    impl HandlerArgument for Multipart {
        fn request_body(_gen: &mut SchemaGenerator) -> Option<RequestBody> {
            Some(simple_request_body(SimpleRequestBody {
                mime_type: mime::MULTIPART_FORM_DATA,
                schema: None,
            }))
        }
    }
};

impl<T> ShouldBeHandlerArgument for Path<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for Path<T> {
    fn parameters(gen: &mut SchemaGenerator) -> Vec<Parameter> {
        let Some((obj, _)) = gen.generate_object::<T>() else {
            warn!("Unsupported handler argument: {}", type_name::<Self>());
            return Vec::new();
        };

        obj.properties
            .into_iter()
            .map(|(name, schema)| Parameter::Path {
                parameter_data: ParameterData {
                    required: obj.required.contains(&name),
                    name,
                    description: None,
                    deprecated: None,
                    format: ParameterSchemaOrContent::Schema(schema.unbox()),
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                style: Default::default(),
            })
            .collect()
    }
}

impl<T> ShouldBeHandlerArgument for Query<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for Query<T> {
    fn parameters(gen: &mut SchemaGenerator) -> Vec<Parameter> {
        let Some((obj, _)) = gen.generate_object::<T>() else {
            warn!("Unsupported handler argument: {}", type_name::<Self>());
            return Vec::new();
        };

        obj.properties
            .into_iter()
            .map(|(name, schema)| Parameter::Query {
                parameter_data: ParameterData {
                    required: obj.required.contains(&name),
                    name,
                    description: None,
                    deprecated: None,
                    format: ParameterSchemaOrContent::Schema(schema.unbox()),
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                allow_reserved: false,
                style: Default::default(),
                allow_empty_value: None,
            })
            .collect()
    }
}
