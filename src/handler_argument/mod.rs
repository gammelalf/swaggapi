#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "axum")]
mod axum;

use indexmap::IndexMap;
use mime::Mime;
use openapiv3::{MediaType, Parameter, ReferenceOr, RequestBody, Schema};
use schemars::gen::SchemaGenerator;

pub trait ShouldBeHandlerArgument {}

/// A type used as argument to a handler which can be described
/// by a [request body object](https://spec.openapis.org/oas/v3.0.3#request-body-object)
/// or a [parameter object](https://spec.openapis.org/oas/v3.0.3#parameter-object)
///
/// This type should be implemented by everything which implements
/// [`FromRequest`] when using [axum] or
/// [`FromRequest`] / [`FromRequestParts`] when using [actix]
pub trait HandlerArgument: ShouldBeHandlerArgument {
    /// Get the [request body object](https://spec.openapis.org/oas/v3.0.3#request-body-object) describing `Self`
    ///
    /// Should return `None` if `Self` doesn't consume the request body
    fn request_body(_gen: &mut SchemaGenerator) -> Option<RequestBody> {
        None
    }

    /// Get the [parameter objects](https://spec.openapis.org/oas/v3.0.3#parameter-object) describing `Self`
    ///
    /// Should return an empty `Vec` if `Self` doesn't parse any parameters
    fn parameters(_gen: &mut SchemaGenerator) -> Vec<Parameter> {
        Vec::new()
    }
}

/// Helper function for building a simple [`RequestBody`]
pub fn simple_request_body(request_body: SimpleRequestBody) -> RequestBody {
    RequestBody {
        content: IndexMap::<_, _>::from_iter([(
            request_body.mime_type.to_string(),
            MediaType {
                schema: request_body.schema,
                ..Default::default()
            },
        )]),
        required: true,
        ..Default::default()
    }
}

/// Describes the response for a specific status code
pub struct SimpleRequestBody {
    /// The request body's mime type
    pub mime_type: Mime,

    /// Optional schema
    pub schema: Option<ReferenceOr<Schema>>,
}

#[doc(hidden)]
pub mod macro_helper {
    use std::marker::PhantomData;
    use std::ops::Deref;

    use openapiv3::{Parameter, RequestBody};
    use schemars::gen::SchemaGenerator;

    use super::{HandlerArgument, ShouldBeHandlerArgument};

    pub fn add_handler_argumment<T: HandlerArgument>(
        _: Option<T>,
        gen: &mut SchemaGenerator,
        parameters: &mut Vec<Parameter>,
        request_body: &mut Vec<RequestBody>,
    ) {
        parameters.extend(T::parameters(gen));
        request_body.extend(T::request_body(gen));
    }

    impl<T> TraitProbe<T>
    where
        T: ShouldBeHandlerArgument,
    {
        pub fn should_be_handler_argument(&self) -> bool {
            true
        }

        pub fn get_handler_argument(&self) -> Option<T> {
            None
        }
    }

    impl<T> TraitProbe<T>
    where
        T: HandlerArgument,
    {
        pub fn is_handler_argument(&self) -> bool {
            true
        }
    }

    impl Else {
        pub fn should_be_handler_argument(&self) -> bool {
            false
        }

        pub fn get_handler_argument(&self) -> Option<NotAnArgument> {
            None
        }

        pub fn is_handler_argument(&self) -> bool {
            false
        }
    }

    pub struct TraitProbe<T>(PhantomData<T>);

    impl<T> TraitProbe<T> {
        pub const fn new() -> Self {
            Self(PhantomData)
        }
    }

    pub struct Else;

    impl<T> Deref for TraitProbe<T> {
        type Target = Else;

        fn deref(&self) -> &Self::Target {
            static ELSE: Else = Else;
            &ELSE
        }
    }

    pub struct NotAnArgument;

    impl ShouldBeHandlerArgument for NotAnArgument {}

    impl HandlerArgument for NotAnArgument {}
}
