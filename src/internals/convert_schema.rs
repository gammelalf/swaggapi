use std::mem;

use ::openapiv3 as openapi;
use ::schemars::schema as schemars;
use log::warn;
use openapiv3::AnySchema;

/// Convert a [schemars](::schemars)'s [`Schema`](::schemars::schema::Schema) to a [openapiv3]'s [`Schema`](openapiv3::Schema)
pub fn convert_schema(schema: schemars::Schema) -> openapi::ReferenceOr<openapi::Schema> {
    match schema {
        schemars::Schema::Object(object) => convert_schema_object(object),
        schemars::Schema::Bool(boolean) => {
            // Empty schema which would match everything
            let mut schema = openapi::ReferenceOr::Item(openapi::Schema {
                schema_data: Default::default(),
                schema_kind: openapi::SchemaKind::Any(openapi::AnySchema::default()),
            });

            if !boolean {
                // Wrap with not to match nothing
                schema = openapi::ReferenceOr::Item(openapi::Schema {
                    schema_data: Default::default(),
                    schema_kind: openapi::SchemaKind::Not {
                        not: Box::new(schema),
                    },
                });
            }

            schema
        }
    }
}

fn convert_schema_object(schema: schemars::SchemaObject) -> openapi::ReferenceOr<openapi::Schema> {
    let schemars::SchemaObject {
        metadata,
        instance_type,
        format,
        enum_values,
        const_value,
        subschemas,
        number,
        string,
        array,
        object,
        reference,
        extensions,
    } = schema;
    if let Some(reference) = reference {
        return openapi::ReferenceOr::Reference { reference };
    }

    if const_value.is_some() {
        warn!("Can't convert `const_value`");
    }

    let mut schema_data = metadata
        .map(|metadata| {
            let schemars::Metadata {
                id: _, // TODO maybe external_docs
                title,
                description,
                default,
                deprecated,
                read_only,
                write_only,
                examples,
            } = *metadata;
            if examples.len() > 1 {
                warn!("Only the first of the schema's examples will be used");
            }
            openapi::SchemaData {
                nullable: false,
                read_only,
                write_only,
                deprecated,
                example: examples.into_iter().next(),
                title,
                description,
                default,
                extensions: extensions.into_iter().collect(),
                // always empty because unique to openapi
                external_docs: None,
                discriminator: None,
            }
        })
        .unwrap_or_default();

    let mut any_schema = AnySchema::default();
    if let Some(subschemas) = subschemas {
        let schemars::SubschemaValidation {
            all_of,
            any_of,
            one_of,
            not,
            if_schema,
            then_schema,
            else_schema,
        } = *subschemas;
        if let Some(all_of) = all_of {
            any_schema.all_of = all_of.into_iter().map(convert_schema).collect();
        }
        if let Some(any_of) = any_of {
            any_schema.any_of = any_of.into_iter().map(convert_schema).collect();
        }
        if let Some(one_of) = one_of {
            any_schema.one_of = one_of.into_iter().map(convert_schema).collect();
        }
        if let Some(not) = not {
            any_schema.not = Some(Box::new(convert_schema(*not)));
        }
        if if_schema.is_some() {
            warn!("Can't convert `if_schema`");
        }
        if then_schema.is_some() {
            warn!("Can't convert `then_schema`");
        }
        if else_schema.is_some() {
            warn!("Can't convert `else_schema`");
        }
    }

    let mut types = Vec::new();
    if let Some(instance_type) = instance_type {
        let InstanceTypes {
            many,
            is_null,
            is_boolean,
            is_object,
            is_array,
            is_number,
            is_string,
            is_integer,
        } = InstanceTypes::parse(instance_type);

        let mut enum_values = enum_values.unwrap_or_default();
        let mut clone_any_schema = || {
            if !many {
                mem::take(&mut any_schema)
            } else {
                any_schema.clone()
            }
        };
        if is_boolean {
            let mut any_schema = clone_any_schema();
            convert_boolean_type(&mut any_schema, &mut enum_values);
            types.push(any_schema);
        }
        if is_object {
            let mut any_schema = clone_any_schema();
            convert_object_type(&mut any_schema, object, &mut enum_values);
            types.push(any_schema);
        }
        if is_array {
            let mut any_schema = clone_any_schema();
            types.extend(convert_array_type(&mut any_schema, array, &mut enum_values));
        }
        if is_number {
            let mut any_schema = clone_any_schema();
            convert_number_type(
                &mut any_schema,
                number.clone(),
                format.as_deref(),
                &mut enum_values,
            );
            types.push(any_schema);
        }
        if is_string {
            let mut any_schema = clone_any_schema();
            convert_string_type(&mut any_schema, string, format.as_deref(), &mut enum_values);
            types.push(any_schema);
        }
        if is_integer {
            let mut any_schema = clone_any_schema();
            convert_integer_type(
                &mut any_schema,
                number.clone(),
                format.as_deref(),
                &mut enum_values,
            );
            types.push(any_schema);
        }

        if matches!(types.len(), 0 | 1) {
            schema_data.nullable = is_null;
        }
    }

    openapi::ReferenceOr::Item(match types.len() {
        0 => openapi::Schema {
            schema_data,
            schema_kind: openapi::SchemaKind::Any(any_schema),
        },
        1 => openapi::Schema {
            schema_data,
            schema_kind: openapi::SchemaKind::Any(types.pop().expect("Length should be one")),
        },
        _ => openapi::Schema {
            schema_data,
            schema_kind: openapi::SchemaKind::OneOf {
                one_of: types
                    .into_iter()
                    .map(|any_schema| {
                        openapi::ReferenceOr::Item(openapi::Schema {
                            schema_data: openapi::SchemaData::default(),
                            schema_kind: openapi::SchemaKind::Any(any_schema),
                        })
                    })
                    .collect(),
            },
        },
    })
}

