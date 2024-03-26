use std::any::type_name;

use actix_web::web;
use log::warn;
use openapiv3::Parameter;
use openapiv3::ParameterData;
use openapiv3::ParameterSchemaOrContent;
use openapiv3::RequestBody;
use schemars::gen::SchemaGenerator;
use schemars::schema::InstanceType;
use schemars::schema::ObjectValidation;
use schemars::schema::SingleOrVec;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

use crate::handler_argument::simple_request_body;
use crate::handler_argument::HandlerArgument;
use crate::handler_argument::ShouldBeHandlerArgument;
use crate::handler_argument::SimpleRequestBody;
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

impl ShouldBeHandlerArgument for web::Bytes {}
impl HandlerArgument for web::Bytes {
    fn request_body(_gen: &mut SchemaGenerator) -> Option<RequestBody> {
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_OCTET_STREAM,
            schema: None,
        }))
    }
}

impl<T> ShouldBeHandlerArgument for web::Json<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for web::Json<T> {
    fn request_body(gen: &mut SchemaGenerator) -> Option<RequestBody> {
        let schema = convert_schema(gen.subschema_for::<T>());
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_JSON,
            schema: Some(schema),
        }))
    }
}

impl<T> ShouldBeHandlerArgument for web::Form<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for web::Form<T> {
    fn request_body(gen: &mut SchemaGenerator) -> Option<RequestBody> {
        let schema = convert_schema(gen.subschema_for::<T>());
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_WWW_FORM_URLENCODED,
            schema: Some(schema),
        }))
    }
}

impl<T> ShouldBeHandlerArgument for web::Path<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for web::Path<T> {
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

impl<T> ShouldBeHandlerArgument for web::Query<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for web::Query<T> {
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
