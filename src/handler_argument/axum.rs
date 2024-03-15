use std::any::type_name;

use axum::body::Bytes;
use axum::extract::{Path, Query, RawForm};
use axum::{Form, Json};
use log::warn;
use openapiv3::{Parameter, ParameterData, ParameterSchemaOrContent, RequestBody};
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, ObjectValidation, SingleOrVec};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

use crate::handler_argument::{
    simple_request_body, HandlerArgument, ShouldBeHandlerArgument, SimpleRequestBody,
};
use crate::internals::convert_schema;

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
        let schema = convert_schema(gen.subschema_for::<T>());
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
        let schema = convert_schema(gen.subschema_for::<T>());
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
        let schema = T::json_schema(gen).into_object();

        match schema.instance_type {
            Some(SingleOrVec::Single(instance_type)) if *instance_type == InstanceType::Object => {}
            _ => {
                warn!("Unsupported handler argument: {}", type_name::<Self>());
                return Vec::new();
            }
        }
        let ObjectValidation {
            required,
            properties,
            .. // TODO check other fields for relevance
        } = *schema.object.unwrap_or_default();

        properties
            .into_iter()
            .map(|(name, schema)| Parameter::Path {
                parameter_data: ParameterData {
                    required: required.contains(&name),
                    name,
                    description: None,
                    deprecated: None,
                    format: ParameterSchemaOrContent::Schema(convert_schema(schema)),
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
        let schema = T::json_schema(gen).into_object();

        match schema.instance_type {
            Some(SingleOrVec::Single(instance_type)) if *instance_type == InstanceType::Object => {}
            _ => {
                warn!("Unsupported handler argument: {}", type_name::<Self>());
                return Vec::new();
            }
        }
        let ObjectValidation {
            required,
            properties,
            .. // TODO check other fields for relevance
        } = *schema.object.unwrap_or_default();

        properties
            .into_iter()
            .map(|(name, schema)| Parameter::Query {
                parameter_data: ParameterData {
                    required: required.contains(&name),
                    name,
                    description: None,
                    deprecated: None,
                    format: ParameterSchemaOrContent::Schema(convert_schema(schema)),
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