#[derive(Default)]
struct InstanceTypes {
    many: bool,
    is_null: bool,
    is_boolean: bool,
    is_object: bool,
    is_array: bool,
    is_number: bool,
    is_string: bool,
    is_integer: bool,
}
impl InstanceTypes {
    pub fn parse(input: schemars::SingleOrVec<schemars::InstanceType>) -> Self {
        let mut output = Self::default();
        let mut set = |instance_type: schemars::InstanceType| match instance_type {
            schemars::InstanceType::Null => {
                output.is_null = true;
                if output.is_null {
                    warn!("Instance type `Null` is specified multiple times");
                }
            }
            schemars::InstanceType::Boolean => {
                output.is_boolean = true;
                if output.is_null {
                    warn!("Instance type `Boolean` is specified multiple times");
                }
            }
            schemars::InstanceType::Object => {
                output.is_object = true;
                if output.is_null {
                    warn!("Instance type `Object` is specified multiple times");
                }
            }
            schemars::InstanceType::Array => {
                output.is_array = true;
                if output.is_null {
                    warn!("Instance type `Array` is specified multiple times");
                }
            }
            schemars::InstanceType::Number => {
                output.is_number = true;
                if output.is_null {
                    warn!("Instance type `Number` is specified multiple times");
                }
            }
            schemars::InstanceType::String => {
                output.is_string = true;
                if output.is_null {
                    warn!("Instance type `String` is specified multiple times");
                }
            }
            schemars::InstanceType::Integer => {
                output.is_integer = true;
                if output.is_null {
                    warn!("Instance type `Integer` is specified multiple times");
                }
            }
        };
        match input {
            schemars::SingleOrVec::Single(instance_type) => {
                output.many = false;
                set(*instance_type);
            }
            schemars::SingleOrVec::Vec(instance_types) => {
                output.many = instance_types.len() > 1;
                instance_types.into_iter().for_each(set)
            }
        }
        output
    }
}

fn convert_object_type(
    any_schema: &mut AnySchema,
    input: Option<Box<schemars::ObjectValidation>>,
    enums: &mut [serde_json::Value],
) {
    let schemars::ObjectValidation {
        max_properties,
        min_properties,
        required,
        properties,
        pattern_properties,
        additional_properties,
        property_names,
    } = *input.unwrap_or_default();

    if !pattern_properties.is_empty() {
        warn!("Can't convert `pattern_properties`");
    }
    if property_names.is_some() {
        warn!("Can't convert `property_names`");
    }
    if enums.iter().any(|value| value.is_object()) {
        warn!("Can't convert `enum` for type `object`");
    }

    any_schema.typ = Some("object".to_string());
    any_schema.properties = properties
        .into_iter()
        .map(|(key, schema)| (key, box_reference_or(convert_schema(schema))))
        .collect();
    any_schema.required = required.into_iter().collect();
    any_schema.additional_properties =
        additional_properties.map(|additional_properties| match *additional_properties {
            schemars::Schema::Bool(boolean) => openapi::AdditionalProperties::Any(boolean),
            schemars::Schema::Object(object) => {
                openapi::AdditionalProperties::Schema(Box::new(convert_schema_object(object)))
            }
        });
    any_schema.min_properties = min_properties.map(convert_u32);
    any_schema.max_properties = max_properties.map(convert_u32);
}

