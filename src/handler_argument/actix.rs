use std::any::type_name;

use actix_web::web;
use log::{debug, warn};
use openapiv3::ParameterSchemaOrContent;
use openapiv3::RequestBody;
use openapiv3::{Parameter, ReferenceOr, Schema, SchemaKind};
use openapiv3::{ParameterData, Type};
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
        let schema = gen.generate::<T>();
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_JSON,
            schema: Some(schema),
        }))
    }
}

impl<T> ShouldBeHandlerArgument for web::Form<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for web::Form<T> {
    fn request_body(gen: &mut SchemaGenerator) -> Option<RequestBody> {
        let schema = gen.generate::<T>();
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_WWW_FORM_URLENCODED,
            schema: Some(schema),
        }))
    }
}

impl<T> ShouldBeHandlerArgument for web::Path<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for web::Path<T> {
    fn parameters(gen: &mut SchemaGenerator, path: &[&str]) -> Vec<Parameter> {
        let Ok(schema) = gen.generate_refless::<T>() else {
            warn!("Unsupported handler argument: {}", type_name::<Self>());
            debug!("generate_refless::<{}>() == Err(_)", type_name::<T>());
            return Vec::new();
        };

        match schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => obj
                .properties
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
                .collect(),
            _ if path.len() == 1 => {
                vec![Parameter::Path {
                    parameter_data: ParameterData {
                        name: path[0].to_string(),
                        description: None,
                        required: !schema.schema_data.nullable,
                        deprecated: None,
                        format: ParameterSchemaOrContent::Schema(ReferenceOr::Item(schema)),
                        example: None,
                        examples: Default::default(),
                        explode: None,
                        extensions: Default::default(),
                    },
                    style: Default::default(),
                }]
            }
            _ => {
                warn!("Unsupported handler argument: {}", type_name::<Self>());
                debug!(
                    "generate_refless::<{}>() == Ok({schema:#?})",
                    type_name::<T>()
                );
                Vec::new()
            }
        }
    }
}

impl<T> ShouldBeHandlerArgument for web::Query<T> {}
impl<T: DeserializeOwned + JsonSchema> HandlerArgument for web::Query<T> {
    fn parameters(gen: &mut SchemaGenerator, _path: &[&str]) -> Vec<Parameter> {
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

impl ShouldBeHandlerArgument for web::Payload {}
impl HandlerArgument for web::Payload {
    fn request_body(_gen: &mut SchemaGenerator) -> Option<RequestBody> {
        Some(simple_request_body(SimpleRequestBody {
            mime_type: mime::APPLICATION_OCTET_STREAM,
            schema: Some(ReferenceOr::Item(Schema {
                schema_data: Default::default(),
                schema_kind: SchemaKind::Any(Default::default()),
            })),
        }))
    }
}