fn convert_array_type(
    any_schema: &mut AnySchema,
    input: Option<Box<schemars::ArrayValidation>>,
    enums: &mut [serde_json::Value],
) -> Vec<AnySchema> {
    let schemars::ArrayValidation {
        items,
        additional_items,
        max_items,
        min_items,
        unique_items,
        contains,
    } = *input.unwrap_or_default();

    if additional_items.is_some() {
        warn!("Can't convert `additional_items`");
    }

    if contains.is_some() {
        warn!("Can't convert `contains`");
    }
    if enums.iter().any(|value| value.is_array()) {
        warn!("Can't convert `enum` for type `array`");
    }

    any_schema.typ = Some("array".to_string());
    any_schema.max_items = max_items.map(convert_u32);
    any_schema.min_items = min_items.map(convert_u32);
    any_schema.unique_items = unique_items;

    let mut arrays = Vec::new();
    let mut push_array = |item: schemars::Schema| {
        let mut any_schema = any_schema.clone();
        any_schema.items = Some(box_reference_or(convert_schema(item)));
        arrays.push(any_schema)
    };
    match items {
        None => push_array(schemars::Schema::Bool(true)),
        Some(schemars::SingleOrVec::Single(item)) => push_array(*item),
        Some(schemars::SingleOrVec::Vec(items)) => items.into_iter().for_each(push_array),
    }
    arrays
}

fn convert_number_type(
    any_schema: &mut AnySchema,
    input: Option<Box<schemars::NumberValidation>>,
    format: Option<&str>,
    enums: &mut [serde_json::Value],
) {
    let schemars::NumberValidation {
        multiple_of,
        mut maximum,
        exclusive_maximum,
        mut minimum,
        exclusive_minimum,
    } = *input.unwrap_or_default();

    if let Some(exclusive_maximum) = exclusive_maximum {
        if maximum.is_some() {
            warn!("`maximum` and `exclusive_maximum` are both set");
        }
        maximum = Some(exclusive_maximum);
    }
    if let Some(exclusive_minimum) = exclusive_minimum {
        if minimum.is_some() {
            warn!("`minimum` and `exclusive_minimum` are both set");
        }
        minimum = Some(exclusive_minimum);
    }

    let mut enumeration = Vec::new();
    for value in enums {
        match value {
            serde_json::Value::Null => enumeration.push(serde_json::Value::Null),
            serde_json::Value::Number(number) => {
                enumeration.push(serde_json::Value::Number(number.clone()));
            }
            _ => {}
        }
    }

    any_schema.typ = Some("number".to_string());
    any_schema.format = format.map(str::to_string);
    any_schema.multiple_of = multiple_of;
    any_schema.exclusive_minimum = minimum.is_some().then_some(exclusive_minimum.is_some());
    any_schema.exclusive_maximum = maximum.is_some().then_some(exclusive_maximum.is_some());
    any_schema.minimum = minimum;
    any_schema.maximum = maximum;
    any_schema.enumeration = enumeration;
}

fn convert_string_type(
    any_schema: &mut AnySchema,
    input: Option<Box<schemars::StringValidation>>,
    format: Option<&str>,
    enums: &mut [serde_json::Value],
) {
    let schemars::StringValidation {
        max_length,
        min_length,
        pattern,
    } = *input.unwrap_or_default();

    let mut enumeration = Vec::new();
    for value in enums {
        match value {
            serde_json::Value::Null => enumeration.push(serde_json::Value::Null),
            serde_json::Value::String(string) => {
                enumeration.push(serde_json::Value::String(mem::take(string)))
            }

            _ => {}
        }
    }

    any_schema.typ = Some("string".to_string());
    any_schema.format = format.map(str::to_string);
    any_schema.pattern = pattern;
    any_schema.enumeration = enumeration;
    any_schema.min_length = min_length.map(convert_u32);
    any_schema.max_length = max_length.map(convert_u32);
}

fn convert_integer_type(
    any_schema: &mut AnySchema,
    input: Option<Box<schemars::NumberValidation>>,
    format: Option<&str>,
    enums: &mut [serde_json::Value],
) {
    convert_number_type(any_schema, input, format, enums);
    any_schema.typ = Some("integer".to_string());
    any_schema.enumeration.retain(|value| match value {
        serde_json::Value::Null => true,
        serde_json::Value::Number(number) => number.is_i64() || number.is_u64(),
        _ => false,
    });
}

fn convert_boolean_type(any_schema: &mut AnySchema, enums: &mut [serde_json::Value]) {
    let mut enumeration = Vec::new();
    for value in enums {
        match value {
            serde_json::Value::Null => enumeration.push(serde_json::Value::Null),
            serde_json::Value::Bool(bool) => {
                enumeration.push(serde_json::Value::Bool(*bool));
            }
            _ => {}
        }
    }
    any_schema.typ = Some("boolean".to_string());
    any_schema.enumeration = enumeration;
}

fn convert_u32(input: u32) -> usize {
    input.try_into().unwrap_or_else(|_| {
        warn!("Couldn't convert u32 lossless to usize");
        usize::MAX
    })
}

fn box_reference_or<T>(input: openapi::ReferenceOr<T>) -> openapi::ReferenceOr<Box<T>> {
    match input {
        openapi::ReferenceOr::Item(item) => openapi::ReferenceOr::Item(Box::new(item)),
        openapi::ReferenceOr::Reference { reference } => {
            openapi::ReferenceOr::Reference { reference }
        }
    }
}
